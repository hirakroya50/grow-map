import { useState } from "react";

function App() {
  const [result, setResult] = useState("Not checked yet");
  const [loading, setLoading] = useState(false);

  async function checkApi() {
    setLoading(true);

    try {
      const response = await fetch("/api/health");

      if (!response.ok) {
        throw new Error(`API returned HTTP ${response.status}`);
      }

      const data: unknown = await response.json();
      setResult(JSON.stringify(data, null, 2));
    } catch (error) {
      setResult(
        error instanceof Error ? error.message : "Could not reach the API",
      );
    } finally {
      setLoading(false);
    }
  }

  return (
    <main style={{ padding: 32, fontFamily: "sans-serif" }}>
      <h1>Grow Map</h1>
      <p>React + Rust inside Turborepo</p>

      <button onClick={checkApi} disabled={loading}>
        {loading ? "Checking..." : "Check Rust API"}
      </button>

      <pre aria-live="polite">{result}</pre>
    </main>
  );
}

export default App;
