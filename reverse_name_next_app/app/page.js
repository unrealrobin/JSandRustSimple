'use client'

import { useState } from 'react';
import axios from 'axios';

export default function Home() {
  
    const [name, setName] = useState("");
    const [reversedName, setReversedName] = useState("");
  
    const handleReverseName = async () => {
      try {
        const response = await axios.get(`http://localhost:8000/reverse/${name}`);
        setReversedName(response.data.reversed_name);
      } catch (error) {
        console.error("Error reversing name:", error);
      }
    };
  
    return (
      <div>
        <input 
          type="text" 
          placeholder="Enter your name" 
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <button onClick={handleReverseName}>Reverse Name</button>
        <p>Reversed Name: {reversedName}</p>
      </div>
    );
}
