import React from "react";
import "./ExploreMenu.css";
import { menu_list } from "../../assets/frontend_assets/assets";
// Import Swiper React components
import { Swiper, SwiperSlide } from 'swiper/react';

// Import Swiper styles
import 'swiper/css';
import 'swiper/css/free-mode';
import 'swiper/css/pagination';

// import required modules
import { Autoplay, FreeMode, Pagination } from 'swiper/modules';

const ExploreMenu = ({ category, setCategory }) => {
  return (
    <div className="explore-menu" id="explore-menu">
      <h1>Explore Our Menu</h1>
      <p className="explore-menu-text">
        Click the below image to see its menu.
        Add to cart & Enjoy it.
      </p>
      <div className="explore-menu-list">
        <Swiper
          slidesPerView={5}
          spaceBetween={30}
          freeMode={true}
          pagination={{clickable:true}}
          autoplay={{delay:2000,
          disableOnInteraction:false}}
          modules={[FreeMode, Pagination,Autoplay]}
          className="mySwiper"
          breakpoints={{
             // When window width is >= 768px
             1080: {
              slidesPerView: 6,
              spaceBetween: 30,
            },
            // When window width is >= 768px
            768: {
              slidesPerView: 4,
              spaceBetween: 30,
            },
            // When window width is < 768px
            0: {
              slidesPerView: 2,
              spaceBetween: 15,
            },
          }}
        >
          {menu_list.map((item, index) => (
            <SwiperSlide key={index}>
              <div
                onClick={() => setCategory(prev => prev === item.menu_name ? "All" : item.menu_name)}
                className="explore-menu-list-item"
              >
                <img
                  className={category === item.menu_name ? "active" : ""}
                  src={item.menu_image}
                  alt={item.menu_name}
                />
                <p>{item.menu_name}</p>
              </div>
            </SwiperSlide>
          ))}
        </Swiper>
      </div>
      <hr />
    </div>
  );
};

export default ExploreMenu;
