import {
  Autocomplete,
  AutocompleteChangeDetails,
  AutocompleteChangeReason,
  AutocompleteRenderGetTagProps,
  AutocompleteRenderInputParams,
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

  const handleRenderInput = useCallback(
    (params: AutocompleteRenderInputParams) => (
      <TextField {...params} label={title} size="small" variant="filled" />
    ),
    [title]
  );

  const handleRenderTags = useCallback(
    (value: string[], getTagProps: AutocompleteRenderGetTagProps) =>
      value.map((option: string, index: number) => (
        <Chip
          label={option}
          {...getTagProps({ index })}
          key={option}
          color="primary"
          variant="outlined"
        />
      )),
    []
  );

  return (
    <Autocomplete
      multiple
      autoHighlight
      blurOnSelect
      openOnFocus
      options={options}
      filterSelectedOptions
      renderInput={handleRenderInput}
      renderTags={handleRenderTags}
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
