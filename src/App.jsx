import Header from "./components/Header";
import Content from "./components/Content";

function App() {
  return (
    <main
      data-tauri-drag-region
      className="bg-base text-primary dark:bg-primary dark:text-base select-none font-semibold w-screen h-screen flex flex-col py-4 cursor-default"
    >
      <Header data-tauri-drag-region />
      <Content data-tauri-drag-region />
    </main>
  );
}

export default App;
