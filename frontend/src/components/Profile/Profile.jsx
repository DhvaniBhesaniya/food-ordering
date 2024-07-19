import React, { useState } from 'react';
import "./Profile.css";

const Profile = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const handleSubmit = (e) => {
    e.preventDefault();
    // Handle update logic here
    console.log('Update profile', { username, password });
  };

  return (
    <div className="profile-container">
      <div className="profile-card">
        <div className="profile-image">
          <img src="https://via.placeholder.com/150" alt="Profile" />
        </div>
        <h2 className="profile-title">Update Profile</h2>
        <form onSubmit={handleSubmit}>
          <div className="input-group">
            <input 
              type="text" 
              value={username} 
              onChange={(e) => setUsername(e.target.value)}
              required
            />
            <label>Username</label>
          </div>
          <div className="input-group">
            <input 
              type="password" 
              value={password} 
              onChange={(e) => setPassword(e.target.value)}
              required
            />
            <label>Password</label>
          </div>
          <button type="submit" className="update-btn">Update</button>
        </form>
      </div>
    </div>
  );
};

export default Profile;