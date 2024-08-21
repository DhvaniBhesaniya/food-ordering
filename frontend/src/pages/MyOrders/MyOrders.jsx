import React, { useContext, useEffect, useState } from "react";
import "./MyOrders.css";
import { StoreContext } from "../../context/StoreContext";
import { assets } from "../../assets/frontend_assets/assets";

const MyOrders = () => {
  const { token } = useContext(StoreContext);
  const [data, setdata] = useState([]);
  const [historyData, setHistoryData] = useState([]);
  const [activeTab, setActiveTab] = useState('orders');

  const fetchOrders = async () => {
    const response = await fetch("/api/order/userorders", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'token': token
      },
      body: JSON.stringify({})
    });

    if (response.ok) {
      const data = await response.json();
      setdata(data.data);
    } else {
      console.error('Error fetching orders:', response.statusText);
    }
  };

  
  const fetchOrdersHistory = async () => {
    const response = await fetch("/api/order/userordershistory", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'token': token
      },
      body: JSON.stringify({})
    });

    if (response.ok) {
      const data = await response.json();
setHistoryData(data.data);
    } else {
      console.error('Error fetching  history orders:', response.statusText);
    }
  };



  useEffect(() => {
    if (token) {
      fetchOrders();
      fetchOrdersHistory();
    }
  }, [token]);

  const renderOrders = () => (
    <div className="container">
      {data.map((order, index) => (
        <div key={index} className="my-orders-order">
        <img src={assets.parcel_icon} alt="" />
          <p>
            {order.items.map((item, index) => {
              if (index === order.items.length - 1) {
                return item.name + " x " + item.quantity;
              } else {
                return item.name + " x " + item.quantity + ", ";
              }
            })}
          </p>
          <p>${order.amount}.00</p>
                 <p>Items: {order.items.length}</p>
                 <p><span>&#x25cf;</span><b>{order.status}</b></p>
                 <button onClick={fetchOrders}>Trake Order</button>
        </div>
      ))}
    </div>
  );
  const renderHistoryOrders = () => (
    <div className="container">
      {historyData.map((order, index) => (
        <div key={index} className="my-orders-order">
        <img src={assets.parcel_icon} alt="" />
          <p>
            {order.items.map((item, index) => {
              if (index === order.items.length - 1) {
                return item.name + " x " + item.quantity;
              } else {
                return item.name + " x " + item.quantity + ", ";
              }
            })}
          </p>
          <p>${order.amount}.00</p>
                 <p>Items: {order.items.length}</p>
                 <p><span>&#x25cf;</span><b>{order.status}</b></p>
                 <button disabled  className="historybutton">Trake Order</button>
        </div>
      ))}
    </div>
  );

  return (
    <div className="my-orders">
      <div className="tabs">
        <h2 
          className={activeTab === 'orders' ? 'active' : ''} 
          onClick={() => setActiveTab('orders')}
        >
          My Orders
        </h2>
        <h2 
          className={activeTab === 'history' ? 'active' : ''} 
          onClick={() => setActiveTab('history')}
        >
          My History
        </h2>
      </div>
      {activeTab === 'orders' ? renderOrders() : renderHistoryOrders()}
    </div>
  );
};

export default MyOrders;






















// ----------------------------------------------------------------------

// import React, { useContext, useEffect, useState } from "react";
// import "./MyOrders.css";
// import { StoreContext } from "../../context/StoreContext";
// // import axios from "axios";
// import { assets } from "../../assets/frontend_assets/assets";
// const MyOrders = () => {
//   const { token } = useContext(StoreContext);
//   const [data, setdata] = useState([]);

//  const fetchOrders = async () => {
//   const response = await fetch("/api/order/userorders", {
//     method: 'POST',
//     headers: {
//       'Content-Type': 'application/json',
//       'token': token
//     },
//     body: JSON.stringify({})
//   });

//   if (response.ok) {
//     const data = await response.json();
//     setdata(data.data);
//   } else {
//     console.error('Error fetching orders:', response.statusText);
//   }
// };



//   useEffect(() => {
//     if (token) {
//    fetchOrders();
//     }
//   }, [token]);
  
//   return(
//      <div className="my-orders">
//      <h2>My Orders</h2>
//      <div className="container">
//         {data.map((order,index)=>{
//             return(
//                 <div className="my-orders-order" key={index}>
//                 <img src={assets.parcel_icon} alt="" />
//                 <p>{order.items.map((item,index)=>{
//                      if (index === order.items.length-1){
//                         return item.name+ " x "+item.quantity
//                      }
//                      else{
//                         return item.name+ " x "+item.quantity+", "                        
//                      }    
//                 })}</p>
//                 <p>${order.amount}.00</p>
//                 <p>Items: {order.items.length}</p>
//                 <p><span>&#x25cf;</span><b>{order.status}</b></p>
//                 <button onClick={fetchOrders}>Trake Order</button>
//                 </div> 
//             )
//         })}
//      </div>

//      </div>

//     )};

// export default MyOrders;
