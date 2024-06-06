import { createContext, useEffect, useState } from "react";
import { food_list } from "../assets/frontend_assets/assets";

export const StoreContext = createContext(null);
const StoreContextProvider = (props) => {
  const [cartItems, setCartItems] = useState({});
  const addtoCart = (itemId) => {
    if (!cartItems[itemId]) {
      setCartItems((prev) => ({ ...prev, [itemId]: 1 }));
    } else {
      setCartItems((prev) => {
        const updatedCart = { ...prev, [itemId]: prev[itemId] + 1 };
        if (updatedCart[itemId] === 0) {
          delete updatedCart[itemId];
        }
        return updatedCart;
      });
    }
  };
  
  const removeFromCart = (itemId) => {
    setCartItems((prev) => {
      const updatedCart = { ...prev, [itemId]: prev[itemId] - 1 };
      if (updatedCart[itemId] === 0) {
        delete updatedCart[itemId];
      }
      return updatedCart;
    });
  };
  

  useEffect(() => {
    console.log(cartItems);
  }, [cartItems]);
  const contextValue = {
    food_list,
    cartItems,
    setCartItems,
    addtoCart,
    removeFromCart,
  };
  return (
    <StoreContext.Provider value={contextValue}>
      {props.children}
    </StoreContext.Provider>
  );
};

export default StoreContextProvider;
