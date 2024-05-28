import { ChangeEvent, FC, useCallback, useRef, useState } from "react";
import { Alert, Box, Button, Divider } from "@mui/material";
import { VisuallyHiddenInput } from "../VisuallyHiddenInput";
import { toast } from "react-toastify";
import sdk from "../../utils/sdk";
import { useNavigate } from "react-router-dom";
import { CalendarMonth } from "@mui/icons-material";
import CalendarSearchButton from "./CalendarSearch/CalendarSearchButton";
import { isAxiosError } from "axios";
import ImportErrorDialog from "./ImportErrorDialog";

const NewCalendarPage: FC = () => {
  const navigate = useNavigate();

  const hiddenInputRef = useRef<HTMLInputElement | null>(null);

  const [importError, setImportError] = useState<string>("");
  const [importErrorDialogOpen, setImportErrorDialogOpen] =
    useState<boolean>(false);

  const handleImportSolutionClick = useCallback(() => {
    if (hiddenInputRef.current) {
      hiddenInputRef.current.value = "";
      hiddenInputRef.current.click();
    }
  }, []);

  const handleImportSolution = useCallback(
    async (e: ChangeEvent<HTMLInputElement>) => {
      if (e.target.files === null) {
        return;
      }
      try {
        const file = e.target.files[0];
        const data = await toast.promise(sdk.importSolution(file), {
          pending: "Import de la solution...",
          error: "Echec de l'import de la solution",
          success: "🚀 Solution importée avec succès !",
        });

        navigate(`/calendar/${data.id}`);
      } catch (err) {
        console.error((err as Error).message);
        if (isAxiosError(err)) {
          setImportError(err.response?.data);
          setImportErrorDialogOpen(true);
        }
      }
    },
    [navigate]
  );

  const handleCalendarSearchButtonClick = useCallback(() => {
    document.dispatchEvent(
      new KeyboardEvent("keydown", { ctrlKey: true, key: "k" })
    );
  }, []);

  const handleImportErrorDialogClose = useCallback(() => {
    setImportErrorDialogOpen(false);
  }, []);

  return (
    <>
      <Box
        display="flex"
        flexDirection="column"
        alignItems="center"
        justifyContent="center"
        sx={{ width: "100%" }}
      >
        <Box height={300} />

        <Box display="flex" flexDirection="column" gap={2} width={1000}>
          <Alert severity="info" variant="outlined">
            Pour commencer, importez ou selectionnez une solution
          </Alert>
          <CalendarSearchButton
            size="large"
            variant="outlined"
            sx={{ textTransform: "none", borderRadius: "10px" }}
            onClick={handleCalendarSearchButtonClick}
          />
          <Divider
            orientation="vertical"
            variant="middle"
            flexItem
            sx={{ borderWidth: "0.1px", width: "95%", alignSelf: "center" }}
          />
          <Button
            onClick={handleImportSolutionClick}
            sx={{ textTransform: "none", borderRadius: "10px" }}
            startIcon={<CalendarMonth />}
            size="large"
            variant="outlined"
          >
            Importer une solution
          </Button>
        </Box>
        <VisuallyHiddenInput
          ref={hiddenInputRef}
          onChange={handleImportSolution}
          type="file"
          accept=".xml"
        />
      </Box>
      <ImportErrorDialog
        open={importErrorDialogOpen}
        onClose={handleImportErrorDialogClose}
        error={importError}
      />
    </>
  );
};

export default NewCalendarPage;
