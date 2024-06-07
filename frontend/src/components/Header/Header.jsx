import React from "react";
import Slider from "react-slick";
import "./Header.css";
import "slick-carousel/slick/slick.css"; 
import "slick-carousel/slick/slick-theme.css";
import CustomArrow from "./CustomArrow";
import { assets } from "../../assets/frontend_assets/assets";

const Header = () => {
  const settings = {
    dots: true,  // make this true or false to show the dots or not 
    infinite: true,
    speed: 500,
    slidesToShow: 1,
    slidesToScroll: 1,
    autoplay: true,
    autoplaySpeed: 3000,
    arrows: false, //  make this true or false if the arrows are needed on the sides of the header images.
    // prevArrow: <CustomArrow />,
    // nextArrow: <CustomArrow />
  };

   // Array of objects containing image and content for each slide
   const slides = [
    {
      image: assets.header_img,
      content: (
        <div className="header-contents">
          <h2>Order your favourite food from here</h2>
          <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. eos illum officiis eius ullam neque quasi eligendi ratione iste autem ex.</p>
          <button>View Menu</button>
        </div>
      )
    },
    {
      image: assets.header_img,
      content: null // No content for the second image
    },
    {
      image: assets.header_img,
      content: (
        <div className="header-contents">
          <h2>Special Offers</h2>
          <p>Get discounts on your favorite meals.</p>
          <button>Learn More</button>
        </div>
      )
    },
    // Add more slides if needed
  ];

  return (
    <div className="header">
      <Slider {...settings}>
        {slides.map((slide, index) => (
          <div key={index}>
            <div 
              className="header-slide" 
              style={{ backgroundImage: `url(${slide.image})` }}
            >
              {slide.content}
            </div>
          </div>
        ))}
      </Slider>
    </div>
  );
};


export default Header;
