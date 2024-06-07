import React from "react";
import "./Footer.css";
import { assets } from "../../assets/frontend_assets/assets";
const Footer = () => {
  return (
    <div>
      <div className="footer" id="footer">
        <div className="footer-content">
          <div className="footer-content-left">
            <img src={assets.logo} alt="" />
            <p>
              Lorem ipsum dolor sit amet consectetur adipisicing elit.
              Similique, quos eius. Iste, eius. Voluptates beatae asperiores
              aliquam inventore consequatur quasi corrupti quos ipsam, neque
              autem, reiciendis iure sequi corporis magni.
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
              <li> Delivery</li>
              <li> Privacy policy</li>
            </ul>
          </div>
          <div className="footer-content-right">
            <h2> Get In Touch </h2>
            <ul>
              <li>+91 101-010-1010</li>
              <li>contact@tomato.com</li>
            </ul>
          </div>
        </div>
        <hr/>
        <p className="footer-copyright"> Copywrite 2024 © Tomato.com - All Right Reserved.</p>
      </div>
    </div>
  );
};
export default Footer;