import express from "express";
// import cors from "cors";
import { connectDB } from "./config/db.js";
import foodRouter from "./routes/foodRoute.js";
import userRouter from "./routes/userRoute.js";
import dotenv from "dotenv";
import cartRouter from "./routes/cartRoute.js";
import orderRouter from "./routes/orderRoute.js";
import path from 'path';


dotenv.config({path:'.env'}); // use this to use the path of .env if to run the server.js from root folder 
// dotenv.config({ path: '../.env' }); // use this to run from the root folder 
// dotenv.config();   // use this to run from the root folder
const port = process.env.PORT || 4001;

// app config
const app = express();


const __dirname  = path.resolve();


// middleware
app.use(express.json());
// app.use(cors());

// // List of allowed origins
// const allowedOrigins = [
//     'https://food-ordering-xaxd.onrender.com/', // Replace with your actual frontend domain
//     'http://localhost:5174' // Replace with other allowed origin(s)
//   ];
  
//   // Configure CORS to allow multiple origins
//   const corsOptions = {
//     origin: (origin, callback) => {
//       if (!origin || allowedOrigins.includes(origin)) {
//         callback(null, true);
//       } else {
//         callback(new Error('Not allowed by CORS'));
//       }
//     },
//     optionsSuccessStatus: 200
//   };



// db connection
connectDB();

// api endpoint
app.use("/api/food", foodRouter);
app.use("/images", express.static('uploads'));
app.use("/images/user_pic", express.static('uploads/user_profiles'));
app.use("/api/user", userRouter);
app.use("/api/cart", cartRouter);
app.use("/api/order", orderRouter);


// app.use(express.static(path.join(__dirname, '/frontend/dist')));
// app.get('*',(req,res)=>{
//   res.sendFile(path.join(__dirname, 'frontend','dist','index.html'));
// })

app.get("/test", (req, res) => {
    res.send("API working");
});

app.listen(port, () => {
    console.log(`Server started on http://localhost:${port}`);
});
























//----------------------------------------

// import express from "express"
// import cors from "cors"
// import { connectDB } from "./config/db.js"
// import foodRouter from "./routes/foodRoute.js"
// import userRouter from "./routes/userRoute.js"
// import 'dotenv/config'
// import cartRouter from "./routes/cartRoute.js"
// import orderRouter from "./routes/orderRoute.js"





// const port = process.env.PORT || 4001; 


// // app config
// const app = express()
// // const port = 4000


// // middleware
// app.use(express.json())
// app.use(cors())



// // db connection

// connectDB();

// // api endpoint
// app.use("/api/food",foodRouter)
// app.use("/images",express.static('uploads'))
// app.use("/api/user",userRouter)
// app.use("/api/cart",cartRouter)
// app.use("/api/order",orderRouter)


// app.get("/test",(req,res)=>{
//     res.send("API working")
// })

// app.listen(port,()=>{
//     console.log(`server started on http://localhost:${port}`)
// })