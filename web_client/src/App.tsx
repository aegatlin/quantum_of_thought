import { useEffect, useState } from "react";
import init, { WasmNotes } from "notes";

function App() {
  const [wasmStatus, setWasmStatus] = useState<string>("Initializing WASM...");

  useEffect(() => {
    async function initWasm() {
      try {
        // Initialize the WASM module
        await init();

        // Create a WasmNotes instance
        const notes = new WasmNotes();
        console.log("✅ Created WasmNotes instance:", notes);

        // Create three notes
        const note1 = notes.create("First note - testing WASM integration");
        console.log("📝 Created note 1:", note1);

        const note2 = notes.create("Second note - this is pretty cool!");
        console.log("📝 Created note 2:", note2);

        const note3 = notes.create("Third note - WASM is working!");
        console.log("📝 Created note 3:", note3);

        // List all notes
        const allNotes = notes.list();
        console.log("📋 List of all notes:", allNotes);

        setWasmStatus("✅ WASM test complete! Check console for results.");
      } catch (error) {
        console.error("Failed to initialize WASM:", error);
        setWasmStatus(`❌ WASM initialization failed: ${error}`);
      }
    }

    initWasm();
  }, []);

  return (
    <div className="min-h-screen bg-background flex items-center justify-center">
      <div className="text-center">
        <h1 className="text-4xl font-bold tracking-tight">
          Quantum of Thought
        </h1>
        <p className="mt-4 text-muted-foreground">{wasmStatus}</p>
      </div>
    </div>
  );
}

export default App;
