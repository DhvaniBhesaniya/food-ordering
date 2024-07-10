import { createContext, useEffect, useState } from "react";
// import { food_list } from "../assets/frontend_assets/assets";

export const StoreContext = createContext(null);

const StoreContextProvider = (props) => {
  const [cartItems, setCartItems] = useState({});
  // console.log(cartItems);
  // use this foodlist to get the data from the database or use the above food_list from assets
  const [food_list, setFoodlist] = useState([]);
  const url = "http://localhost:4000";
  const [token, setToken] = useState("");

  const addtoCart = async (itemId) => {
    if (!cartItems[itemId]) {
      setCartItems((prev) => ({ ...prev, [itemId]: 1 }));
    } else {
      setCartItems((prev) => {
        const updatedCart = { ...prev, [itemId]: prev[itemId] + 1 };
        return updatedCart;
      });
    }
    if (token) {
      await fetch(`${url}/api/cart/add`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'token': token
        },
        body: JSON.stringify({ itemId })
      });
    }
  };

  const removeFromCart = async (itemId) => {
    setCartItems((prev) => {
      const updatedCart = { ...prev, [itemId]: prev[itemId] - 1 };
      if (updatedCart[itemId] === 0) {
        delete updatedCart[itemId];
      }
      return updatedCart;
    });
    if (token) {
      await fetch(`${url}/api/cart/remove`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'token': token
        },
        body: JSON.stringify({ itemId })
      });
    }
  };

  const getTotalCartAmount = () => {
    let totalAmount = 0;
    for (const item in cartItems) {
      let itemInfo = food_list.find((product) => product._id === item);
      totalAmount += itemInfo.price * cartItems[item];
    }
    return totalAmount;
  };

  const fetchFoodList = async () => {
    const response = await fetch(`${url}/api/food/list`);
    const data = await response.json();
    if (data.success) {
      setFoodlist(data.data);
    } else {
      console.error(data.message);
    }
  };

  const loadCartData = async (token) => {
    const response = await fetch(`${url}/api/cart/get`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'token': token
      }
    });
    const data = await response.json();
    setCartItems(data.cartData);
  };

  useEffect(() => {
    async function loadData() {
      await fetchFoodList();

      if (localStorage.getItem("token")) {
        const savedToken = localStorage.getItem("token");
        setToken(savedToken);
        await loadCartData(savedToken);
      }
    }
    loadData();
  }, []);

  const contextValue = {
    food_list,
    cartItems,
    setCartItems,
    addtoCart,
    removeFromCart,
    getTotalCartAmount,
    url,
    token,
    setToken,
  };

  return (
    <StoreContext.Provider value={contextValue}>
      {props.children}
    </StoreContext.Provider>
  );
};

export default StoreContextProvider;




// ----------------------------------------------


// import axios from "axios";
// import { createContext, useEffect, useState } from "react";
// // import { food_list } from "../assets/frontend_assets/assets";

// export const StoreContext = createContext(null);
// const StoreContextProvider = (props) => {
//   const [cartItems, setCartItems] = useState({});
//   // console.log(cartItems);
//   // use this foodlist to get the data from the database or use the above food_list from assets
//   const [food_list, setFoodlist] = useState([]);
//   const url = "http://localhost:4000";
//   const [token, setToken] = useState("");
//   const addtoCart = async (itemId) => {
//     if (!cartItems[itemId]) {
//       setCartItems((prev) => ({ ...prev, [itemId]: 1 }));
//     } else {
//       setCartItems((prev) => {
//         const updatedCart = { ...prev, [itemId]: prev[itemId] + 1 };
//         return updatedCart;
//       });
//     }
//     if (token) {
//       await axios.post(url+"/api/cart/add",{itemId},{headers:{token}})
//     }
//   };

//   const removeFromCart = async (itemId) => {
//     setCartItems((prev) => {
//       const updatedCart = { ...prev, [itemId]: prev[itemId] - 1 };
//       if (updatedCart[itemId] === 0) {
//         delete updatedCart[itemId];
//       }
//       return updatedCart;
//     });
//     if (token) {
//       await axios.post(url+"/api/cart/remove",{itemId},{headers:{token}})
//     }
//   };

//   const getTotalCartAmount = () => {
//     let totalAmount = 0;
//     for (const item in cartItems) {
//       // if (cartItems[item] > 0) {
//       let itemInfo = food_list.find((product) => product._id === item);
//       totalAmount += itemInfo.price * cartItems[item];
//       // }
//     }
//     return totalAmount;
//   };

//   const fetchFoodList = async () => {
//     const response = await axios.get(url+"/api/food/list");
//     // console.log(response);
//     if (response.data.success) {
//       setFoodlist(response.data.data);
//     } else {
//       toast.error(response.data.message);
//     }
//   };

//   const loadCartData  = async (token) => {
//     const response = await axios.post(url+"/api/cart/get",{},{headers:{token}});
//     setCartItems(response.data.cartData);
//   }

//   useEffect(() => {
//     async function loadData() {
//       await fetchFoodList();

//       if (localStorage.getItem("token")) {
//         setToken(localStorage.getItem("token"));
//         await loadCartData(localStorage.getItem("token"));
//       }
//     }
//     loadData();
//   }, []);

//   const contextValue = {
//     food_list,
//     cartItems,
//     setCartItems,
//     addtoCart,
//     removeFromCart,
//     getTotalCartAmount,
//     url,
//     token,
//     setToken,
//   };
//   return (
//     <StoreContext.Provider value={contextValue}>
//       {props.children}
//     </StoreContext.Provider>
//   );
// };

// export default StoreContextProvider;
