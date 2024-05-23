import {
  ChangeEvent,
  FC,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import CalendarDrawer from "./CalendarPageDrawer/CalendarDrawer";
import Calendar from "./CalendarPageBody/Calendar";
import { Box, Typography } from "@mui/material";
import FullCalendar from "@fullcalendar/react";
import { useNavigate, useSearchParams } from "react-router-dom";
import sdk from "../../utils/sdk";
import {
  SolutionFiltersInfo,
  ReadSolutionBody,
  ShortSolutionInfo,
} from "../../types/api";
import {
  CalendarDisplaySettings,
  CalendarSearchParams,
  ColorMode,
  ViewLevel,
  ViewMode,
} from "../../types/calendar";
import CalendarSpeedDial from "./CalendarSpeedDial";
import { stringify } from "csv-stringify/browser/esm/sync";
import { DateTime } from "luxon";
import { ShortSessionInfo } from "../../types/core";
import { toast } from "react-toastify";
import { VisuallyHiddenInput } from "../VisuallyHiddenInput";
import { timestampStrToDateTime } from "../../utils/dates";

interface Props {
  solutionId: string;
}

const CalendarPage: FC<Props> = ({ solutionId }) => {
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();

  const fullCalendarRef = useRef<FullCalendar | null>(null);
  const solutionDebounce = useRef<ReturnType<typeof setTimeout> | null>(null);
  const hiddenInputRef = useRef<HTMLInputElement | null>(null);
  const initialSearchParamFrom = useRef<DateTime>(
    timestampStrToDateTime(searchParams.get(CalendarSearchParams.FROM))
  );

  const [solutionInfo, setSolutionInfo] = useState<ShortSolutionInfo>({
    createdAt: new Date(),
    fileName: "",
    calendarStart: new Date(),
    id: "",
  });

  const [sessions, setSessions] = useState<ShortSessionInfo[]>([]);
  const [solutionFiltersOptions, setSolutionFiltersOptions] =
    useState<SolutionFiltersInfo>({
      courses: [],
      groups: [],
      parts: [],
      rooms: [],
      teachers: [],
    });

  const [solutionFilters, setSolutionFilters] = useState<SolutionFiltersInfo>(
    () => ({
      courses: searchParams.getAll(CalendarSearchParams.COURSE) ?? [],
      groups: searchParams.getAll(CalendarSearchParams.GROUP) ?? [],
      parts: searchParams.getAll(CalendarSearchParams.PART) ?? [],
      rooms: searchParams.getAll(CalendarSearchParams.ROOM) ?? [],
      teachers: searchParams.getAll(CalendarSearchParams.TEACHER) ?? [],
    })
  );

  const [calendarDisplay, setCalendarDisplay] =
    useState<CalendarDisplaySettings>(() => ({
      viewMode:
        (searchParams.get(CalendarSearchParams.VIEW_MODE) as ViewMode) ??
        ViewMode.DEFAULT,
      viewLevel:
        (searchParams.get(CalendarSearchParams.VIEW_LEVEL) as ViewLevel) ??
        ViewLevel.WEEK,
      colorMode:
        (searchParams.get(CalendarSearchParams.COLOR_MODE) as ColorMode) ??
        ColorMode.BY_PART,
    }));

  const [from, setFrom] = useState<DateTime>(() =>
    timestampStrToDateTime(searchParams.get(CalendarSearchParams.FROM))
  );

  const [to, setTo] = useState<DateTime>(() =>
    timestampStrToDateTime(searchParams.get(CalendarSearchParams.TO))
  );

  const [calendarLoading, setCalendarLoading] = useState<boolean>(false);

  const initialFullCalendarDate = useMemo(
    () =>
      initialSearchParamFrom?.current?.isValid
        ? initialSearchParamFrom?.current?.toJSDate()
        : undefined,
    []
  );

  const handleFullCalendarDatesSet = useCallback(
    (activeStart: Date, activeEnd: Date) => {
      setFrom(DateTime.fromJSDate(activeStart));
      setTo(DateTime.fromJSDate(activeEnd));
    },
    []
  );

  const handleDrawerFiltersChange = useCallback(
    (newSolutionFilters: SolutionFiltersInfo) => {
      setSolutionFilters(newSolutionFilters);
    },
    []
  );

  const handleDisplayChange = useCallback(
    (newCalendarDisplay: CalendarDisplaySettings) =>
      setCalendarDisplay(newCalendarDisplay),
    []
  );

  const handlePrevPeriod = useCallback(() => {
    fullCalendarRef.current?.getApi()?.prev();
  }, []);

  const handleNextPeriod = useCallback(() => {
    fullCalendarRef.current?.getApi()?.next();
  }, []);

  const handleImportSolutionClick = useCallback(() => {
    hiddenInputRef.current?.click();
  }, []);

  const handleExportJSONClick = useCallback(() => {
    const fileName = "export";
    const json = JSON.stringify(sessions, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    const href = URL.createObjectURL(blob);

    const link = document.createElement("a");
    link.href = href;
    link.download = fileName + ".json";
    document.body.appendChild(link);
    link.click();

    document.body.removeChild(link);
    URL.revokeObjectURL(href);
  }, [sessions]);

  const handleExportCSVClick = useCallback(() => {
    const fileName = "export";
    const csv = stringify(
      sessions.map((s) => ({
        ...s,
        from: s.from.toISOString(),
        to: s.to.toISOString(),
        groups: s.groups.map((g) => g.id).join(", "),
        rooms: s.rooms.map((r) => r.id).join(", "),
        teachers: s.teachers.map((t) => t.id).join(", "),
      })),
      {
        columns: [
          { key: "from", header: "Debut" },
          { key: "to", header: "Fin" },
          { key: "course.id", header: "Matière" },
          { key: "part.id", header: "Catégorie" },
          { key: "teachers", header: "Enseignant(s)" },
          { key: "rooms", header: "Salle(s)" },
          { key: "groups", header: "Groupe(s)" },
        ],
        header: true,
      }
    );
    const blob = new Blob([csv], { type: "application/csv" });
    const href = URL.createObjectURL(blob);

    const link = document.createElement("a");
    link.href = href;
    link.download = fileName + ".csv";
    document.body.appendChild(link);
    link.click();

    document.body.removeChild(link);
    URL.revokeObjectURL(href);
  }, [sessions]);

  const fetchFilters = useCallback(async (id: string) => {
    try {
      const filters = await toast.promise(sdk.getFilters(id), {
        error: "Impossible de charger les filtres",
      });
      setSolutionFiltersOptions(filters);
      setSolutionFilters((oldFilters) => ({
        courses: oldFilters.courses.filter((c) => filters.courses.includes(c)),
        groups: oldFilters.groups.filter((g) => filters.groups.includes(g)),
        parts: oldFilters.parts.filter((p) => filters.parts.includes(p)),
        rooms: oldFilters.rooms.filter((r) => filters.rooms.includes(r)),
        teachers: oldFilters.teachers.filter((t) =>
          filters.teachers.includes(t)
        ),
      }));
    } catch (err) {
      console.error((err as Error).message);
    }
  }, []);

  const fetchSolution = useCallback(async (id: string) => {
    try {
      const solution = await toast.promise(sdk.getSolution(id), {
        error: "Impossible de charger la solution",
      });
      setSolutionInfo(solution);
      if (!initialSearchParamFrom.current?.isValid) {
        fullCalendarRef.current?.getApi()?.gotoDate(solution.calendarStart);
      }
    } catch (err) {
      console.error((err as Error).message);
    }
  }, []);

  const querySolution = useCallback((id: string, body: ReadSolutionBody) => {
    if (solutionDebounce.current !== null) {
      clearTimeout(solutionDebounce.current);
    }
    solutionDebounce.current = setTimeout(async () => {
      setCalendarLoading(true);
      try {
        const newSessions = await toast.promise(sdk.querySolution(id, body), {
          error: "Impossible de charger le calendrier",
        });
        setSessions(newSessions);
      } catch (err) {
        console.error((err as Error).message);
      }
      setCalendarLoading(false);
    }, 50);
  }, []);

  const handleImportSolution = useCallback(
    async (e: ChangeEvent<HTMLInputElement>) => {
      if (e.target.files === null) {
        return;
      }
      try {
        const file = e.target.files[0];
        const data = await toast.promise(sdk.importSolution(file), {
          pending: "Import de la solution...",
          error: "Echec de l'import de la solution",
          success: "🚀 Solution importée avec succès !",
        });

        navigate(`/calendar/${data.id}`);
      } catch (err) {
        console.error((err as Error).message);
      }
    },
    [navigate]
  );

  useEffect(() => {
    void fetchSolution(solutionId);
  }, [fetchSolution, solutionId]);

  useEffect(() => {
    void fetchFilters(solutionId);
  }, [fetchFilters, solutionId]);

  useEffect(() => {
    const isoFrom = from?.toISO();
    const isoTo = to?.toISO();
    if (!isoFrom || !isoTo) {
      return;
    }
    querySolution(solutionId, { ...solutionFilters, from: isoFrom, to: isoTo });
  }, [querySolution, solutionId, from, solutionFilters, to]);

  useEffect(() => {
    setSearchParams(
      () => {
        const map: string[][] = [];
        if (from.isValid && to.isValid) {
          map.push([CalendarSearchParams.FROM, from.toMillis().toString()]);
          map.push([CalendarSearchParams.TO, to.toMillis().toString()]);
        }
        map.push(
          ...solutionFilters.courses.map((course) => [
            CalendarSearchParams.COURSE,
            course,
          ])
        );
        map.push(
          ...solutionFilters.groups.map((group) => [
            CalendarSearchParams.GROUP,
            group,
          ])
        );
        map.push(
          ...solutionFilters.parts.map((part) => [
            CalendarSearchParams.PART,
            part,
          ])
        );
        map.push(
          ...solutionFilters.rooms.map((room) => [
            CalendarSearchParams.ROOM,
            room,
          ])
        );
        map.push(
          ...solutionFilters.teachers.map((teacher) => [
            CalendarSearchParams.TEACHER,
            teacher,
          ])
        );
        map.push([CalendarSearchParams.COLOR_MODE, calendarDisplay.colorMode]);
        map.push([CalendarSearchParams.VIEW_MODE, calendarDisplay.viewMode]);
        map.push([CalendarSearchParams.VIEW_LEVEL, calendarDisplay.viewLevel]);

        return new URLSearchParams(map);
      },
      { replace: true }
    );
  }, [from, setSearchParams, solutionFilters, to, calendarDisplay]);

  return (
    <>
      <Box display="flex" flexGrow={1} flexDirection="column" gap={1}>
        <Box
          display="flex"
          flexDirection="row"
          justifyContent="space-between"
          gap={2}
        >
          <Box flex={2} />
          <Box flex={6}>
            <Typography width={1300} noWrap variant="h4">
              {solutionInfo.fileName}
            </Typography>
          </Box>
        </Box>
        <Box display="flex" flexDirection="row" gap={2}>
          <Box flex={2}>
            <CalendarDrawer
              from={from}
              to={to}
              onPrevDate={handlePrevPeriod}
              onNextDate={handleNextPeriod}
              filtersOptions={solutionFiltersOptions}
              filters={solutionFilters}
              onFiltersChange={handleDrawerFiltersChange}
              display={calendarDisplay}
              onDisplayChange={handleDisplayChange}
            />
          </Box>
          <Box flex={6}>
            <Calendar
              sessions={sessions}
              fullCalendarRef={fullCalendarRef}
              initialFrom={initialFullCalendarDate}
              onDatesSet={handleFullCalendarDatesSet}
              display={calendarDisplay}
              loading={calendarLoading}
            />
          </Box>
        </Box>
      </Box>
      <CalendarSpeedDial
        onExportJSONClick={handleExportJSONClick}
        onExportCSVClick={handleExportCSVClick}
        onImportSolutionClick={handleImportSolutionClick}
      />
      <VisuallyHiddenInput
        ref={hiddenInputRef}
        onChange={handleImportSolution}
        type="file"
        accept=".xml"
      />
    </>
  );
};

export default CalendarPage;
