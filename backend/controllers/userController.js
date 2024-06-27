import userModel from "../models/userModel.js";
import jwt from "jsonwebtoken";
import bcrypt from "bcrypt";
import validator from "validator";

// login user

const loginUser = async (req, res) => {
  const { email, password } = req.body;

  try {
    const user = await userModel.findOne({ email });

    if (!user) {
      return res.json({ success: false, message: "User does not exist" });
    }

    const isMatch = await bcrypt.compare(password, user.password);
    if (!isMatch) {
      return res.json({ success: false, message: "Invalid Password" });
    }
    const token = createToken(user._id);
    res.json({ success: true, message: "Login successful", token });
  } catch (error) {
    res.json({ success: false, message: error });
  }
};

const createToken = (id) => {
  const token = jwt.sign({ id }, process.env.JWT_SECRET, {
    expiresIn: "2m",
  });
  return token;
};

// register user
const registerUser = async (req, res) => {
  const { name, password, email } = req.body;
  try {
    // checking if user exist or not.
    const exists = await userModel.findOne({ email });
    if (exists) {
      return res.json({ success: false, message: "User already exists" });
    }
    // validating email format & strong password
    if (!validator.isEmail(email)) {
      return res.json({
        success: false,
        message: "Please enter a valid email.",
      });
    }
    if (password.length < 8) {
      return res.json({
        success: false,
        message: "Password must be at least 8 characters long.",
      });
    }

    // hashing user password

    const salt = await bcrypt.genSalt(10);
    const hashedPassword = await bcrypt.hash(password, salt);

    const newUser = new userModel({
      name,
      password: hashedPassword,
      email,
    });

    const user = await newUser.save();
    const token = createToken(user._id);
    res.json({
      success: true,
      message: "User registered successfully",
      token,
    });
  } catch (error) {
    res.json({ success: false, message: error });
  }
};

export { loginUser, registerUser };
