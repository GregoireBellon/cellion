import {
  FC,
  SyntheticEvent,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import {
  Autocomplete,
  AutocompleteCloseReason,
  AutocompleteInputChangeReason,
  Box,
  Dialog,
  DialogContent,
  DialogTitle,
  TextField,
} from "@mui/material";
import { ShortSolutionInfo } from "../../types/api";
import sdk from "../../utils/sdk";
import { Search } from "@mui/icons-material";
import { useNavigate } from "react-router-dom";
import { toast } from "react-toastify";

interface Props {
  open: boolean;
  onClose: () => void;
}

const CalendarSearchDialog: FC<Props> = ({ open, onClose }) => {
  const navigate = useNavigate();

  const [solutions, setSolutions] = useState<ShortSolutionInfo[]>([]);
  const [autoCompleteInput, setAutoCompleteInput] = useState<string>("");
  const [autoCompleteValue, setAutoCompleteValue] = useState<
    ShortSolutionInfo | undefined
  >(undefined);
  const [autoCompleteOpen, setAutoCompleteOpen] = useState<boolean>(true);
  const [loading, setLoading] = useState<boolean>(false);

  const autoCompleteRef = useRef<HTMLDivElement>(null);

  const handleClose = useCallback(() => {
    onClose();
    setAutoCompleteOpen(false);
    setAutoCompleteInput("");
    setAutoCompleteValue(undefined);
  }, [onClose]);

  const handleAutoCompleteInputChange = useCallback(
    (
      _: SyntheticEvent<Element, Event>,
      value: string,
      reason: AutocompleteInputChangeReason
    ) => {
      if (reason === "input") {
        setAutoCompleteInput(value);
      }
    },
    []
  );

  const handleOptionClick = useCallback(
    (_: SyntheticEvent<Element, Event>, value: ShortSolutionInfo | string) => {
      if (typeof value !== "string") {
        navigate(`/calendar/${value.id}`);
        handleClose();
      }
    },
    [handleClose, navigate]
  );

  const handelAutoCompleteClose = useCallback(
    (_: SyntheticEvent<Element, Event>, reason: AutocompleteCloseReason) => {
      if (reason === "selectOption" || reason === "escape") {
        handleClose();
      }
    },
    [handleClose]
  );

  const fetchSolutions = useCallback(async () => {
    setLoading(true);
    try {
      const data = await toast.promise(sdk.listSolutions(), {
        error: "Impossible de récupérer les solutions",
      });
      setSolutions(data);
    } catch (err) {
      console.error((err as Error).message);
    }
    setLoading(false);
  }, []);

  useEffect(() => {
    if (open) {
      void fetchSolutions();
    }
  }, [fetchSolutions, open]);

  useEffect(() => {
    // https://github.com/mui/material-ui/issues/33004
    setTimeout(() => {
      if (open && autoCompleteRef.current !== null) {
        // to get focus
        autoCompleteRef.current.click();
      }
    }, 0);
  }, [open]);

  useEffect(() => {
    if (open) {
      setAutoCompleteOpen(true);
    }
  }, [open]);

  return (
    <Dialog
      open={open}
      onClose={handleClose}
      PaperProps={{ sx: { position: "absolute", top: 50 } }}
    >
      <DialogTitle>Rechercher une solution</DialogTitle>
      <DialogContent>
        <Autocomplete
          freeSolo
          disableClearable
          options={solutions}
          fullWidth
          size="medium"
          autoHighlight
          ref={autoCompleteRef}
          sx={{ width: 500 }}
          getOptionLabel={(option) =>
            typeof option === "string" ? "" : option.fileName
          }
          loading={loading}
          open={autoCompleteOpen}
          onClose={handelAutoCompleteClose}
          value={autoCompleteValue}
          onChange={handleOptionClick}
          inputValue={autoCompleteInput}
          onInputChange={handleAutoCompleteInputChange}
          renderInput={(params) => (
            <TextField
              {...params}
              InputProps={{
                ...params.InputProps,
                placeholder: "Rechercher une solution",
                type: "search",
                startAdornment: <Search />,
              }}
            />
          )}
          renderOption={(props, option) => (
            <Box {...props} key={option.id} component="li">
              {option.fileName}
            </Box>
          )}
        />
      </DialogContent>
    </Dialog>
  );
};

export default CalendarSearchDialog;
