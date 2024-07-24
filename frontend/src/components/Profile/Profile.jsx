import React, { useContext, useState, useEffect } from "react";
import "./Profile.css";
import SaveButton from "./saveButton/SaveButton";
import CancelButton from "./cancelButton/CancelButton";
import '@fortawesome/fontawesome-free/css/all.min.css';
import { StoreContext } from "../../context/StoreContext";

const Profile = () => {
  const { userData } = useContext(StoreContext);
  const [activeTab, setActiveTab] = useState("general");
  const [profileImage, setProfileImage] = useState(null);
  const [formData, setFormData] = useState({
    customerId: "#" + userData.customerId,
    name: userData.name,
    email: userData.email,
    phoneNumber: "+91 1234567890",
    currentPassword: "",
    newPassword: "",
    profileImg: "",
  });

  useEffect(() => {
    if (userData && userData.profileImg) {
      const imageUrl = `/images/user_pic/${userData.customerId}/${userData.profileImg}`;
      setProfileImage(imageUrl);
    }
  }, [userData]);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData({ ...formData, [name]: value });
  };

  const handleImageChange = (event) => {
    const file = event.target.files[0]; // Get the selected file
    if (file) {
      setProfileImage(URL.createObjectURL(file)); // Display the selected image
      setFormData({ ...formData, profileImg: file }); // Add file to formData
    }
  };

  const updateUserDetails = async () => {
    const data = new FormData(); // Create a new FormData object

    // Append form data
    data.append("name", formData.name);
    data.append("email", formData.email);
    data.append("phoneNumber", formData.phoneNumber);
    data.append("currentPassword", formData.currentPassword);
    data.append("newPassword", formData.newPassword);
    if (formData.profileImg) {
      data.append("profileImg", formData.profileImg); // Append the image file
    }

    try {
      const response = await fetch("/api/user/updateuser", {
        method: "POST",
        headers: {
          token: localStorage.getItem("token"),
        },
        body: data, // Send FormData
      });
      const result = await response.json();
      if (result.success) {
        console.log("User details updated successfully");
      } else {
        alert(result.message);
        console.error("Failed to update user details", result.message);
      }
    } catch (error) {
      console.error("Error updating user details", error);
    }
  };

  const handleTabClick = (tab) => {
    setActiveTab(tab);
  };

  return (
    <div className="container">
      <div className="card">
        <div className="card-body">
          <div className="row-bordered">
            <div className="col-3">
              <div className="profile-image-container">
                <input
                  type="file"
                  id="profile-image-input"
                  accept="image/*"
                  className="profile-image-input"
                  onChange={handleImageChange}
                />
                <label
                  htmlFor="profile-image-input"
                  className="profile-image-label"
                  onClick={(e) => e.preventDefault()} // Prevents the input from opening on image click
                >
                  {profileImage ? (
                    <img
                      src={profileImage}
                      alt="Profile"
                      className="profile-image"
                    />
                  ) : (
                    <div className="profile-image-placeholder">
                      <span>Upload Image</span>
                    </div>
                  )}
                </label>
                  <div
                    className="edit-button"
                    onClick={() =>
                      document.getElementById("profile-image-input").click()
                    }
                  >
                    <i className="fa fa-pencil-alt"></i>
                  </div>
              </div>

              <div className="list-group">
                <a
                  className={`list-group-item ${
                    activeTab === "general" ? "active" : ""
                  }`}
                  onClick={() => handleTabClick("general")}
                >
                  General
                </a>
                <a
                  className={`list-group-item ${
                    activeTab === "changePassword" ? "active" : ""
                  }`}
                  onClick={() => handleTabClick("changePassword")}
                >
                  Change password
                </a>
              </div>
            </div>
            <div className="col-9">
              <div className="tab-content">
                {activeTab === "general" && (
                  <div className="tab-pane active">
                    <div className="card-body pb-2">
                      <div className="form-group">
                        <label className="form-label">CustomerID</label>
                        <input
                          type="text"
                          className="form-control"
                          name="customerId"
                          value={formData.customerId}
                          onChange={handleChange}
                          readOnly
                        />
                      </div>
                      <div className="form-group">
                        <label className="form-label">Username</label>
                        <input
                          type="text"
                          className="form-control"
                          name="name"
                          value={formData.name}
                          onChange={handleChange}
                        />
                      </div>
                      <div className="form-group">
                        <label className="form-label">E-mail</label>
                        <input
                          type="text"
                          className="form-control"
                          name="email"
                          value={formData.email}
                          onChange={handleChange}
                        />
                      </div>
                      <div className="form-group">
                        <label className="form-label">Phone Number</label>
                        <input
                          type="text"
                          className="form-control mb-1"
                          name="phoneNumber"
                          value={formData.phoneNumber}
                          onChange={handleChange}
                        />
                      </div>
                    </div>
                  </div>
                )}
                {activeTab === "changePassword" && (
                  <div className="tab-pane active">
                    <div className="card-body pb-2">
                      <div className="form-group">
                        <label className="form-label">Current password</label>
                        <input
                          type="password"
                          className="form-control"
                          name="currentPassword"
                          value={formData.currentPassword}
                          onChange={handleChange}
                        />
                      </div>
                      <div className="form-group">
                        <label className="form-label">New password</label>
                        <input
                          type="password"
                          className="form-control"
                          name="newPassword"
                          value={formData.newPassword}
                          onChange={handleChange}
                        />
                        <svg
                          viewBox="0 0 576 512"
                          height="1em"
                          xmlns="http://www.w3.org/2000/svg"
                        >
                          {/* <path d="M288 32c-80.8 0-145.5 36.8-192.6 80.6C48.6 156 17.3 208 2.5 243.7c-3.3 7.9-3.3 16.7 0 24.6C17.3 304 48.6 356 95.4 399.4C142.5 443.2 207.2 480 288 480s145.5-36.8 192.6-80.6c46.8-43.5 78.1-95.4 93-131.1c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C433.5 68.8 368.8 32 288 32zM144 256a144 144 0 1 1 288 0 144 144 0 1 1 -288 0zm144-64c0 35.3-28.7 64-64 64c-7.1 0-13.9-1.2-20.3-3.3c-5.5-1.8-11.9 1.6-11.7 7.4c.3 6.9 1.3 13.8 3.2 20.7c13.7 51.2 66.4 81.6 117.6 67.9s81.6-66.4 67.9-117.6c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3z"></path> */}
                        </svg>
                      </div>
                      <div className="form-group">
                        <label className="form-label">
                          Repeat new password (not required now)
                        </label>
                        <input
                          disabled
                          type="password"
                          className="form-control"
                        />
                      </div>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
        <div className="button-container">
          <SaveButton onClick={updateUserDetails} />
          <CancelButton
            onClick={() =>
              setFormData({
                customerId: "#" + userData.customerId,
                name: userData.name,
                email: userData.email,
                phoneNumber: "+91 1234567890",
                currentPassword: "",
                newPassword: "",
              })
            }
          />
        </div>
      </div>
    </div>
  );
};

export default Profile;
