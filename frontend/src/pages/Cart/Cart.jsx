import React, { useContext } from "react";
import "./Cart.css";
import { StoreContext } from "../../context/StoreContext";
import { useNavigate } from "react-router-dom";

const Cart = () => {
  const {
    setCartItems,
    cartItems,
    food_list,
    getTotalCartAmount,
    removeFromCart,
  } = useContext(StoreContext);
  const navigate = useNavigate();
  const handleCheckout = () => {
    if (getTotalCartAmount() > 0) {
      navigate("/order");
    }
  };

  const isCheckoutDisabled = getTotalCartAmount() === 0;

  return (
    <div className="cart">
      <div className="cart-items">
        <div className="cart-item-title">
          <p>Items</p>
          <p>Title</p>
          <p>Price</p>
          <p>Quantity</p>
          <p>Total</p>
          <p>Add/Remove</p>
        </div>
        <br />
        <hr />
        {food_list.map((item, index) => {
          if (cartItems[item._id] > 0) {
            return (
              <div>
                <div className="cart-item-title cart-items-item">
                  <img src={item.image} alt="" />
                  <p>{item.name}</p>
                  <p>${item.price}</p>
                  <p>{cartItems[item._id]}</p>
                  <p>${item.price * cartItems[item._id]}</p>
                  <p>
                    <span
                      onClick={() => removeFromCart(item._id)}
                      className="cross"
                    >
                      x
                    </span>
                    {/* <span
                      onClick={() =>
                        setCartItems((prev) => ({
                          ...prev,
                          [item._id]: prev[item._id] + 1,
                        }))
                      }
                      className="add"
                    >
                      +
                    </span>
                    <span
                      onClick={() =>
                        setCartItems((prev) => ({
                          ...prev,
                          [item._id]: prev[item._id] - 1,
                        }))
                      }
                      className="minus"
                    >
                      -
                    </span> 
                     <span
                      onClick={() =>
                        setCartItems((prev) => ({ ...prev, [item._id]: 0 }))
                      }
                      className="cross"
                    >
                      x
                    </span> */}
                  </p>
                </div>
                <hr />
              </div>
            );
          }
        })}
      </div>
      <div className="cart-bottom">
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
          <button
            onClick={handleCheckout}
            disabled={isCheckoutDisabled}
            className={isCheckoutDisabled ? "disabled-button" : ""}
          >
            PROCEED TO CHECKOUT
          </button>
        </div>
        <div className="cart-promocode">
          <div>
            <p> If you have promo code enter it here.</p>
            <div className="cart-promocode-input">
              <input type="text" placeholder="promo code.." name="" id="" />
              <button>Submit</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Cart;
