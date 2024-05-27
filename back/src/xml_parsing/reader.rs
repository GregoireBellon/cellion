use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::{self, FromStr},
};

use log::error;
use quick_xml::{
    de::from_str,
    events::{self, Event},
    name::QName,
    DeError, Reader, Writer,
};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub enum EventHandlingError {
    DeserializationError(DeserializationError),
    UnsupportedEventType,
}

#[derive(Debug)]
pub enum XmlRoutingError<E> {
    HandlingError(E),
    RoutingError(quick_xml::Error),
}

#[derive(Debug)]
pub enum DeserializationError {
    Utf8Error(str::Utf8Error),
    DeError(DeError),
    QuickXmlError(quick_xml::Error),
}

pub type Router<'a, R, E, Context> = Vec<XmlRouting<'a, R, E, Context>>;

pub struct XmlRouting<'a, R: BufRead, E, Context> {
    pub route: Vec<&'static str>,
    pub handler: Box<dyn FnMut(Event, &mut XmlParser<R>, &mut Context) -> Result<(), E> + 'a>,
}

pub struct XmlParser<R: BufRead> {
    reader: Reader<R>,
    buffer: Vec<u8>,
    serialization_buffer: Vec<u8>,
}

impl<'a> XmlParser<BufReader<File>> {
    pub fn from_file(file_path: &Path) -> Result<Self, quick_xml::Error> {
        let reader = Reader::from_file(file_path)?;

        return Ok(Self::new(reader));
    }
}

impl<'a, R: BufRead> XmlParser<R> {
    pub fn new(reader: Reader<R>) -> Self {
        XmlParser {
            reader: reader,
            buffer: Vec::new(),
            serialization_buffer: Vec::new(),
        }
    }

    pub fn walk_buffer<E, Context>(
        &mut self,
        router: &mut Router<'a, R, E, Context>,
        context: &mut Context,
    ) -> Result<(), XmlRoutingError<E>> {
        let mut buffer = Vec::new();

        let mut current_route = Vec::new();

        loop {
            buffer.clear();
            let event_result = self.reader.read_event_into(&mut buffer);

            match event_result {
                Err(e) => return Err(XmlRoutingError::RoutingError(e)),
                Ok(event) => match event.as_ref() {
                    Event::Eof => break,

                    Event::Start(bs) => {
                        current_route.push(Self::qname_to_string(&bs.name()));
                        let consumed_event = self
                            .route_xml_event(event, router, &mut current_route, context)
                            .map_err(XmlRoutingError::HandlingError)?;

                        if consumed_event {
                            current_route.pop();
                        }
                    }

                    Event::End(_) => {
                        current_route.pop();
                    }

                    Event::Empty(e) => {
                        current_route.push(Self::qname_to_string(&e.name()));
                        self.route_xml_event(event, router, &mut current_route, context)
                            .map_err(XmlRoutingError::HandlingError)?;
                        current_route.pop();
                    }

                    _ => {}
                },
            };
        }
        return Result::Ok(());
    }

    fn qname_to_string(name: &QName) -> String {
        String::from_str(str::from_utf8(name.into_inner()).unwrap()).unwrap()
    }

    // match the route with the router, returns true if it consumed the event
    fn route_xml_event<E, Context>(
        &mut self,
        event: Event,
        router: &mut Router<'a, R, E, Context>,
        current_route: &mut Vec<String>,
        context: &mut Context,
    ) -> Result<bool, E> {
        // debug!("In {:?}", current_route);
        for routing in router {
            // debug!("comparing with {:?}", routing.route);
            if check_eq_vecs(&routing.route, current_route) {
                let ret = (&mut routing.handler)(event, self, context);
                return ret.map(|_| true);
            }
        }

        return Ok(false);
    }

    pub fn handle_event<T: DeserializeOwned, F, Ret>(
        &mut self,
        event: Event,
        mut f: F,
    ) -> Result<Ret, EventHandlingError>
    where
        F: FnMut(T) -> Ret,
    {
        match self.deserialize_event::<T>(event) {
            Ok(elem) => Ok(f(elem)),
            Err(e) => {
                error!(
                    "Error while trying to deserialize {}: {:?}",
                    std::any::type_name::<T>(),
                    e
                );
                Err(e)
            }
        }
    }
    pub fn deserialize_event<T: DeserializeOwned>(
        &mut self,
        event: Event,
    ) -> Result<T, EventHandlingError> {
        match event {
            Event::Empty(e) => self
                .deserialize_empty_element(e)
                .map_err(EventHandlingError::DeserializationError),
            Event::Start(e) => self
                .deserialize_element(e)
                .map_err(EventHandlingError::DeserializationError),
            _ => Err(EventHandlingError::UnsupportedEventType),
        }
    }

    /// Deserialize an xml element that has the form : ```<tag />```
    fn deserialize_empty_element<T: DeserializeOwned>(
        &mut self,
        element: events::BytesStart,
    ) -> Result<T, DeserializationError> {
        self.buffer.clear();

        let mut w = Writer::new(&mut self.buffer);
        w.write_event(Event::Empty(element))
            .map_err(DeserializationError::QuickXmlError)?;

        return deserialize_from_buffer(&mut self.buffer);
    }

    /// From https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/
    /// Deserialize an XML element that has the form: ```<tag> </tag>```
    fn deserialize_element<T: DeserializeOwned>(
        &mut self,
        start_tag: events::BytesStart,
    ) -> Result<T, DeserializationError> {
        let mut depth: u8 = 0;

        self.buffer.clear();

        let mut w = Writer::new(&mut self.buffer);

        let tag_name = start_tag.name();

        w.write_event(Event::Start(start_tag.clone()))
            .map_err(DeserializationError::QuickXmlError)?;

        loop {
            self.serialization_buffer.clear();
            let event = self
                .reader
                .read_event_into(&mut self.serialization_buffer)
                .map_err(DeserializationError::QuickXmlError)?;

            w.write_event(&event)
                .map_err(DeserializationError::QuickXmlError)?;

            match event {
                Event::Start(e) if e.name() == tag_name => depth += 1,
                Event::End(e) if e.name() == tag_name => {
                    if depth == 0 {
                        return deserialize_from_buffer(&mut self.buffer);
                    }
                    depth -= 1;
                }
                Event::Eof => {
                    return Err(DeserializationError::QuickXmlError(
                        quick_xml::Error::UnexpectedEof(format!(
                            "Unexpected end of file while reading {:?}",
                            tag_name
                        )),
                    ))
                }
                _ => {}
            }
        }
    }
}

/// Errors can be Utf8Error or DeError
fn deserialize_from_buffer<T: DeserializeOwned>(
    output_buffer: &mut Vec<u8>,
) -> Result<T, DeserializationError> {
    let str = std::str::from_utf8(&output_buffer).map_err(DeserializationError::Utf8Error)?;

    return from_str::<T>(str).map_err(DeserializationError::DeError);
}

fn check_eq_vecs<T: PartialEq<U>, U>(v1: &Vec<T>, v2: &Vec<U>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }

    let mut v1_iterator = v1.iter();
    let mut v2_iterator = v2.iter();

    let mut v1_next = v1_iterator.next();
    let mut v2_next = v2_iterator.next();

    while v1_next.is_some() && v2_next.is_some() {
        if v1_next.unwrap() != v2_next.unwrap() {
            return false;
        }

        v1_next = v1_iterator.next();
        v2_next = v2_iterator.next();
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::check_eq_vecs;

    #[test]
    fn should_be_eq_vectors() {
        let v1: Vec<&'static str> = vec!["a", "b", "c"];
        let v2 = vec![String::from("a"), String::from("b"), String::from("c")];

        assert!(check_eq_vecs(&v1, &v2));
    }
    #[test]
    fn should_not_be_eq_vectors() {
        let v1: Vec<&'static str> = vec!["a", "b", "d"];
        let v2 = vec![String::from("a"), String::from("b"), String::from("c")];

        assert!(false == check_eq_vecs(&v1, &v2));
    }
}
