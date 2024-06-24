import React, { useEffect, useState } from "react";
import Navbar from "./components/Navbar/Navbar";
import { Route, Routes } from "react-router-dom";
import Home from "./pages/Home/Home";
import Cart from "./pages/Cart/Cart";
import PlaceOrder from "./pages/PlaceOrder/PlaceOrder";
import Footer from "./components/Footer/Footer"
import './App.css';
import LoginPopup from "./components/LoginPopup/LoginPopup";
import Loader from "./components/Loader/Loader";

const App = () => {
  const [showLogin, setShowLogin] = useState(false);
  const [isLoading, setIsLoading] = useState(true); // Loading state

  useEffect(() => {
    // Simulate loading for 5 seconds
    const timer = setTimeout(() => {
      setIsLoading(false);
    }, 5000);

    return () => clearTimeout(timer); // Cleanup timer on unmount
  }, []);

  return (
    <>
      {showLogin ? <LoginPopup setShowLogin={setShowLogin} /> : null}
      {isLoading ? (
        <div className="loader-container">
          <Loader />
        </div>
      ) : (
        <><div className="app">
            <Navbar setShowLogin={setShowLogin} />
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/cart" element={<Cart />} />
              <Route path="/order" element={<PlaceOrder />} />
            </Routes>
          </div><Footer /></>
      )}
    </>
  );
};

export default App;




// import React, { useState } from "react";
// import Navbar from "./components/Navbar/Navbar";
// import { Route, Routes } from "react-router-dom";
// import Home from "./pages/Home/Home";
// import Cart from "./pages/Cart/Cart";
// import PlaceOrder from "./pages/PlaceOrder/PlaceOrder";
// import Footer from "./components/Footer/Footer"
// import './App.css';
// import LoginPopup from "./components/LoginPopup/LoginPopup";

// const App = () => {
//   const [showLogin,setShowLogin] = useState(false);
//   return (
//     <>
//     {showLogin?<LoginPopup setShowLogin={setShowLogin}/>:<></>}
//       <div className="app">
//         <Navbar setShowLogin={setShowLogin} />
//         <Routes>
//           <Route path="/" element={<Home />} />
//           <Route path="/cart" element={<Cart />} />
//           <Route path="/order" element={<PlaceOrder />} />
//         </Routes>
//       </div>
//       <Footer />
//     </>
//   );
// };

// export default App;
