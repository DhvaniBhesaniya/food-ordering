import React, { useContext, useEffect } from "react";
import "./Verify.css";
import { useNavigate, useSearchParams } from "react-router-dom";
import { StoreContext } from "../../context/StoreContext";
// import axios from "axios";

const Verify = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const success = searchParams.get("success");
  const orderId = searchParams.get("orderId");
  // const { url } = useContext(StoreContext);
  const navigate = useNavigate();

  // http://localhost:5173/verify?success=true&orderId=668bbc1b642f581d7f682319
  // console.log(success,orderId);

  const verifyOrder = async () => {
    try {
      const response = await fetch(`/api/order/verify`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          orderId,
          success,
        })
      });
  
      if (response.ok) {
        const data = await response.json();
        if (data.success) {
          navigate("/myorders");
        } else {
          navigate("/");
        }
      } else {
        console.error('Failed to verify order:', response.statusText);
        // Handle error as needed
      }
    } catch (error) {
      console.error('Error verifying order:', error);
      // Handle network or other errors
    }
  };
  
  useEffect(()=> {
    verifyOrder();
  },[])

  return (
    <div className="verify">
      <div className="spinner"></div>
    </div>
  );
};

export default Verify;
