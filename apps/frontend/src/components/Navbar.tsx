import { Button } from "./ui/button"
export default function Navbar() {
  return (
    <nav className="w-full h-14 px-8 bg-gray-100 flex justify-between items-center">
      <div><h1 className="text-2xl font-bold">zapier-rust</h1></div>
      <div className="flex gap-4 items-center justify-center">
        <a href="/login" className="bg-gray-100 shadow-none text-gray-500 hover:bg-transparent">Login</a>
        <Button className="rounded-full bg-emerald-500">Signup</Button>
      </div>
    </nav>
  )
}