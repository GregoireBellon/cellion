import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
} from "@mui/material";
import { FC, useCallback } from "react";

interface Props {
  open: boolean;
  onClose: () => void;
  error: string;
}

const ImportErrorDialog: FC<Props> = ({ open, onClose, error }) => {
  const handleClose = useCallback(() => {
    onClose();
  }, [onClose]);

  return (
    <Dialog open={open} onClose={handleClose}>
      <DialogTitle>Erreur d'import</DialogTitle>
      <DialogContent>
        <DialogContentText>{error}</DialogContentText>
      </DialogContent>
      <DialogActions>
        <Button onClick={handleClose}>Fermer</Button>
      </DialogActions>
    </Dialog>
  );
};

export default ImportErrorDialog;
