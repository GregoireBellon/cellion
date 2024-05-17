import { Button, styled } from "@mui/material";
import { FC, useCallback, useEffect, useState } from "react";
import CalendarSearchDialog from "../CalendarSearchDialog";
import { Search } from "@mui/icons-material";

const KBM = styled("span", {
  shouldForwardProp: (propName) => propName !== "size",
})<{ size?: "small" | "medium" }>(({ theme, size }) => ({
  background: `linear-gradient(180deg, ${theme.palette.grey[100]} 0%, ${theme.palette.grey[50]} 75%)`,
  borderRadius: theme.shape.borderRadius,
  boxShadow: `inset 0 1px 2px 1px white, 0 1px 0 0 ${theme.palette.grey[600]}`,
  color: theme.palette.text.secondary,
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  padding: size === "small" ? "0 2px" : "0 0.25em",
  height: size === "small" ? "20px" : "1.5em",
  minWidth: size === "small" ? "10px" : "1.5em",
  fontSize: size === "small" ? "13px !important" : "1em",
}));

const CalendarSearch: FC = () => {
  const [open, setOpen] = useState<boolean>(false);

  const handleSearchDialogClose = useCallback(() => {
    setOpen(false);
  }, []);

  const handleSearchButtonClick = useCallback(() => {
    setOpen(true);
  }, []);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.ctrlKey && event.key === "k") {
        event.preventDefault();
        setOpen(true);
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  return (
    <>
      <CalendarSearchDialog open={open} onClose={handleSearchDialogClose} />
      <Button
        startIcon={<Search />}
        endIcon={<KBM size="small">Ctrl+k</KBM>}
        sx={{ textTransform: "none", borderRadius: "10px", mb: 2 }}
        fullWidth
        onClick={handleSearchButtonClick}
        disableFocusRipple
      >
        Rechercher une solution...
      </Button>
    </>
  );
};

export default CalendarSearch;
