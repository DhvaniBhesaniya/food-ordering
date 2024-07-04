import jwt from "jsonwebtoken"

const authMiddleware = async (req,res,next) => {
const {token} = req.headers;
if (!token) {
    return res.json({success:false,message:"not authorized login again"})
}
try {
    const tocken_decode = jwt.verify(token,process.env.JWT_SECRET);
    req.body.userId = tocken_decode.id;
    next();
} catch (error) {
    console.log(error);
    return res.json({success:false,message:"not authorized login again.."})
}

}

export default authMiddleware;