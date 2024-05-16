import { FC, useCallback } from "react";
import { ArrowBack, ArrowForward } from "@mui/icons-material";
import { Box, IconButton, Typography } from "@mui/material";

interface Props {
  interval: string;
  onPrev: () => void;
  onNext: () => void;
}

const CalendarHeaderToolbar: FC<Props> = ({ interval, onPrev, onNext }) => {
  const handlePrev = useCallback(() => {
    onPrev();
  }, [onPrev]);

  const handleNext = useCallback(() => {
    onNext();
  }, [onNext]);

  return (
    <Box display="flex" flexDirection="row" justifyContent="center" gap={2}>
      <IconButton size="large" onClick={handlePrev}>
        <ArrowBack fontSize="large" />
      </IconButton>
      <Typography
        variant="h4"
        sx={{ lineHeight: 1.6, textAlign: "center", width: 400 }}
      >
        {interval}
      </Typography>
      <IconButton size="large" onClick={handleNext}>
        <ArrowForward fontSize="large" />
      </IconButton>
    </Box>
  );
};

export default CalendarHeaderToolbar;
