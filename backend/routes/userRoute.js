import express from "express";
import authMiddleware from "../middleware/auth.js";
import { updateUserData,getUserData, loginUser, registerUser } from "../controllers/userController.js";
import multer from "multer";
import fs from "fs";
import jwt from "jsonwebtoken";
import userModel from "../models/userModel.js";

const userRouter = express.Router()

const storage = multer.diskStorage({
    destination: (req, file, cb) => {
      const token = req.headers.token;
      const decodedToken = jwt.verify(token, process.env.JWT_SECRET);
      const userId = decodedToken.id;
      userModel.findById(userId).then((user) => {
        const userFolder = `uploads/user_profiles/${user.customerId}`;
        fs.mkdirSync(userFolder, { recursive: true });
        cb(null, userFolder);
      });
    },
    filename: (req, file, cb) => {
      cb(null, file.originalname); // Store the file with the original name
    },
  });

const upload = multer({ storage: storage });

userRouter.post("/register",registerUser)
userRouter.post("/login",loginUser)
userRouter.get("/userdata",authMiddleware,getUserData)
userRouter.post("/updateuser",authMiddleware,upload.single("profileImg"),updateUserData)

export default userRouter;