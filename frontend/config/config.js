// config.js

const getBaseUrl = () => {
    let env = "development"; // for local running.
    // let env = "production"; // for production running.
    
    if (env === 'development') {
      return 'http://localhost:4000';
    } else if (env === 'production') {
      return 'https://food-ordering-xaxd.onrender.com';
    } else {
      throw new Error('Invalid environment');
    }
  };
  
  export default getBaseUrl;
  