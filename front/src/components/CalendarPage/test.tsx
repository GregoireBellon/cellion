import { ExpandMore } from "@mui/icons-material";
import {
  Accordion,
  AccordionDetails,
  AccordionProps,
  AccordionSummary,
  Box,
  ToggleButton,
  ToggleButtonGroup,
  Typography,
} from "@mui/material";
import { FC, useEffect, useMemo } from "react";
import { ViewMode } from "../../types/calendar";
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

  useEffect(() => {
    setSearchParams((old) => {
      const newSearchParams = new URLSearchParams(old);
      newSearchParams.set("viewMode", ViewMode.DEFAULT);
      return newSearchParams;
    });
  }, [setSearchParams, viewModeParam]);

  return (
    <Accordion sx={sx} defaultExpanded>
      <AccordionSummary expandIcon={<ExpandMore />}>
        <Typography variant="h4">Affichage</Typography>
      </AccordionSummary>
      <AccordionDetails>
        <Box display="flex" flexDirection="column" gap={3}>
          <ToggleButtonGroup
            color="primary"
            value={viewModeParam}
            exclusive
            fullWidth
            size="small"
          >
            <ToggleButton value={ViewMode.DEFAULT}>Défaut</ToggleButton>
            <ToggleButton value={ViewMode.BY_ROOM}>Par salle</ToggleButton>
          </ToggleButtonGroup>
        </Box>
      </AccordionDetails>
    </Accordion>
  );
};

export default CalendarDisplay;
