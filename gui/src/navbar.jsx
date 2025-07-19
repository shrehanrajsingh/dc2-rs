import { useAppStore } from "./store/useAppStore";

export default function Navbar() {
  const page = useAppStore((state) => state.page);
  const setPage = useAppStore((state) => state.setPage);

  return (
    <nav className="w-full flex gap-4 py-2 px-4 align-middle items-center border-b-[0.1px] border-b-gray-500">
      <div className="cursor-default">
        <h1 className="text-sm font-bold">DC 2</h1>
      </div>
      <ul className="flex gap-4 text-sm">
        <li
          onClick={() => setPage("home")}
          className={`hover:underline cursor-pointer ${
            page == "home" ? "underline" : ""
          }`}
        >
          Home
        </li>
        <li
          onClick={() => setPage("messages")}
          className={`hover:underline cursor-pointer ${
            page == "messages" ? "underline" : ""
          }`}
        >
          Messages
        </li>
        <li
          onClick={() => setPage("network")}
          className={`hover:underline cursor-pointer ${
            page == "network" ? "underline" : ""
          }`}
        >
          Network
        </li>
        <li
          onClick={() => setPage("settings")}
          className={`hover:underline cursor-pointer ${
            page == "settings" ? "underline" : ""
          }`}
        >
          Settings
        </li>
        <li
          onClick={() => setPage("quit")}
          className={`hover:underline cursor-pointer ${
            page == "quit" ? "underline" : ""
          }`}
        >
          Quit
        </li>
      </ul>
    </nav>
  );
}
