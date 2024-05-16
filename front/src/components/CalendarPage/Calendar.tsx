import { FC, RefObject, useCallback, useEffect, useMemo } from "react";
import FullCalendar from "@fullcalendar/react";
import dayGridPlugin from "@fullcalendar/daygrid";
import timeGridPlugin from "@fullcalendar/timegrid";
import interactionPlugin from "@fullcalendar/interaction";
import resourceTimelinePlugin from "@fullcalendar/resource-timeline";
import { SolutionInfo } from "../../types/api";
import { DatesSetArg, EventSourceInput } from "@fullcalendar/core/index.js";
import { CalendarDisplay } from "../../types/calendar";
import { getColorBySessionId } from "../../utils/colors";
import {
  FullCalendarViewName,
  getFullCalendarViewName,
} from "../../utils/dates";

interface Props {
  onDatesSet: (activeStart: Date, activeEnd: Date) => void;
  fullCalendarRef: RefObject<FullCalendar>;
  instance: SolutionInfo;
  initialFrom: Date;
  display: CalendarDisplay;
}

const Calendar: FC<Props> = ({
  fullCalendarRef,
  instance,
  initialFrom,
  onDatesSet,
  display,
}) => {
  const view: FullCalendarViewName = useMemo(
    () => getFullCalendarViewName(display.viewMode, display.viewLevel),
    [display.viewLevel, display.viewMode]
  );

  const colorDict = useMemo(
    () => getColorBySessionId(instance.sessions, display.colorMode),
    [display.colorMode, instance.sessions]
  );

  const events = useMemo(
    (): EventSourceInput =>
      instance.sessions.map((session) => ({
        start: session.from,
        end: session.to,
        id: session.id,
        title: session.course.id,
        color: colorDict[session.id],
      })),
    [colorDict, instance.sessions]
  );

  const handleDatesSet = useCallback(
    (arg: DatesSetArg) => {
      onDatesSet(arg.view.activeStart, arg.view.activeEnd);
    },
    [onDatesSet]
  );

  useEffect(() => {
    const api = fullCalendarRef?.current?.getApi();
    if (api === undefined) {
      return;
    }
    api.changeView(view);
  }, [fullCalendarRef, view]);

  return (
    <FullCalendar
      schedulerLicenseKey={"GPL-My-Project-Is-Open-Source"}
      plugins={[
        dayGridPlugin,
        timeGridPlugin,
        interactionPlugin,
        resourceTimelinePlugin,
      ]}
      buttonText={{
        today: "Aujourd'hui",
        month: "Mois",
        week: "Semaine",
        day: "Jour",
      }}
      datesSet={handleDatesSet}
      headerToolbar={false}
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
      initialDate={initialFrom}
    />
  );
};

export default Calendar;
