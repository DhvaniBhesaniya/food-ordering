import React from "react";
import "./CancelButton.css";

const CancelButton = () => {
  return (
    <button className="cancel-button">
      <div className="svgWrapper1">
        <div className="svgWrapper">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            width="30"
            height="30"
            className="icon"
          >
            <line x1="18" y1="6" x2="6" y2="18" stroke="currentColor" stroke-width="2"></line>
            <line x1="6" y1="6" x2="18" y2="18" stroke="currentColor" stroke-width="2"></line>
          </svg>
        </div>
      </div>
      <span>Cancel</span>
    </button>
  );
};

export default CancelButton;












// ----------------------------------------------------------

// import React from "react";
// import "./CancelButton.css";

// const CancelButton = () => {
//   return (
//     <button className="cancel-button">
//       <svg
//         xmlns="http://www.w3.org/2000/svg"
//         width="24"
//         height="24"
//         viewBox="0 0 24 24"
//         className="svg-icon"
//         fill="none"
//         stroke="currentColor"
//         stroke-width="2"
//         stroke-linecap="round"
//         stroke-linejoin="round"
//       >
//         <line x1="18" y1="6" x2="6" y2="18"></line>
//         <line x1="6" y1="6" x2="18" y2="18"></line>
//       </svg>
//       <span className="label">Cancel</span>
//     </button>
//   );
// };

// export default CancelButton;

// cross sign svg
{
  /* <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"  className="svg-icon" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <line x1="18" y1="6" x2="6" y2="18"></line>
  <line x1="6" y1="6" x2="18" y2="18"></line>
</svg> */
}

// calender sign svg
{
  /* <svg xmlns="http://www.w3.org/2000/svg" width="24" viewBox="0 0 24 24" height="24" fill="none" className="svg-icon">
        <g strokeWidth="2" strokeLinecap="round" stroke="#fff">
          <rect y="5" x="4" width="16" rx="2" height="16"></rect>
          <path d="m8 3v4"></path>
          <path d="m16 3v4"></path>
          <path d="m4 11h16"></path>
        </g>
      </svg> */
}
