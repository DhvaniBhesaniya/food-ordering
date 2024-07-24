import React, { useContext, useState } from "react";
import "./Navbar.css";
import { assets } from "../../assets/frontend_assets/assets";
import { Link, useNavigate } from "react-router-dom";
import { StoreContext } from "../../context/StoreContext";
import "@fortawesome/fontawesome-free/css/all.min.css";

const Navbar = ({ setShowLogin }) => {
  const [menu, setMenu] = useState("menu");
  const [searchOpen, setSearchOpen] = useState(false);
  const { getTotalCartAmount, token, setToken, setCartItems, setUserData } =
    useContext(StoreContext);

  const navigate = useNavigate();
  const logout = () => {
    localStorage.removeItem("token");
    setToken("");
    setCartItems({});
    setUserData({});
    navigate("/");
  };

  return (
    <div className="navbar">
      <Link to={"/"}>
        <img src={assets.logo} alt="Logo" className="logo" />
      </Link>
      <ul className="navbar-menu">
        <Link
          to={"/"}
          onClick={() => setMenu("home")}
          className={menu === "home" ? "active" : ""}
        >
          home
        </Link>
        <a
          href="#explore-menu"
          onClick={() => setMenu("menu")}
          className={menu === "menu" ? "active" : ""}
        >
          menu
        </a>
        <a
          href="#app-download"
          onClick={() => setMenu("mobile-app")}
          className={menu === "mobile-app" ? "active" : ""}
        >
          mobile-app
        </a>
        <a
          href="#footer"
          onClick={() => setMenu("contact-us")}
          className={menu === "contact-us" ? "active" : ""}
        >
          contact us
        </a>
      </ul>
      <div className="navbar-right">
        <div className={`search-container ${searchOpen ? "open" : ""}`}>
          <button
            className="btn-search"
            onClick={() => setSearchOpen(!searchOpen)}
          >
            <i className="fas fa-search"></i>
          </button>
          <input
            type="text"
            className="input-search"
            placeholder="Type to Search..."
          />
        </div>
        <div className="navbar-search-icon">
          <Link to={"/cart"}>
            <img src={assets.basket_icon} alt="Cart" />
          </Link>
          <div className={getTotalCartAmount() === 0 ? "" : "dot"}></div>
        </div>
        {!token ? (
          <button className="sign-in-button" onClick={() => setShowLogin(true)}>sign in</button>
        ) : (
          <div className="navbar-profile">
            <img src={assets.profile_icon} alt="Profile" />
            <ul className="nav-profile-dropdown">
              <Link to={"/profile"}>
                <li>
                  <img src={assets.profile_icon} alt="Profile" /> <p>Profile</p>
                </li>
              </Link>
              <Link to={"/myorders"}>
                <li>
                  <img src={assets.bag_icon} alt="Orders" /> <p>Orders</p>
                </li>
              </Link>
              <hr />
              <li onClick={logout}>
                <img src={assets.logout_icon} alt="Logout" />
                <p>Logout</p>
              </li>
            </ul>
          </div>
        )}
      </div>
    </div>
  );
};

export default Navbar;

// import React, { useContext, useState } from "react";
// import "./Navbar.css";
// import { assets } from "../../assets/frontend_assets/assets";
// import { Link, useNavigate } from "react-router-dom";
// import { StoreContext } from "../../context/StoreContext";

// const Navbar = ({ setShowLogin }) => {
//   const [menu, setMenu] = useState("menu");
//   const { getTotalCartAmount, token, setToken,setCartItems } = useContext(StoreContext);

//   const navigate = useNavigate();
//   const logout = () => {
//     localStorage.removeItem("token");
//     setToken("");
//     setCartItems({});
//     navigate("/");
//   };

//   return (
//     <div className="navbar">
//       <Link to={"/"}>
//         <img src={assets.logo} alt="" className="logo" />
//       </Link>
//       <ul className="navbar-menu">
//         <Link
//           to={"/"}
//           onClick={() => setMenu("home")}
//           className={menu === "home" ? "active" : ""}
//         >
//           home
//         </Link>
//         <a
//           href="#explore-menu"
//           onClick={() => setMenu("menu")}
//           className={menu === "menu" ? "active" : ""}
//         >
//           menu
//         </a>
//         <a
//           href="#app-download"
//           onClick={() => setMenu("mobile-app")}
//           className={menu === "mobile-app" ? "active" : ""}
//         >
//           mobile-app
//         </a>
//         <a
//           href="#footer"
//           onClick={() => setMenu("contact-us")}
//           className={menu === "contact-us" ? "active" : ""}
//         >
//           contact us
//         </a>
//       </ul>
//       <div className="navbar-right">
//         <img src={assets.search_icon} alt="" />
//         <div className="navbar-search-icon">
//           <Link to={"/cart"}>
//             <img src={assets.basket_icon} alt="" />
//           </Link>
//           <div className={getTotalCartAmount() === 0 ? "" : "dot"}></div>
//         </div>
//         {!token ? (
//           <button onClick={() => setShowLogin(true)}>sign in </button>
//         ) : (
//           <div className="navbar-profile">
//             <img src={assets.profile_icon} alt="" />
//             <ul className="nav-profile-dropdown">
//               <Link to={"/profile"}>
//                 <li>
//                   <img src={assets.profile_icon} alt="" /> <p>Profile</p>
//                 </li>
//               </Link>
//               <Link to={"/myorders"}>
//                 <li>
//                   <img src={assets.bag_icon} alt="" /> <p>Orders</p>
//                 </li>
//               </Link>
//               <hr />
//               <li onClick={logout}>
//                 <img src={assets.logout_icon} alt="" />
//                 <p>Logout</p>
//               </li>
//             </ul>
//           </div>
//         )}
//       </div>
//     </div>
//   );
// };

// export default Navbar;
