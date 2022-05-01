import {ChangeEvent, useState} from 'react'
import logo from './logo.svg'
import './App.css'
import {invoke} from '@tauri-apps/api/tauri'

function App() {
  const [name, setName] = useState<string>("Julián");

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo"/>
        <p>Hello Vite + React! + Rust + Python</p>
        <p>
          <button type="button" onClick={async () => {
            await invoke("hello", {name});
          }}>
            Hey Python! Say Hi to {name}!
          </button>
        </p>
        <input
          onChange={(event: ChangeEvent<HTMLInputElement>) => {
            setName(event.target.value);
          }}
          value={name}
        />
      </header>
    </div>
  )
}

export default App
