import {
  Accordion,
  AccordionDetails,
  AccordionProps,
  AccordionSummary,
  Typography,
} from "@mui/material";
import { FC, useCallback } from "react";
import CalendarFilter from "./CalendarFilter";
import { CalendarFiltersInfo } from "../../types/api";
import { useSearchParams } from "react-router-dom";
import { ExpandMore } from "@mui/icons-material";

interface Props {
  calendarFilters: CalendarFiltersInfo;
  sx?: AccordionProps["sx"];
}

const CalendarFilters: FC<Props> = ({ calendarFilters, sx }) => {
  const [searchParams, setSearchParams] = useSearchParams();

  const handleItemChange = useCallback(
    (key: string) => (value: string, checked: boolean) => {
      setSearchParams((old) => {
        const newSearchParams = new URLSearchParams(old);
        if (checked) {
          if (!newSearchParams.has(key, value)) {
            newSearchParams.append(key, value);
          }
        } else {
          if (newSearchParams.has(key, value)) {
            newSearchParams.delete(key, value);
          }
        }
        return newSearchParams;
      });
    },
    [setSearchParams]
  );

  const isItemChecked = useCallback(
    (key: string, value: string) => {
      return searchParams.has(key, value);
    },
    [searchParams]
  );

  return (
    <Accordion defaultExpanded sx={sx}>
      <AccordionSummary expandIcon={<ExpandMore />}>
        <Typography variant="h4">Filtres</Typography>
      </AccordionSummary>
      <AccordionDetails>
        <CalendarFilter
          title="Cours"
          items={calendarFilters.courses.map((course) => ({
            id: course.id,
            label: course.name,
            checked: isItemChecked("course", course.id),
          }))}
          onItemChange={handleItemChange("course")}
        />
        <CalendarFilter
          title="Catégorie"
          items={calendarFilters.parts.map((part) => ({
            id: part.id,
            label: part.label,
            checked: isItemChecked("part", part.id),
          }))}
          onItemChange={handleItemChange("part")}
        />
        <CalendarFilter
          title="Salle"
          items={calendarFilters.rooms.map((room) => ({
            id: room.id,
            label: room.name,
            checked: isItemChecked("room", room.id),
          }))}
          onItemChange={handleItemChange("room")}
        />
        <CalendarFilter
          title="Groupe"
          items={calendarFilters.groups.map((group) => ({
            id: group.id,
            label: group.name,
            checked: isItemChecked("group", group.id),
          }))}
          onItemChange={handleItemChange("group")}
        />
        <CalendarFilter
          title="Enseignant"
          items={calendarFilters.teachers.map((teacher) => ({
            id: teacher.id,
            label: teacher.name,
            checked: isItemChecked("teacher", teacher.id),
          }))}
          onItemChange={handleItemChange("teacher")}
        />
      </AccordionDetails>
    </Accordion>
  );
};

export default CalendarFilters;
