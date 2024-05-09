import { FC, ImgHTMLAttributes } from "react";
import icon from "../../assets/lion-face-icon.svg";

const Logo: FC<ImgHTMLAttributes<HTMLImageElement>> = (props) => {
  return <img height={50} src={icon} {...props} />;
};

export default Logo;
