import { ExpandMore } from "@mui/icons-material";
import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Checkbox,
  FormControlLabel,
  FormGroup,
} from "@mui/material";
import { ChangeEvent, FC, useCallback } from "react";

interface Props {
  title: string;
  items: { id: string; label: string; checked: boolean }[];
  onItemChange: (id: string, checked: boolean) => void;
}

const CalendarFilter: FC<Props> = ({ title, items, onItemChange }) => {
  const handleItemChange = useCallback(
    (id: string) => (e: ChangeEvent<HTMLInputElement>) => {
      onItemChange(id, e.target.checked);
    },
    [onItemChange]
  );

  return (
    <Accordion defaultExpanded>
      <AccordionSummary expandIcon={<ExpandMore />}>{title}</AccordionSummary>
      <AccordionDetails>
        <FormGroup>
          {items.map((item) => (
            <FormControlLabel
              key={item.id}
              control={
                <Checkbox
                  checked={item.checked}
                  onChange={handleItemChange(item.id)}
                />
              }
              label={item.label}
            />
          ))}
        </FormGroup>
      </AccordionDetails>
    </Accordion>
  );
};

export default CalendarFilter;
