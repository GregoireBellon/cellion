import { ChangeEvent, FC, useCallback, useRef } from "react";
import CalendarSearch from "./CalendarPageDrawer/CalendarSearch";
import { Box, Button, Divider } from "@mui/material";
import { VisuallyHiddenInput } from "../VisuallyHiddenInput";
import { toast } from "react-toastify";
import sdk from "../../utils/sdk";
import { useNavigate } from "react-router-dom";
import { CalendarMonth } from "@mui/icons-material";

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
        <CalendarSearch />
        <Divider
          orientation="vertical"
          variant="middle"
          flexItem
          sx={{ borderWidth: "1px" }}
        />
        <Button
          onClick={handleImportSolutionClick}
          sx={{ textTransform: "none" }}
          startIcon={<CalendarMonth />}
          size="large"
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
