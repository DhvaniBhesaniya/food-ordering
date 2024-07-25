import React, { useState, useEffect } from "react";
import "./Orders.css";
import { toast } from "react-toastify";
import { assets } from "../../assets/admin_assets/assets";

const Orders = () => {
  const [orders, setOrders] = useState([]);

  const fetchAllOrders = async () => {
    try {
      const response = await fetch("/api/order/list");
      const data = await response.json();
      if (data.success) {
        setOrders(data.data);
      } else {
        toast.error(data.message);
      }
    } catch (error) {
      console.error("Error fetching orders:", error);
      toast.error("Failed to fetch orders");
    }
  };

  const statusHandler = async (event, orderId) => {
    const newStatus = event.target.value;
    try {
      // Optimistically update the UI
      setOrders((prevOrders) =>
        prevOrders.map((order) =>
          order._id === orderId ? { ...order, status: newStatus } : order
        )
      );

      const response = await fetch("/api/order/status", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          orderId,
          status: newStatus,
        }),
      });
      const data = await response.json();

      if (data.success) {
        toast.success(`Order status successfully changed to ${newStatus}`);
      } else {
        // Revert the optimistic update if the API call fails
        await fetchAllOrders();
        toast.error(data.message || "Failed to update order status");
      }
    } catch (error) {
      console.error("Error updating order status:", error);
      await fetchAllOrders(); // Revert the optimistic update
      toast.error("An error occurred while updating order status");
    }
  };

  useEffect(() => {
    fetchAllOrders();
  }, []); // Remove dependency on orders

  return (
    <div className="order add">
      <h3>Order Page</h3>
      <div className="order-list">
        {orders.map((order, index) => (
          <div key={order._id} className="order-item">
            <img src={assets.parcel_icon} alt="" />
            <div>
              <p className="order-item-food">
                {order.items.map(
                  (item, idx) =>
                    `${item.name} x ${item.quantity}${
                      idx < order.items.length - 1 ? ", " : ""
                    }`
                )}
              </p>
              <p className="order-item-name">
                {`${order.address.firstName} ${order.address.lastName}`}
              </p>
              <div className="order-item-address">
                <p>{`${order.address.street},`}</p>
                <p>
                  {`${order.address.city}, ${order.address.state}, ${order.address.country}, ${order.address.zipCode}`}
                </p>
              </div>
              <p className="order-item-phone">{order.address.phone}</p>
            </div>
            <p>Items: {order.items.length}</p>
            <p>Total: {order.amount}</p>
            <select
              onChange={(event) => statusHandler(event, order._id)}
              value={order.status}
              disabled={order.status === "Delivered"}
            >
              <option value="Food Processing">Food Processing</option>
              <option value="Out for delivery">Out for delivery</option>
              <option value="Delivered">Delivered</option>
            </select>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Orders;
