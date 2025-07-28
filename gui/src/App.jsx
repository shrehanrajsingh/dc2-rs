import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Navbar from "./navbar";
import { useAppStore } from "./store/useAppStore";
import Home from "./components/Home";
import { envBackendExec } from "./store/envVars";
import Footer from "./footer";
import { check_server_is_running, get_peers } from "./t-api/api";

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  const page = useAppStore((state) => state.page);
  const setPage = useAppStore((state) => state.setPage);

  const compList = {
    home: <Home />,
  };

  // check_server_is_running();
  // get_peers();

  return (
    <main className="container">
      <Navbar />

      {compList[page]}

      <Footer />
    </main>
  );
}

export default App;
