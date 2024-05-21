import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Box,
  Typography,
} from "@mui/material";
import { FC, useCallback } from "react";
import CalendarFilter from "./CalendarFilter";
import { ExpandMore } from "@mui/icons-material";
import { SolutionFiltersInfo } from "../../../types/api";

interface Props {
  options: SolutionFiltersInfo;
  value: SolutionFiltersInfo;
  onChange: (neValue: SolutionFiltersInfo) => void;
}

const CalendarFilters: FC<Props> = ({ options, value, onChange }) => {
  const handleCoursesChange = useCallback(
    (newCourses: string[]) => {
      onChange({
        ...value,
        courses: newCourses,
      });
    },
    [value, onChange]
  );

  const handlePartsChange = useCallback(
    (newParts: string[]) => {
      onChange({
        ...value,
        parts: newParts,
      });
    },
    [value, onChange]
  );

  const handleRoomsChange = useCallback(
    (newRooms: string[]) => {
      onChange({
        ...value,
        rooms: newRooms,
      });
    },
    [value, onChange]
  );

  const handleGroupsChange = useCallback(
    (newGroups: string[]) => {
      onChange({
        ...value,
        groups: newGroups,
      });
    },
    [value, onChange]
  );

  const handleTeachersChange = useCallback(
    (newTeachers: string[]) => {
      onChange({
        ...value,
        teachers: newTeachers,
      });
    },
    [value, onChange]
  );

  const handleCoursesClear = useCallback(() => {
    onChange({ ...value, courses: [] });
  }, [value, onChange]);

  const handlePartsClear = useCallback(() => {
    onChange({ ...value, parts: [] });
  }, [value, onChange]);

  const handleRoomsClear = useCallback(() => {
    onChange({ ...value, rooms: [] });
  }, [value, onChange]);

  const handleGroupsClear = useCallback(() => {
    onChange({ ...value, groups: [] });
  }, [value, onChange]);

  const handleTeachersClear = useCallback(() => {
    onChange({ ...value, teachers: [] });
  }, [value, onChange]);

  return (
    <Accordion defaultExpanded sx={{ p: 1 }}>
      <AccordionSummary expandIcon={<ExpandMore />}>
        <Typography variant="h4">Filtres</Typography>
      </AccordionSummary>
      <AccordionDetails>
        <Box display="flex" flexDirection="column" gap={3}>
          <CalendarFilter
            title="Cours"
            options={options.courses}
            value={value.courses}
            onChange={handleCoursesChange}
            onClear={handleCoursesClear}
          />
          <CalendarFilter
            title="Catégories"
            options={options.parts}
            value={value.parts}
            onChange={handlePartsChange}
            onClear={handlePartsClear}
          />
          <CalendarFilter
            title="Salles"
            options={options.rooms}
            value={value.rooms}
            onChange={handleRoomsChange}
            onClear={handleRoomsClear}
          />
          <CalendarFilter
            title="Groupes"
            options={options.groups}
            value={value.groups}
            onChange={handleGroupsChange}
            onClear={handleGroupsClear}
          />
          <CalendarFilter
            title="Enseignants"
            options={options.teachers}
            value={value.teachers}
            onChange={handleTeachersChange}
            onClear={handleTeachersClear}
          />
        </Box>
      </AccordionDetails>
    </Accordion>
  );
};

export default CalendarFilters;
