import { useState } from "react";
import { useEffect } from "react";

class TextMessage {
  from;
  time;
  content;
  constructor(from, time, content) {
    this.from = from;
    this.time = time;
    this.content = content;
  }
}

export default function Home() {
  const [allTexts, setAllTexts] = useState([
    new TextMessage(
      "127.0.0.1",
      new Date(),
      (
        <div>
          <p>
            Welcome to DC2.
            <br />
            You are running DC2 version 1.0alpha.
            <br />
            DC2 is a modern, secure alternative to the original DC++, built
            entirely in Rust.
            <pre>
              {`_______    ______    ______  
/       \  /      \  /      \ 
$$$$$$$  |/$$$$$$  |/$$$$$$  |
$$ |  $$ |$$ |  $$/ $$____$$ |
$$ |  $$ |$$ |       /    $$/ 
$$ |  $$ |$$ |   __ /$$$$$$/  
$$ |__$$ |$$ \__/  |$$ |_____ 
$$    $$/ $$    $$/ $$       |
$$$$$$$/   $$$$$$/  $$$$$$$$/ `}
            </pre>
          </p>
        </div>
      )
    ),
  ]);
  const [text, setText] = useState("");

  const handleFormSubmit = (e) => {
    e.preventDefault();
    setAllTexts((old) => [
      ...old,
      new TextMessage("127.0.0.1", new Date(), text),
    ]);
    setText("");
    document.getElementById("input-box").focus();
  };

  useEffect(() => {
    const textDiv = document.getElementById("text-div");
    if (textDiv) {
      textDiv.scrollTop = textDiv.scrollHeight;
    }
  }, [allTexts]);

  return (
    <div className="pt-8 max-h-screen h-screen pb-6 w-screen">
      <div className="grid grid-cols-12 h-full w-full">
        <div className="col-span-2 h-full bg-neutral-600/30 shadow-sm">
          <div className="p-4">
            <h2 className="font-semibold text-gray-50 mb-4 text-lg">
              DC2 Client
            </h2>
            <ul className="space-y-2">
              <li className="flex items-center p-2 rounded-md hover:bg-gray-200/20 cursor-pointer transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-5 w-5 mr-3 text-gray-50"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path d="M5.5 2A3.5 3.5 0 002 5.5v2.879a2.5 2.5 0 00.732 1.767l6.5 6.5a2.5 2.5 0 003.536 0l2.878-2.878a2.5 2.5 0 000-3.536l-6.5-6.5A2.5 2.5 0 007.38 3H5.5zM6 7a1 1 0 100-2 1 1 0 000 2z" />
                </svg>
                <span className="text-gray-50">Hubs</span>
              </li>
              <li className="flex items-center p-2 rounded-md hover:bg-gray-200/20 cursor-pointer transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-5 w-5 mr-3 text-gray-50"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path d="M2 5a2 2 0 012-2h7a2 2 0 012 2v4a2 2 0 01-2 2H9l-3 3v-3H4a2 2 0 01-2-2V5z" />
                  <path d="M15 7v2a4 4 0 01-4 4H9.828l-1.766 1.767c.28.149.599.233.938.233h2l3 3v-3h2a2 2 0 002-2V9a2 2 0 00-2-2h-1z" />
                </svg>
                <span className="text-gray-50">Private Messages</span>
              </li>
              <li className="flex items-center p-2 rounded-md hover:bg-gray-200/20 cursor-pointer transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-5 w-5 mr-3 text-gray-50"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
                    clipRule="evenodd"
                  />
                </svg>
                <span className="text-gray-50">Search</span>
              </li>
              <li className="flex items-center p-2 rounded-md hover:bg-gray-200/20 cursor-pointer transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-5 w-5 mr-3 text-gray-50"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z"
                    clipRule="evenodd"
                  />
                </svg>
                <span className="text-gray-50">Download Queue</span>
              </li>
              <li className="flex items-center p-2 rounded-md hover:bg-gray-200/20 cursor-pointer transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-5 w-5 mr-3 text-gray-50"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM6.293 6.707a1 1 0 010-1.414l3-3a1 1 0 011.414 0l3 3a1 1 0 01-1.414 1.414L11 5.414V13a1 1 0 11-2 0V5.414L7.707 6.707a1 1 0 01-1.414 0z"
                    clipRule="evenodd"
                  />
                </svg>
                <span className="text-gray-50">Finished uploads</span>
              </li>
              <li className="flex items-center p-2 rounded-md hover:bg-gray-200/20 cursor-pointer transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-5 w-5 mr-3 text-gray-50"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                    clipRule="evenodd"
                  />
                </svg>
                <span className="text-gray-50">Finished downloads</span>
              </li>
              <li className="flex items-center p-2 rounded-md hover:bg-gray-200/20 cursor-pointer transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-5 w-5 mr-3 text-gray-50"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                </svg>
                <span className="text-gray-50">Favorite Hubs</span>
              </li>
              <li className="flex items-center p-2 rounded-md hover:bg-gray-200/20 cursor-pointer transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-5 w-5 mr-3 text-gray-50"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z"
                    clipRule="evenodd"
                  />
                </svg>
                <span className="text-gray-50">Favorite Users</span>
              </li>
              <li className="flex items-center p-2 rounded-md hover:bg-gray-200/20 cursor-pointer transition-colors">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-5 w-5 mr-3 text-gray-50"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M12.316 3.051a1 1 0 01.633 1.265l-4 12a1 1 0 11-1.898-.632l4-12a1 1 0 011.265-.633zM5.707 6.293a1 1 0 010 1.414L3.414 10l2.293 2.293a1 1 0 11-1.414 1.414l-3-3a1 1 0 010-1.414l3-3a1 1 0 011.414 0zm8.586 0a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 11-1.414-1.414L16.586 10l-2.293-2.293a1 1 0 010-1.414z"
                    clipRule="evenodd"
                  />
                </svg>
                <span className="text-gray-50">Debug Console</span>
              </li>
            </ul>
          </div>
        </div>
        <div className="col-span-5 h-full">
          <div className="p-4 flex flex-col h-full">
            <div className="flex-grow bg-gray-500/10">
              <div
                id="text-div"
                className="flex flex-col space-y-2 p-4 overflow-y-auto h-full max-h-[85vh]
              [&::-webkit-scrollbar]:w-2
            [&::-webkit-scrollbar-track]:bg-gray-100
            [&::-webkit-scrollbar-thumb]:bg-gray-300
            dark:[&::-webkit-scrollbar-track]:bg-neutral-700
            dark:[&::-webkit-scrollbar-thumb]:bg-neutral-500"
              >
                {allTexts.map((msg, idx) => (
                  <div key={idx} className="rounded-md px-3 py-2 text-gray-100">
                    <div className="text-xs text-gray-400 flex justify-between mb-1">
                      <span>{msg.from}</span>
                      <span>
                        {msg.time instanceof Date
                          ? msg.time.toLocaleTimeString()
                          : msg.time}
                      </span>
                    </div>
                    <div>{msg.content}</div>
                  </div>
                ))}
              </div>
            </div>
            <form onSubmit={handleFormSubmit}>
              <div className="grid grid-cols-8">
                <div className="col-span-7">
                  <input
                    type="text"
                    className="bg-gray-500/30 w-full h-full outline-0 px-2 text-gray-200"
                    placeholder="Type here..."
                    onChange={(e) => setText(e.target.value)}
                    value={text}
                    autoFocus={true}
                    id="input-box"
                  />
                </div>
                <div className="col-span-1">
                  <button
                    type="submit"
                    className="bg-gray-200/20 px-4 py-2 w-full hover:bg-gray-500/30 cursor-pointer"
                  >
                    Send
                  </button>
                </div>
              </div>
            </form>
          </div>
        </div>
        <div className="col-span-5 h-full">
          <div className="p-4 flex h-full">
            <div className="flex-grow">
              <table className="min-w-full rounded-md overflow-hidden cursor-default">
                <thead>
                  <tr>
                    <th className="px-4 py-2 text-left text-gray-100">Nick</th>
                    <th className="px-4 py-2 text-left text-gray-100">Share</th>
                    <th className="px-4 py-2 text-left text-gray-100">
                      Comment
                    </th>
                    <th className="px-4 py-2 text-left text-gray-100">Tag</th>
                    <th className="px-4 py-2 text-left text-gray-100">
                      Connection
                    </th>
                    <th className="px-4 py-2 text-left text-gray-100">IP</th>
                  </tr>
                </thead>
                <tbody>
                  <tr className="hover:bg-black/20">
                    <td className="px-4 py-2 text-gray-200">CoolCat42</td>
                    <td className="px-4 py-2 text-gray-200">1.2 TB</td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200">192.168.1.10</td>
                  </tr>
                  <tr className="hover:bg-black/20">
                    <td className="px-4 py-2 text-gray-200">FileHunter</td>
                    <td className="px-4 py-2 text-gray-200">800 GB</td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200">10.0.0.5</td>
                  </tr>
                  <tr className="hover:bg-black/20">
                    <td className="px-4 py-2 text-gray-200">NightOwl</td>
                    <td className="px-4 py-2 text-gray-200">2.5 TB</td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200">172.16.0.22</td>
                  </tr>
                  <tr className="hover:bg-black/20">
                    <td className="px-4 py-2 text-gray-200">ShareBear</td>
                    <td className="px-4 py-2 text-gray-200">600 GB</td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200">192.168.0.44</td>
                  </tr>
                  <tr className="hover:bg-black/20">
                    <td className="px-4 py-2 text-gray-200">Zenith</td>
                    <td className="px-4 py-2 text-gray-200">3.1 TB</td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200"></td>
                    <td className="px-4 py-2 text-gray-200">203.0.113.7</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
