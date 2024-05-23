import { ArrowBack, ArrowForward } from "@mui/icons-material";
import { Paper, Box, IconButton, Typography } from "@mui/material";
import { DateTime, Interval } from "luxon";
import { FC, useCallback, useMemo } from "react";

interface Props {
  from: DateTime;
  to: DateTime;
  onPrev: () => void;
  onNext: () => void;
}

const CalendarDates: FC<Props> = ({ from, to, onPrev, onNext }) => {
  const intervalStr = useMemo(() => {
    if (!from.isValid || !to.isValid) {
      return "";
    }

    return Interval.fromDateTimes(from, to).toLocaleString(DateTime.DATE_MED, {
      locale: "fr-FR",
    });
  }, [from, to]);

  const handlePrev = useCallback(() => {
    onPrev();
  }, [onPrev]);

  const handleNext = useCallback(() => {
    onNext();
  }, [onNext]);

  return (
    <Paper sx={{ borderRadius: 2 }}>
      <Box
        display="flex"
        flexDirection="row"
        alignItems="center"
        justifyContent="space-between"
        gap={2}
      >
        <IconButton size="large" onClick={handlePrev}>
          <ArrowBack fontSize="medium" />
        </IconButton>
        <Typography variant="h5" sx={{ textAlign: "center" }}>
          {intervalStr}
        </Typography>
        <IconButton size="large" onClick={handleNext}>
          <ArrowForward fontSize="medium" />
        </IconButton>
      </Box>
    </Paper>
  );
};

export default CalendarDates;
