import { useState } from "react";
import NavBar from "./Navbar";
import { Button } from "./ui/button";
import axios from "axios";
export default function SignUp() {
    // TODO: Use forms and add validation
    const [username, setUsername] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");

    
    const handleSignUp = async(e: React.FormEvent<HTMLFormElement>) => {
        console.log("inside handleSignUp");
        e.preventDefault();
        const response = await axios.post("http://localhost:8080/api/v1/auth/signup", {
            username: username,
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
        <h1>SignUp</h1>
        <form onSubmit={handleSignUp}>
        <input type="text" placeholder="Username" value={username} onChange={(e) => setUsername(e.target.value)}   />
        <input type="email" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)}   />
        <input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)}   />
        <button type="submit" >SignUp</button>
        </form>
      </div>
    </div>
  )
}