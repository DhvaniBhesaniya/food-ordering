import mongoose from "mongoose"

const userschema = new mongoose.Schema({
    customerId : {type:String},
    name: {type: String, required: true},
    email: {type: String, required: true, unique: true},
    password: {type: String, required: true},
    cartData: {type: Object, default: {}},
    orderHistory: [{type: mongoose.Schema.Types.ObjectId, ref: 'order'}],
    profileImg:{type:String,default: ""}
}, {minimize: false})

const userModel = mongoose.models.user || mongoose.model("user", userschema);
export default userModel;




// id, username ,email, phone,
// current passwordf , new password