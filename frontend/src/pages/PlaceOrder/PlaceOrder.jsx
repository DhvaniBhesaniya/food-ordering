import React, { useContext, useEffect } from "react";
import "./PlaceOrder.css";
import { StoreContext } from "../../context/StoreContext";
import { useNavigate } from "react-router-dom";
const PlaceOrder = () => {
  const { getTotalCartAmount } = useContext(StoreContext);
  const navigate = useNavigate();

  useEffect(()=>{
    if (getTotalCartAmount() === 0) {
      navigate("/");
    }

  })

  return (
    <form className="place-order">
      <div className="place-order-left">
        <p className="title">Delivery Information</p>
        <div className="multi-fields">
          <input type="text" placeholder=" First Name " required/>
          <input type="text" placeholder=" Last Name" required/>
        </div>
        <input type="text" placeholder=" email Address" required/>
        <input type="text" placeholder=" Street" required/>
        <div className="multi-fields">
          <input type="text" placeholder=" City " required/>
          <input type="text" placeholder=" State" required/>
        </div>
        <div className="multi-fields">
          <input type="text" placeholder=" Zip Code " required/>
          <input type="text" placeholder=" Country" required/>
        </div>
        <input type="text" placeholder=" phone " required/>
      </div>
      <div className="place-order-right">
        <div className="cart-total">
          <h2>Cart Totals</h2>
          <div>
            <div className="cart-total-details">
              <p>Subtotal</p>
              <p>${getTotalCartAmount()}</p>
            </div>
            <hr />
            <div className="cart-total-details">
              <p>Deliver Fee</p>
              <p>${getTotalCartAmount() === 0 ? 0 : 2}</p>
            </div>
            <hr />
            <div className="cart-total-details">
              <p>Total</p>
              <b>
                ${getTotalCartAmount() === 0 ? 0 : getTotalCartAmount() + 2}
              </b>
            </div>
          </div>
          <button> PROCEED TO PAYMENT</button>
        </div>
      </div>
    </form>
  );
};

export default PlaceOrder;
