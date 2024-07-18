import React, { useContext, useEffect, useState } from "react";
import "./FoodItem.css";
import { assets } from "../../assets/frontend_assets/assets";
import { StoreContext } from "../../context/StoreContext";
// import getBaseUrl from "../../../config/config";

const FoodItem = ({ id, name, price, description, image }) => {
  const { cartItems, addtoCart, removeFromCart, inputUpdateCart } =
    useContext(StoreContext);
  const [inputValue, setInputValue] = useState(cartItems[id] || 0);

  useEffect(() => {
    setInputValue(cartItems[id] ? cartItems[id].toString() : '');
  }, [cartItems, id]);

  const handleInputChange = (e) => {
    const newValue = e.target.value;
    setInputValue(newValue);
    
    // if try to empty the cardid number set it to one
    if (newValue === '') {
      inputUpdateCart(id, 1);
    } else {
      const numValue = parseInt(newValue, 10);
      if (!isNaN(numValue) && numValue >= 0) {
        inputUpdateCart(id, numValue);
      }
    }
  };

  // Determine the base URL for images based on environment
  // const baseUrl = getBaseUrl();

  //  console.log(cartItems[id]);

  return (
    <div className="food-item">
      <div className="food-item-img-container">
        <img className="food-item-image" src={`/images/${image}`} alt="" />
        {!cartItems[id] ? (
          <img
            className="add"
            onClick={() => addtoCart(id)}
            src={assets.add_icon_white}
            alt=""
          />
        ) : (
          <div className="food-item-counter">
            <img
              onClick={() => removeFromCart(id)}
              src={assets.remove_icon_red}
              alt=""
            />
            {/* <p>{cartItems[id]}</p> */}
            <input
              type="text"
              value={inputValue}
              onChange={handleInputChange}
            />
            <img
              onClick={() => addtoCart(id)}
              src={assets.add_icon_green}
              alt=""
            />
          </div>
        )}
      </div>
      <div className="food-item-info">
        <div className="food-item-name-rating">
          <p>{name}</p>
          <img src={assets.rating_starts} alt="" />
        </div>
        <p className="food-item-desc">{description}</p>
        <p className="food-item-price">${price}</p>
      </div>
    </div>
  );
};

export default FoodItem;

// import React, { useContext } from "react";
// import "./FoodItem.css";
// import { assets } from "../../assets/frontend_assets/assets";
// import { StoreContext } from "../../context/StoreContext";
// import { Link } from "react-router-dom";

// const FoodItem = ({ id, name, price, description, image }) => {
//   const { cartItems, addtoCart, removeFromCart,url } = useContext(StoreContext);

//   return (
//     <div className="food-item">
//       <div className="food-item-img-container">
//         {/* <img className="food-item-image" src={image} alt="" /> */}
//         <img className="food-item-image" src={url+"/images/"+image} alt="" />
//         {!cartItems[id] ? (
//           <img
//             className="add"
//             onClick={() => addtoCart(id)}
//             src={assets.add_icon_white}
//             alt=""
//           />
//         ) : (
//           <div className="food-item-counter">
//             <img
//               onClick={() => removeFromCart(id)}
//               src={assets.remove_icon_red}
//               alt=""
//             />
//             <p>{cartItems[id]}</p>
//             <img
//               onClick={() => addtoCart(id)}
//               src={assets.add_icon_green}
//               alt=""
//             />
//           </div>
//         )}
//       </div>
//       <div className="food-item-info">
//         <div className="food-item-name-rating">
//           <p>{name}</p>
//           <img src={assets.rating_starts} alt="" />
//         </div>
//         <p className="food-item-desc"> {description}</p>
//         <div className="food-item-price-cart">
//           <p className="food-item-price"> ${price}</p>
//           {cartItems[id] && (
//             <Link to={"/cart"}>
//             <img
//               className="food-item-view-cart"
//               src={assets.basket_icon}
//               alt=""
//             />
//             </Link>
//           )}
//         </div>
//       </div>
//     </div>
//   );
// };

// export default FoodItem;
