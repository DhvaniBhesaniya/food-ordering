// import React from "react";
// import "./Footer.css";
// import { assets } from "../../assets/frontend_assets/assets";
// const Footer = () => {
//   return (
//     <div>
//       <div className="footer" id="footer">
//         <div className="footer-content">
//           <div className="footer-content-left">
//             <img src={assets.logo} alt="" />
//             <p>
//               Best way to Order,
//               See the menu..
//               Add your food..
//               CheckOut & Enjoy it.   
//             </p>
//             <div className="footer-social-icons">
//               <img src={assets.facebook_icon} alt="" />
//               <img src={assets.twitter_icon} alt="" />
//               <img src={assets.linkedin_icon} alt="" />
//             </div>
//           </div>
//           <div className="footer-content-center">
//             <h2>Company</h2>
//             <ul>
//               <li>Home</li>
//               <li>About Us</li>
//               <li> Delivery</li>
//               <li> Privacy policy</li>
//             </ul>
//           </div>
//           <div className="footer-content-right">
//             <h2> Get In Touch </h2>
//             <ul>
//               <li>+91 101-010-1010</li>
//               <li>contact@tomato.com</li>
//             </ul>
//           </div>
//         </div>
//         <hr/>
//         <p className="footer-copyright"> Copywrite 2024 © Tomato.com - All Right Reserved.</p>
//       </div>
//     </div>
//   );
// };
// export default Footer;


import React from "react";
import "./Footer.css";
import { assets } from "../../assets/frontend_assets/assets";

const Footer = () => {
  return (
    <div className="footer-container">
      <div className="footer-animation">
        <div className="footer-bg-one"></div>
        <div className="footer-bg-two"></div>
      </div>
      <div className="footer" id="footer">
        <div className="footer-content">
          <div className="footer-content-left">
            <img src={assets.logo} alt="" />
            <p>
              Best way to Order,
              See the menu..
              Add your food..
              CheckOut & Enjoy it.
              <br/>   
            </p>
            <div className="footer-social-icons">
              <img src={assets.facebook_icon} alt="" />
              <img src={assets.twitter_icon} alt="" />
              <img src={assets.linkedin_icon} alt="" />
            </div>
          </div>
          <div className="footer-content-center">
            <h2>Company</h2>
            <ul>
              <li>Home</li>
              <li>About Us</li>
              <li>Delivery</li>
              <li>Privacy policy</li>
            </ul>
          </div>
          <div className="footer-content-right">
            <h2>Get In Touch</h2>
            <ul>
              <li>+91 101-010-1010</li>
              <li>contact@tomato.com</li>
              <li><b>Dhvani Bhesaniya</b></li>
            </ul>
          </div>
        </div>
        <hr/>
        <p className="footer-copyright">Copywrite 2024 © Tomato.com - All Right Reserved.</p>
      </div>
    </div>
  );
};

export default Footer;
