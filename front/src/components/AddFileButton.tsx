import { Add } from "@mui/icons-material";
import { IconButton } from "@mui/material";
import { FC } from "react";

const AddFileButton: FC = () => {
  return (
    <IconButton
      sx={{
        position: "fixed",
        bottom: 50,
        right: 50,
        // backgroundColor: (theme) => theme.palette.primary.main,
      }}
      size="large"
    >
      <Add />
    </IconButton>
  );
};

export default AddFileButton;
