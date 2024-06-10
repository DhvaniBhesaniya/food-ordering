const express = require('express');
const cors = require('cors');




// app config
const app = express()
const port = 4000


// middleware
app.use(express.json())
app.use(cors())

app.get("/api_testing",(req,res)=>{
    res.send("API working")
})

app.listen(port,()=>{
    console.log(`server started on http://localhost:${port}`)
})