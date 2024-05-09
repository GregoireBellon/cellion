import { FC, useCallback, useEffect, useMemo, useRef, useState } from "react";
import CalendarDrawer from "./CalendarDrawer";
import Calendar from "./Calendar";
import { Box } from "@mui/material";
import FullCalendar from "@fullcalendar/react";
import { useParams, useSearchParams } from "react-router-dom";
import sdk from "../../utils/sdk";
import {
  CalendarFiltersInfo,
  CalendarInfo,
  ReadCalendarBody,
} from "../../types/api";

const CalendarPage: FC = () => {
  const { fileId } = useParams<"fileId">();
  const fullCalendarRef = useRef<FullCalendar>(null);

  const [searchParams, setSearchParams] = useSearchParams();
  const [calendar, setCalendar] = useState<CalendarInfo>({ sessions: [] });
  const [calendarFilters, setCalendarFilters] = useState<CalendarFiltersInfo>({
    courses: [],
    groups: [],
    parts: [],
    rooms: [],
    teachers: [],
  });

  const readCalendarBody: ReadCalendarBody = useMemo(
    () => ({
      courses: searchParams.getAll("course"),
      parts: searchParams.getAll("parts"),
      groups: searchParams.getAll("group"),
      rooms: searchParams.getAll("room"),
      students: searchParams.getAll("student"),
      teachers: searchParams.getAll("teacher"),
    }),
    [searchParams]
  );

  const fromDateUrlParam = useMemo(() => {
    const dateStr = searchParams.get("from");
    if (dateStr === null) {
      return new Date();
    }

    const timestamp = Number.parseInt(dateStr);
    if (Number.isNaN(timestamp)) {
      return new Date();
    }
    return new Date(timestamp);
  }, [searchParams]);

  const fetchFilters = useCallback(async (id: string) => {
    try {
      const filters = await sdk.getFilters(id);
      setCalendarFilters(filters);
    } catch (err) {
      console.error((err as Error).message);
    }
  }, []);

  const fetchCalendar = useCallback(
    async (id: string, body: ReadCalendarBody) => {
      try {
        const newCalendar = await sdk.getCalendar(id, body);
        setCalendar(newCalendar);
      } catch (err) {
        console.error((err as Error).message);
      }
    },
    []
  );

  useEffect(() => {
    if (fileId === undefined) {
      return;
    }
    void fetchFilters(fileId);
  }, [fetchFilters, fileId]);

  // TODO ADD DEBOUNCE
  useEffect(() => {
    if (fileId === undefined) {
      return;
    }
    void fetchCalendar(fileId, readCalendarBody);
  }, [fetchCalendar, fileId, readCalendarBody]);

  useEffect(() => {
    const fullCalendarApi = fullCalendarRef.current?.getApi();
    if (fullCalendarApi === undefined) {
      return;
    }
    setSearchParams((old) => {
      const newSearchParams = new URLSearchParams(old);
      newSearchParams.set(
        "from",
        fullCalendarApi.view.activeStart.getTime().toString()
      );
      newSearchParams.set(
        "to",
        fullCalendarApi.view.activeEnd.getTime().toString()
      );
      return newSearchParams;
    });
  }, [setSearchParams]);

  return (
    <Box display="flex" flexGrow={1} flexDirection="column">
      {/* Calendar title ? */}
      <Box display="flex" flexDirection="row" flexGrow={1} gap={2}>
        <CalendarDrawer calendarFilters={calendarFilters} />
        <Calendar
          calendar={calendar}
          fullCalendarRef={fullCalendarRef}
          from={fromDateUrlParam}
        />
      </Box>
    </Box>
  );
};

export default CalendarPage;
