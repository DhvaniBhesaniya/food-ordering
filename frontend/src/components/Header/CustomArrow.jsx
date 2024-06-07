import React from "react";

const CustomArrow = (props) => {
  const { className, style, onClick } = props;
  return (
    <div
      className={className}
      style={{ ...style, display: "block", background: "black", zIndex: 1 , borderRadius: "50%",}}
      onClick={onClick}
    />
  );
};

export default CustomArrow;
