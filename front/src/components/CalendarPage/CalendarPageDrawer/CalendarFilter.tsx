import {
  Autocomplete,
  AutocompleteChangeDetails,
  AutocompleteChangeReason,
  Chip,
  TextField,
} from "@mui/material";
import { FC, SyntheticEvent, useCallback } from "react";

interface Props {
  title: string;
  options: string[];
  value: string[];
  onChange: (newValue: string[]) => void;
  onClear: () => void;
}

const CalendarFilter: FC<Props> = ({
  title,
  options,
  value,
  onChange,
  onClear,
}) => {
  const handleAutoCompleteChange = useCallback(
    (
      _: SyntheticEvent<Element, Event>,
      __: string[],
      reason: AutocompleteChangeReason,
      details?: AutocompleteChangeDetails<string>
    ) => {
      if (reason === "clear") {
        onClear();
      } else if (details !== undefined) {
        if (reason === "selectOption") {
          onChange([...value, details.option]);
        } else if (reason === "removeOption") {
          onChange(value.filter((v) => v !== details.option));
        }
      }
    },
    [onClear, onChange, value]
  );

  return (
    <Autocomplete
      multiple
      autoHighlight
      blurOnSelect
      openOnFocus
      options={options}
      filterSelectedOptions
      renderInput={(params) => (
        <TextField {...params} label={title} size="small" />
      )}
      renderTags={(value: string[], getTagProps) =>
        value.map((option: string, index: number) => (
          <Chip
            variant="filled"
            label={option}
            {...getTagProps({ index })}
            key={option}
          />
        ))
      }
      value={value}
      onChange={handleAutoCompleteChange}
      popupIcon={null}
      limitTags={20}
      openText="Ouvrir"
      clearText="Supprimer"
      closeText="Fermer"
      loadingText="Chargement..."
      noOptionsText="Pas d'options"
      getLimitTagsText={() => "Voir plus"}
    />
  );
};

export default CalendarFilter;
