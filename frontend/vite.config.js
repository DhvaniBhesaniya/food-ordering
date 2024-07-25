import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    build: {
      outDir: 'dist',
    },
    proxy:{
      '/api': {
       target:  'http://localhost:4000',
       secure:false,
      },
      '/image': {
        target: 'http://localhost:4000', // Adjust the target to your image server
        secure: false,
      },
    },
  },
});


// import { defineConfig } from "vite";
// import react from "@vitejs/plugin-react";

// // https://vitejs.dev/config/
// export default defineConfig({
//   plugins: [react()],
//   server: {
//     // host: false,
//     // staticPort: true,
//     // port: 5173,
//     proxy:{
//       '/api': {
//        target:  'http://localhost:4000',
//        secure:false,
//       },
//       '/image': {
//         target: 'http://localhost:4000', // Adjust the target to your image server
//         secure: false,
//       },
//     },
//   },
// });
