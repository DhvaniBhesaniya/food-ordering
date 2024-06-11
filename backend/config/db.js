import mongoose from "mongoose";

export const connectDB = async () => {
  await mongoose
    .connect(
      "mongodb+srv://tomato:p1MjFPD5y2fVUVJo@gohan.kns9i2e.mongodb.net/food-delivery?retryWrites=true&w=majority&appName=gohan"
    )
    .then(() => console.log("DB connected"));
};
