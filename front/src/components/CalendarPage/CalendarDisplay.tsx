import { ExpandMore } from "@mui/icons-material";
import {
  Accordion,
  AccordionDetails,
  AccordionProps,
  AccordionSummary,
  Box,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  ToggleButton,
  ToggleButtonGroup,
  Typography,
} from "@mui/material";
import { FC, useCallback, useEffect, useMemo } from "react";
import { ColorMode, ViewMode } from "../../types/calendar";
import { useSearchParams } from "react-router-dom";

interface Props {
  sx?: AccordionProps["sx"];
}

const CalendarDisplay: FC<Props> = ({ sx }) => {
  const [searchParams, setSearchParams] = useSearchParams();

  const viewModeParam = useMemo(
    () => searchParams.get("viewMode"),
    [searchParams]
  );

  const colorModeParam = useMemo(
    () => searchParams.get("colorMode"),
    [searchParams]
  );

  const handleViewModeChange = useCallback(
    (
      _: React.MouseEvent<HTMLElement, MouseEvent>,
      newViewMode: ViewMode | null
    ) => {
      if (newViewMode === null) {
        return;
      }
      setSearchParams((old) => {
        const newSearchParams = new URLSearchParams(old);
        newSearchParams.set("viewMode", newViewMode);
        return newSearchParams;
      });
    },
    [setSearchParams]
  );

  const handleColorModeChange = useCallback(
    (e: SelectChangeEvent) => {
      setSearchParams((old) => {
        const newSearchParams = new URLSearchParams(old);
        newSearchParams.set("colorMode", e.target.value);
        return newSearchParams;
      });
    },
    [setSearchParams]
  );

  // doesnt work with deps
  useEffect(() => {
    if (!viewModeParam) {
      setSearchParams((old) => {
        const newSearchParams = new URLSearchParams(old);
        newSearchParams.set("viewMode", ViewMode.DEFAULT);
        return newSearchParams;
      });
    }
  });

  // doesnt work with deps
  useEffect(() => {
    if (!colorModeParam) {
      setSearchParams((old) => {
        const newSearchParams = new URLSearchParams(old);
        newSearchParams.set("colorMode", ColorMode.BY_PART);
        return newSearchParams;
      });
    }
  });

  return (
    <Accordion sx={sx} defaultExpanded>
      <AccordionSummary expandIcon={<ExpandMore />}>
        <Typography variant="h4">Affichage</Typography>
      </AccordionSummary>
      <AccordionDetails>
        <Box display="flex" flexDirection="column" gap={3}>
          <ToggleButtonGroup
            color="primary"
            value={viewModeParam ?? ViewMode.DEFAULT}
            exclusive
            onChange={handleViewModeChange}
            fullWidth
            size="small"
          >
            <ToggleButton value={ViewMode.DEFAULT}>Défaut</ToggleButton>
            <ToggleButton value={ViewMode.BY_ROOM}>Par salle</ToggleButton>
          </ToggleButtonGroup>
          <FormControl fullWidth>
            <InputLabel>Colorer par</InputLabel>
            <Select
              value={colorModeParam ?? ColorMode.BY_PART}
              label="ColorerPar"
              onChange={handleColorModeChange}
            >
              <MenuItem value={ColorMode.BY_PART}>Catégorie</MenuItem>
              <MenuItem value={ColorMode.BY_COURSE}>Matière</MenuItem>
              <MenuItem value={ColorMode.BY_ROOM}>Salle</MenuItem>
              <MenuItem value={ColorMode.BY_TEACHER}>Enseignant</MenuItem>
            </Select>
          </FormControl>
        </Box>
      </AccordionDetails>
    </Accordion>
  );
};

export default CalendarDisplay;
