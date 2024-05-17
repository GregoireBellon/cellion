import {
  FC,
  MutableRefObject,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from "react";
import FullCalendar from "@fullcalendar/react";
import dayGridPlugin from "@fullcalendar/daygrid";
import timeGridPlugin from "@fullcalendar/timegrid";
import interactionPlugin from "@fullcalendar/interaction";
import resourceTimelinePlugin from "@fullcalendar/resource-timeline";
import { ResourceSourceInput } from "@fullcalendar/resource";
import {
  DatesSetArg,
  EventClickArg,
  EventSourceInput,
} from "@fullcalendar/core/index.js";
import { CalendarDisplay } from "../../types/calendar";
import { getColorBySessionId } from "../../utils/colors";
import {
  FullCalendarViewName,
  getFullCalendarViewName,
} from "../../utils/dates";
import { ShortSessionInfo } from "../../types/core";
import { LinearProgress, Popover } from "@mui/material";

interface Props {
  onDatesSet: (activeStart: Date, activeEnd: Date) => void;
  fullCalendarRef: MutableRefObject<FullCalendar | null>;
  sessions: ShortSessionInfo[];
  initialFrom: Date;
  display: CalendarDisplay;
  loading: boolean;
}

const Calendar: FC<Props> = ({
  fullCalendarRef,
  sessions,
  initialFrom,
  onDatesSet,
  display,
  loading,
}) => {
  const [poperAnchorEl, setPoperAnchorEl] = useState<HTMLElement | null>(null);

  const popperOpen = useMemo(() => Boolean(poperAnchorEl), [poperAnchorEl]);

  const view: FullCalendarViewName = useMemo(
    () => getFullCalendarViewName(display.viewMode, display.viewLevel),
    [display.viewLevel, display.viewMode]
  );

  const colorDict = useMemo(
    () => getColorBySessionId(sessions, display.colorMode),
    [display.colorMode, sessions]
  );

  const events = useMemo(
    (): EventSourceInput =>
      sessions.map((session) => ({
        start: session.from,
        end: session.to,
        id: session.id,
        title: session.course.id,
        color: colorDict[session.id],
        interactive: true,
        resourceId: session.rooms[0].id,
      })),
    [colorDict, sessions]
  );

  // actually less efficient than using values returned by solutions/:solutionId/filters
  // but faster because only displays used rooms
  const resources = useMemo(
    (): ResourceSourceInput =>
      sessions.map((session) => ({ id: session.rooms[0].id })),
    [sessions]
  );

  const handleDatesSet = useCallback(
    (arg: DatesSetArg) => {
      onDatesSet(arg.view.activeStart, arg.view.activeEnd);
    },
    [onDatesSet]
  );

  const handleEventClick = useCallback((arg: EventClickArg) => {
    setPoperAnchorEl(arg.el);
  }, []);

  const handlePopperClose = useCallback(() => {
    setPoperAnchorEl(null);
  }, []);

  useEffect(() => {
    const api = fullCalendarRef?.current?.getApi();
    if (api === undefined) {
      return;
    }
    api.changeView(view);
  }, [view, fullCalendarRef]);

  return (
    <>
      <LinearProgress sx={{ opacity: loading ? undefined : 0 }} />
      <Popover
        open={popperOpen}
        onClose={handlePopperClose}
        anchorEl={poperAnchorEl}
        anchorOrigin={{
          vertical: "bottom",
          horizontal: "left",
        }}
      >
        coucou
      </Popover>
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
        weekText="S"
        initialView="timeGridWeek"
        locale="fr"
        allDaySlot={false}
        slotMinTime={"07:00:00"}
        slotDuration={{ minutes: view.includes("resource") ? 60 : 15 }}
        firstDay={1}
        events={events}
        resources={resources}
        resourceAreaColumns={[
          {
            headerContent: "Salles",
            field: "id",
          },
        ]}
        height="auto"
        stickyHeaderDates
        ref={fullCalendarRef}
        initialDate={initialFrom}
        eventMaxStack={5}
        eventClick={handleEventClick}
      />
    </>
  );
};

export default Calendar;
