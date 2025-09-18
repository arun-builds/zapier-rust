import { useEffect, useState } from "react";
import axios from "axios";
export default function GetZap() {
    const [zaps, setZaps] = useState<any[]>([]);
    useEffect(() => {
        const fetchZap = async () => {
            const response = await axios.get("http://localhost:8080/api/v1/zap/getzap");
            console.log(response);
            setZaps(response.data);
        }
        fetchZap();
    }, []);
  return (
    <div>
      <h1>GetZap</h1>
      <div>
        {zaps.map((zap) => (
          <div key={zap.id}>{zap.id}</div>
        ))}
      </div>
    </div>
  )
}