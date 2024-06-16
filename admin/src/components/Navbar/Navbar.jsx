import React from 'react'
import './Navbar.css'
import { assets } from '../../assets/admin_assets/assets'

const Navbar = () => {
  return (
    <div className='navbar'>
    <img src={assets.logo} alt="" className="logo" />
    <img className="profile" src={assets.profile_image} alt=""></img>
      
    </div>
  )
}

export default Navbar
