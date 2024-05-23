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
  EventInput,
} from "@fullcalendar/core/index.js";
import { CalendarDisplaySettings } from "../../../types/calendar";
import { getColorBySessionId } from "../../../utils/colors/colors";
import {
  FullCalendarViewName,
  getFullCalendarViewName,
} from "../../../utils/dates";
import { ShortSessionInfo } from "../../../types/core";
import { Box, LinearProgress } from "@mui/material";
import EventDetailsDialog from "./EventDetailsDialog";
import chroma from "chroma-js";
import { useIsDarkMode } from "../../../utils/colors/useIsDarkMode";

interface Props {
  onDatesSet: (activeStart: Date, activeEnd: Date) => void;
  fullCalendarRef: MutableRefObject<FullCalendar | null>;
  sessions: ShortSessionInfo[];
  initialFrom?: Date;
  display: CalendarDisplaySettings;
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
  const isDarkMode = useIsDarkMode();

  const [poperAnchorEl, setPoperAnchorEl] = useState<HTMLElement | null>(null);
  const [selectedEvent, setSelectedEvent] = useState<{
    id: string;
    color: string;
  } | null>(null);

  const popperOpen = useMemo(() => Boolean(poperAnchorEl), [poperAnchorEl]);

  const view: FullCalendarViewName = useMemo(
    () => getFullCalendarViewName(display.viewMode, display.viewLevel),
    [display.viewLevel, display.viewMode]
  );

  const colorDict = useMemo(
    () => getColorBySessionId(sessions, display.colorMode, isDarkMode),
    [display.colorMode, isDarkMode, sessions]
  );

  const events = useMemo(
    (): EventInput[] =>
      sessions.map((session) => ({
        start: session.from,
        end: session.to,
        id: session.id,
        title: session.course.id,
        color: colorDict[session.id],
        borderColor: isDarkMode
          ? chroma(colorDict[session.id] ?? "black")
              .brighten(1)
              .css()
          : chroma(colorDict[session.id] ?? "black")
              .darken(1)
              .css(),
        interactive: true,
        resourceId: session.rooms[0].id,
      })),
    [colorDict, isDarkMode, sessions]
  );

  const selectedSession = useMemo(() => {
    if (selectedEvent === null) {
      return null;
    }
    const session = sessions.find((s) => s.id === selectedEvent.id);
    if (session === undefined) {
      return null;
    }
    return {
      ...session,
      color: selectedEvent.color,
    };
  }, [selectedEvent, sessions]);

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
    setSelectedEvent({ id: arg.event.id, color: arg.event.backgroundColor });
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
  }, [fullCalendarRef, view]);

  return (
    <>
      <LinearProgress sx={{ opacity: loading ? undefined : 0 }} />
      <EventDetailsDialog
        open={popperOpen}
        onClose={handlePopperClose}
        session={selectedSession}
      />
      <Box
        sx={{
          ["& > *"]: {
            overflow: "hidden",
          },

          ["& > .fc"]: {
            background: (theme) => theme.palette.background.paper,
            color: (theme) => theme.palette.text.primary,
            borderRadius: "3px",
            overflow: "hidden",
          },
          ["&  .fc-col-header, .fc-col-header-cell, fc-timegrid-axis, td, th, .fc-scrollgrid, .fc-timegrid-slot"]:
            {
              background: (theme) => theme.palette.background.paper,
              borderColor: (theme) =>
                isDarkMode ? theme.palette.grey[800] : theme.palette.grey[300],
            },
          ["& .fc-event-main:hover"]: {
            cursor: "pointer",
          },
          ["& .fc-event > *"]: {
            color: (theme) => theme.palette.text.primary,
            borderColor: (theme) =>
              isDarkMode ? theme.palette.grey[800] : theme.palette.grey[300],
          },
          ["& .fc-event"]: {
            boxShadow: "unset",
            borderRadius: "4px",
          },
          ["& .fc-popover"]: {
            backgroundColor: (theme) => theme.palette.background.paper,
            overflowX: "hidden",
          },
          ["& .fc-popover-body"]: {
            maxHeight: "300px",
            overflow: "scroll",
            backgroundColor: (theme) => theme.palette.background.paper,
            overflowX: "hidden",
            overflowY: "auto",
          },
          ["& .fc-popover-title"]: {
            color: (theme) => theme.palette.text.primary,
          },
          ["& .fc-timegrid-col-events > a"]: {
            backgroundColor: (theme) => theme.palette.primary.contrastText,
            boxShadow: "unset",
            borderRadius: "4px",
          },
        }}
      >
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
          // handleWindowResize={true}
          weekNumbers={true}
          weekText="S"
          initialView="timeGridWeek"
          locale="fr"
          allDaySlot={false}
          slotMinTime={"07:00:00"}
          slotDuration={{ minutes: view.includes("resource") ? 60 : 15 }}
          // slotDuration={60}
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
          eventMaxStack={view.includes("Day") ? 10 : 5}
          // eventMaxStack={10}
          eventClick={handleEventClick}
          resourceAreaWidth={120}
          moreLinkText={"éléments"}
        />
      </Box>
    </>
  );
};

export default Calendar;
