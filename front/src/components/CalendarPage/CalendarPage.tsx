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
import { Box } from "@mui/material";
import FullCalendar from "@fullcalendar/react";
import { useNavigate, useSearchParams } from "react-router-dom";
import sdk from "../../utils/sdk";
import { SolutionFiltersInfo, ReadSolutionBody } from "../../types/api";
import {
  CalendarDisplaySettings,
  CalendarSearchParams,
  ColorMode,
  ViewLevel,
  ViewMode,
} from "../../types/calendar";
import CalendarSpeedDial from "./CalendarSpeedDial";
import { stringify } from "csv-stringify/browser/esm/sync";
import CalendarHeaderToolbar from "./CalendarPageBody/CalendarHeaderToolbar";
import { DateTime, Interval } from "luxon";
import { ShortSessionInfo } from "../../types/core";
import { toast } from "react-toastify";
import { VisuallyHiddenInput } from "../VisuallyHiddenInput";

interface Props {
  solutionId: string;
}

const CalendarPage: FC<Props> = ({ solutionId }) => {
  const navigate = useNavigate();

  const fullCalendarRef = useRef<FullCalendar | null>(null);
  const solutionDebounce = useRef<ReturnType<typeof setTimeout> | null>(null);
  const hiddenInputRef = useRef<HTMLInputElement | null>(null);

  const [searchParams, setSearchParams] = useSearchParams();
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

  const [from, setFrom] = useState<DateTime>(() => {
    const searchParamsFrom = searchParams.get(CalendarSearchParams.FROM);
    if (searchParamsFrom === null) {
      return DateTime.invalid("empty");
    }
    return DateTime.fromMillis(Number.parseInt(searchParamsFrom));
  });

  const [to, setTo] = useState<DateTime>(() => {
    const searchParamsTo = searchParams.get(CalendarSearchParams.TO);
    if (searchParamsTo === null) {
      return DateTime.invalid("empty");
    }
    return DateTime.fromMillis(Number.parseInt(searchParamsTo));
  });

  const [calendarLoading, setCalendarLoading] = useState<boolean>(false);

  const initialFullCalendarDate = useMemo(
    () => (from.isValid ? from.toJSDate() : new Date()),
    [from]
  );

  const intervalStr = useMemo(() => {
    if (!from.isValid || !to.isValid) {
      return "";
    }

    return Interval.fromDateTimes(from, to).toLocaleString(DateTime.DATE_MED, {
      locale: "fr-FR",
    });
  }, [from, to]);

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

  const fetchFilters = useCallback(async (id: string) => {
    try {
      const filters = await toast.promise(sdk.getFilters(id), {
        error: "Impossible de charger les filtres",
      });
      setSolutionFiltersOptions(filters);
      setSolutionFilters({
        courses: [],
        groups: [],
        parts: [],
        rooms: [],
        teachers: [],
      });
    } catch (err) {
      console.error((err as Error).message);
    }
  }, []);

  const fetchSolution = useCallback((id: string, body: ReadSolutionBody) => {
    if (solutionDebounce.current !== null) {
      clearTimeout(solutionDebounce.current);
    }
    solutionDebounce.current = setTimeout(async () => {
      setCalendarLoading(true);
      try {
        const newSolution = await toast.promise(sdk.getSolution(id, body), {
          error: "Impossible de charger la solution",
        });
        setSessions(newSolution);
      } catch (err) {
        console.error((err as Error).message);
      }
      setCalendarLoading(false);
    }, 50);
  }, []);

  const handlePrevPeriod = useCallback(() => {
    fullCalendarRef.current?.getApi()?.prev();
  }, []);

  const handleNextPeriod = useCallback(() => {
    fullCalendarRef.current?.getApi()?.next();
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

  const handleImportSolutionClick = useCallback(() => {
    hiddenInputRef.current?.click();
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

  const handleDrawerDatesChange = useCallback((newDate: DateTime | null) => {
    if (newDate === null) {
      return;
    }
    fullCalendarRef.current?.getApi()?.gotoDate(newDate.toJSDate());
  }, []);

  const handleFullCalendarDatesSet = useCallback(
    (activeStart: Date, activeEnd: Date) => {
      const fullCalendarFrom = activeStart;
      const fullCalendarTo = activeEnd;

      setFrom(DateTime.fromJSDate(fullCalendarFrom));
      setTo(DateTime.fromJSDate(fullCalendarTo));
    },
    []
  );

  useEffect(() => {
    setSearchParams(() => {
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
    });
  }, [from, setSearchParams, solutionFilters, to, calendarDisplay]);

  useEffect(() => {
    if (solutionId === undefined) {
      return;
    }
    void fetchFilters(solutionId);
  }, [fetchFilters, solutionId]);

  useEffect(() => {
    if (solutionId === undefined) {
      return;
    }
    const isoFrom = from?.toISO();
    const isoTo = to?.toISO();
    if (!isoFrom || !isoTo) {
      return;
    }
    fetchSolution(solutionId, { ...solutionFilters, from: isoFrom, to: isoTo });
  }, [fetchSolution, solutionId, from, solutionFilters, to]);

  return (
    <>
      <Box display="flex" flexGrow={1} flexDirection="column">
        {/* Calendar title ? */}
        <Box display="flex" flexDirection="row" flexGrow={1} gap={2}>
          <CalendarDrawer
            filtersOptions={solutionFiltersOptions}
            filters={solutionFilters}
            onFiltersChange={handleDrawerFiltersChange}
            date={from}
            onDateChange={handleDrawerDatesChange}
            display={calendarDisplay}
            onDisplayChange={handleDisplayChange}
          />
          <Box display="flex" flexDirection="column">
            <CalendarHeaderToolbar
              onPrev={handlePrevPeriod}
              onNext={handleNextPeriod}
              interval={intervalStr}
            />
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
