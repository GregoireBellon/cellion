import { ChangeEvent, FC, useCallback, useRef } from "react";
import { Alert, Box, Button, Divider } from "@mui/material";
import { VisuallyHiddenInput } from "../VisuallyHiddenInput";
import { toast } from "react-toastify";
import sdk from "../../utils/sdk";
import { useNavigate } from "react-router-dom";
import { CalendarMonth } from "@mui/icons-material";
import CalendarSearchButton from "./CalendarSearch/CalendarSearchButton";

const NewCalendarPage: FC = () => {
  const navigate = useNavigate();

  const hiddenInputRef = useRef<HTMLInputElement | null>(null);

  const handleImportSolutionClick = useCallback(() => {
    hiddenInputRef.current?.click();
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
      }
    },
    [navigate]
  );

  const handleCalendarSearchButtonClick = useCallback(() => {
    document.dispatchEvent(
      new KeyboardEvent("keydown", { ctrlKey: true, key: "k" })
    );
  }, []);

  return (
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
  );
};

export default NewCalendarPage;
