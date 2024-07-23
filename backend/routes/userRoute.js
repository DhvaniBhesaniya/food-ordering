import express from "express";
import authMiddleware from "../middleware/auth.js";
import { updateUserData,getUserData, loginUser, registerUser } from "../controllers/userController.js";

const userRouter = express.Router()

userRouter.post("/register",registerUser)
userRouter.post("/login",loginUser)
userRouter.get("/userdata",authMiddleware,getUserData)
userRouter.post("/updateuser",authMiddleware,updateUserData)

export default userRouter;