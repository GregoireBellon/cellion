import { CalendarMonth } from "@mui/icons-material";
import {
  SpeedDial,
  SpeedDialAction,
  SpeedDialIcon,
  Typography,
} from "@mui/material";
import { FC, useCallback, useState } from "react";

interface Props {
  onExportJSONClick: () => void;
  onExportCSVClick: () => void;
  onImportInstanceClick: () => void;
}

const CalendarSpeedDial: FC<Props> = ({
  onExportJSONClick,
  onExportCSVClick,
  onImportInstanceClick,
}) => {
  const [open, setOpen] = useState<boolean>(false);

  const handleOpen = useCallback(() => {
    setOpen(true);
  }, []);

  const handleClose = useCallback(() => {
    setOpen(false);
  }, []);

  const handleExportJSONClick = useCallback(() => {
    setOpen(false);
    onExportJSONClick();
  }, [onExportJSONClick]);

  const handleExportCSVClick = useCallback(() => {
    setOpen(false);
    onExportCSVClick();
  }, [onExportCSVClick]);

  const handleImportInstanceClick = useCallback(() => {
    setOpen(false);
    onImportInstanceClick();
  }, [onImportInstanceClick]);

  return (
    <SpeedDial
      sx={{
        position: "fixed",
        bottom: 32,
        right: 32,
        ["& > .MuiSpeedDial-fab"]: {
          opacity: open ? 1 : 0.4,
        },
      }}
      ariaLabel="speed-dial"
      icon={<SpeedDialIcon />}
      open={open}
      onOpen={handleOpen}
      onClose={handleClose}
    >
      <SpeedDialAction
        icon={
          <Typography fontSize={12} fontWeight={800}>
            .CSV
          </Typography>
        }
        tooltipTitle="Exporter en csv"
        onClick={handleExportCSVClick}
      />
      <SpeedDialAction
        icon={
          <Typography fontSize={12} fontWeight={800}>
            .JSON
          </Typography>
        }
        tooltipTitle="Exporter en json"
        onClick={handleExportJSONClick}
      />
      <SpeedDialAction
        icon={<CalendarMonth />}
        tooltipTitle="Importer une instance"
        onClick={handleImportInstanceClick}
      />
    </SpeedDial>
  );
};

export default CalendarSpeedDial;
