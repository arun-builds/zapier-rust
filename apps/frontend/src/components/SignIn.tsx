import { useState } from "react";
import NavBar from "./Navbar";
import { Button } from "./ui/button";
import axios from "axios";
export default function SignIn() {
    // TODO: Use forms and add validation
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");

    
    const handleSignIn = async(e: React.FormEvent<HTMLFormElement>) => {
        console.log("inside handleSignUp");
        e.preventDefault();
        const response = await axios.post("http://localhost:8080/api/v1/auth/login", {
            email: email,
            password: password,
        },{
            headers: {
                "Authorization": `Bearer ${localStorage.getItem("token")}`
            }
        });
        console.log(response);
    }
  return (
    <div className='flex flex-col items-center  h-screen'>
      <NavBar />
      <div className='flex flex-col items-center justify-center'>
        <h1>SignIn</h1>
        <form onSubmit={handleSignIn}>
        <input type="email" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)}   />
        <input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)}   />
        <button type="submit" >SignIn</button>
        </form>
      </div>
    </div>
  )
}