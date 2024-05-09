import { FC, RefObject, useMemo } from "react";
import FullCalendar from "@fullcalendar/react";
import dayGridPlugin from "@fullcalendar/daygrid";
import timeGridPlugin from "@fullcalendar/timegrid";
import interactionPlugin from "@fullcalendar/interaction";
import { CalendarInfo } from "../../types/api";
import { EventSourceInput } from "@fullcalendar/core/index.js";

interface Props {
  fullCalendarRef: RefObject<FullCalendar>;
  calendar: CalendarInfo;
  from: Date;
}

const Calendar: FC<Props> = ({ fullCalendarRef, calendar, from }) => {
  const events = useMemo(
    (): EventSourceInput =>
      calendar.sessions.map((session) => ({
        start: session.from,
        end: session.to,
        id: session.id,
        title: session.course.name,
      })),
    [calendar]
  );

  return (
    <FullCalendar
      plugins={[dayGridPlugin, timeGridPlugin, interactionPlugin]}
      headerToolbar={false}
      buttonText={{
        today: "Aujourd'hui",
        month: "Mois",
        week: "Semaine",
        day: "Jour",
      }}
      handleWindowResize={true}
      weekNumbers={true}
      weekText={"S"}
      initialView="timeGridWeek"
      locale="fr"
      allDaySlot={false}
      slotMinTime={"07:00:00"}
      slotDuration={"00:15:00"}
      firstDay={1}
      events={events}
      height="auto"
      stickyHeaderDates
      ref={fullCalendarRef}
      initialDate={from}
    />
  );
};

export default Calendar;
