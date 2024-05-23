import { Box, Dialog, DialogContent, DialogTitle } from "@mui/material";
import { FC, useCallback, useMemo } from "react";
import { ShortSessionInfo } from "../../../types/core";
import chroma from "chroma-js";
import {
  Category,
  Groups,
  Key,
  Person,
  School,
  WatchLater,
} from "@mui/icons-material";
import { DateTime, Interval } from "luxon";
import { useIsDarkMode } from "../../../utils/colors/useIsDarkMode";

interface Props {
  session: (ShortSessionInfo & { color: string }) | null;
  open: boolean;
  onClose: () => void;
}

const EventDetailsDialog: FC<Props> = ({ session, open, onClose }) => {
  const isDarkMode = useIsDarkMode();

  const backgroundColor = useMemo(() => {
    if (session === null) {
      return undefined;
    }
    const color = chroma(session.color);

    return (isDarkMode ? color.darken(1.5) : color.brighten(1.5)).css();
  }, [isDarkMode, session]);

  const handleClose = useCallback(() => {
    onClose();
  }, [onClose]);

  return (
    <Dialog
      open={open}
      PaperProps={{
        sx: {
          backgroundColor,
          minWidth: 400,
        },
      }}
      onClose={handleClose}
    >
      <DialogTitle>Détails</DialogTitle>
      {session && (
        <DialogContent>
          <Box display="flex" flexDirection="column" gap={0.6}>
            <Box display="flex" flexDirection="row" gap={2}>
              <WatchLater />
              {Interval.fromDateTimes(
                DateTime.fromJSDate(session.from),
                DateTime.fromJSDate(session.to)
              ).toLocaleString(DateTime.TIME_24_SIMPLE)}
            </Box>
            <Box display="flex" flexDirection="row" gap={2}>
              <School /> {session.course.id}
            </Box>
            <Box display="flex" flexDirection="row" gap={2}>
              <Person /> {session.teachers.map(({ id }) => id).join(", ")}
            </Box>
            <Box display="flex" flexDirection="row" gap={2}>
              <Key /> {session.rooms.map(({ id }) => id).join(", ")}
            </Box>
            <Box display="flex" flexDirection="row" gap={2}>
              <Groups /> {session.groups.map(({ id }) => id).join(", ")}
            </Box>
            <Box display="flex" flexDirection="row" gap={2}>
              <Category /> {session.part.id}
            </Box>
          </Box>
        </DialogContent>
      )}
    </Dialog>
  );
};

export default EventDetailsDialog;
