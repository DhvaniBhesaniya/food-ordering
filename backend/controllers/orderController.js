import orderModel from "../models/orderModel.js";
import userModel from "../models/userModel.js";
import Stripe from "stripe";
import dotenv from "dotenv";

dotenv.config({ path: ".env" });
const stripe = new Stripe(process.env.STRIPE_SECRET_KEY);

// placing the order for frontend
const placeOrder = async (req, res) => {
  const frontend_url = "http://localhost:5173";
  // const frontend_url = "https://food-ordering-xaxd.onrender.com";

  try {
    const newOrder = new orderModel({
      userId: req.body.userId,
      items: req.body.items,
      amount: req.body.amount,
      address: req.body.address,
    });
    await newOrder.save();

    await userModel.findByIdAndUpdate(req.body.userId, { cartData: {} });
    const line_items = req.body.items.map((item) => ({
      price_data: {
        currency: "inr",
        product_data: {
          name: item.name,
        },
        unit_amount: item.price * 100 * 80,
      },
      quantity: item.quantity,
    }));
    line_items.push({
      price_data: {
        currency: "inr",
        product_data: {
          name: "Delivery Charge",
        },
        unit_amount: 2 * 100 * 80,
      },
      quantity: 1,
    });

    const session = await stripe.checkout.sessions.create({
      line_items: line_items,
      mode: "payment",
      success_url: `${frontend_url}/verify?success=true&orderId=${newOrder._id}`,
      cancel_url: `${frontend_url}/verify?success=false&orderId=${newOrder._id}`,
    });
    res.json({ success: true, session_url: session.url });
  } catch (error) {
    // console.log(error);
    res.json({ success: false, message: "Error...." });
  }
};

const verifyOrder = async (req, res) => {
  const { orderId, success } = req.body;
  try {
    if (success == "true") {
      await orderModel.findByIdAndUpdate(orderId, { payment: true });
      res.json({ success: true, message: "paid" });
    } else {
      await orderModel.findByIdAndDelete(orderId);
      res.json({ success: false, message: "Payment Failed" });
    }
  } catch (error) {
    console.log(error);
    res.json({ success: false, message: "Payment Failed" });
  }
};

// user order for frontend
const userOrder = async (req, res) => {
  try {
    let userOrders = await orderModel.find({ userId: req.body.userId });
    // only get the orders whose status  is not equal to Delivered
    userOrders = userOrders.filter((order) => order.status !== "Delivered");
    res.json({ success: true, data: userOrders });
  } catch (error) {
    console.log(error);
    res.json({ success: false, message: error });
  }
};

// user order history
const userHistoryOrder = async (req, res) => {
  try {
    const user = await userModel
      .findById(req.body.userId)
      .populate("orderHistory")
      .exec();

    if (!user) {
      return res
        .status(404)
        .json({ success: false, message: "User not found" });
    }

    res.json({ success: true, data: user.orderHistory });
  } catch (error) {
    console.log(error);
    res.status(500).json({ success: false, message: error.message });
  }
};

// Listing orders for admin panal
const listOrders = async (req, res) => {
  try {
    const orders = await orderModel.find({});
    res.json({ success: true, data: orders });
  } catch (error) {
    console.log(error);
    res.json({ success: false, message: "Error" });
  }
};

//api for updating order status from admin side.
const updateStatus = async (req, res) => {
  try {
    const { orderId, status } = req.body;

    // Update the order status
    const updatedOrder = await orderModel.findByIdAndUpdate(orderId, {
      status: status,
    });

    if (!updatedOrder) {
      return res.status(404).json({ message: "Order not found" });
    }

    // If the status is "Delivered", move the order to user's orderHistory
    if (status === "Delivered") {
      const user = await userModel.findById(updatedOrder.userId);

      if (!user) {
        return res.status(404).json({ message: "User not found" });
      }

      // Add the order to user's orderHistory if it doesn't already exist
      if (!user.orderHistory.includes(updatedOrder._id)) {
        user.orderHistory.push(updatedOrder._id);
        await user.save();
      } else {
        console.log("Order already exists in user's orderHistory");
      }

      // // Remove the order from the orderModel
      // await orderModel.findByIdAndDelete(orderId);

      return res
        .status(200)
        .json({ message: "Order delivered and moved to user's history" });
    }

    res.status(200).json(updatedOrder);
  } catch (error) {
    console.error("Error updating order status:", error);
    res.status(500).json({ message: "Internal server error" });
  }
};
// //api for updating order status from admin side.
// const updateStatus = async (req,res) => {
//   try {
//     const order = await orderModel.findByIdAndUpdate(req.body.orderId,{status:req.body.status});
//     res.json({success:true,data:"Status updated..."})
//   } catch (error) {
//     console.log(error);
//     res.json({success:false,message:"Error while updating status..."})
//   }

// }

export {
  placeOrder,
  verifyOrder,
  userOrder,
  userHistoryOrder,
  listOrders,
  updateStatus,
};
