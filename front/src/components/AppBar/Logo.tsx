import { FC, ImgHTMLAttributes } from "react";
import icon from "../../assets/lion-face-icon.svg";
import { Box } from "@mui/material";

const Logo: FC<ImgHTMLAttributes<HTMLImageElement>> = (props) => {
  return (
    <Box display="flex" justifyContent="center" alignItems="center">
      <img height={28} src={icon} {...props} />
    </Box>
  );
};

export default Logo;
