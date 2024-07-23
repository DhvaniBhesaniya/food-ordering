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
    expiresIn: "24h",
  });
  return token;
};

// register user
const registerUser = async (req, res) => {
  const { name, password, email } = req.body;
  try {
    // Checking if the user already exists
    const exists = await userModel.findOne({ email });
    if (exists) {
      return res.json({ success: false, message: "User already exists" });
    }

    // Validating email format & strong password
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

    // Hashing the user's password
    const salt = await bcrypt.genSalt(10);
    const hashedPassword = await bcrypt.hash(password, salt);

    // Generating a dynamic customer ID
    const lastUser = await userModel.findOne().sort({ createdAt: -1 });
    let customerId;
    if (lastUser && lastUser.customerId) {
      const lastCustomerNumber = parseInt(
        lastUser.customerId.replace("#Customer", "")
      );
      customerId = `#Customer${lastCustomerNumber + 1}`;
    } else {
      customerId = "#Customer1";
    }

    const newUser = new userModel({
      customerId,
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
    res.json({ success: false, message: error.message });
  }
};

const getUserData = async (req, res) => {
  try {
    // Extract token from headers
    const token = req.headers.token;

    // Verify and decode token
    const decodedToken = jwt.verify(token, process.env.JWT_SECRET);

    // Extract user ID from decoded token
    const userId = decodedToken.id;

    // Find user by ID
    const user = await userModel.findById(userId);

    res.json({ success: true, data: user });
  } catch (error) {
    res.json({ success: false, message: error.message });
  }
};

const updateUserData = async (req, res) => {
  const { name, email, phoneNumber, currentPassword, newPassword } = req.body;
  
  try {
    // extracting user id from token
    const token = req.headers.token;
    const decodedToken = jwt.verify(token, process.env.JWT_SECRET);
    const userId = decodedToken.id;

    // Find the user by ID
    const user = await userModel.findById(userId);

    // Check if the current password and the new password are given
    if (currentPassword && newPassword) {
      const isMatch = await bcrypt.compare(currentPassword, user.password);
      if (!isMatch) {
        return res.json({ success: false, message: 'Current password is incorrect' });
      }

      // Hash the new password
      const salt = await bcrypt.genSalt(10);
      const hashedPassword = await bcrypt.hash(newPassword, salt);

      // Update the user with the new password
      user.password = hashedPassword;
    }

    // Update the user details
    if (name) user.name = name;
    if (email) user.email = email;
    // if (phoneNumber) user.phoneNumber = phoneNumber;

    // Save the updated user
    await user.save();

    res.json({ success: true, message: 'User details updated successfully' });
  } catch (error) {
    res.json({ success: false, message: error.message });
  }
};

export { loginUser, registerUser, getUserData, updateUserData };
