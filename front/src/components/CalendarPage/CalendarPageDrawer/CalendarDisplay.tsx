import { ExpandMore } from "@mui/icons-material";
import {
  Accordion,
  AccordionDetails,
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
import { FC, useCallback } from "react";
import {
  CalendarDisplaySettings,
  ColorMode,
  ViewLevel,
  ViewMode,
} from "../../../types/calendar";

interface Props {
  value: CalendarDisplaySettings;
  onChange: (newValue: CalendarDisplaySettings) => void;
}

const CalendarDrawerDisplay: FC<Props> = ({ value, onChange }) => {
  const handleViewModeChange = useCallback(
    (
      _: React.MouseEvent<HTMLElement, MouseEvent>,
      newViewMode: ViewMode | null
    ) => {
      if (newViewMode === null) {
        return;
      }
      onChange({ ...value, viewMode: newViewMode });
    },
    [onChange, value]
  );

  const handleColorModeChange = useCallback(
    (e: SelectChangeEvent) => {
      onChange({ ...value, colorMode: e.target.value as ColorMode });
    },
    [onChange, value]
  );

  const handleViewLevelChange = useCallback(
    (
      _: React.MouseEvent<HTMLElement, MouseEvent>,
      newViewLevel: ViewLevel | null
    ) => {
      if (newViewLevel === null) {
        return;
      }
      onChange({ ...value, viewLevel: newViewLevel });
    },
    [onChange, value]
  );

  return (
    <Accordion sx={{ p: 1 }} defaultExpanded>
      <AccordionSummary expandIcon={<ExpandMore />}>
        <Typography variant="h4">Affichage</Typography>
      </AccordionSummary>
      <AccordionDetails>
        <Box display="flex" flexDirection="column" gap={3}>
          {/* <CustomDatePicker /> */}
          <ToggleButtonGroup
            color="primary"
            value={value.viewLevel}
            exclusive
            onChange={handleViewLevelChange}
            fullWidth
            size="small"
          >
            <ToggleButton value={ViewLevel.DAY}>Jour</ToggleButton>
            <ToggleButton value={ViewLevel.WEEK}>Semaine</ToggleButton>
            <ToggleButton value={ViewLevel.MONTH}>Mois</ToggleButton>
          </ToggleButtonGroup>
          <ToggleButtonGroup
            color="primary"
            value={value.viewMode}
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
              value={value.colorMode}
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

export default CalendarDrawerDisplay;
