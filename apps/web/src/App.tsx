import { useState } from "react";
import createClient from "openapi-fetch";
import type { Health, paths } from "@grow-map/api-client";

const client = createClient<paths>();

function App() {
  const [health, setHealth] = useState<Health | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function checkApi() {
    setLoading(true);
    setHealth(null);
    setError(null);

    try {
      // The path and response type come from generated OpenAPI types.
      const { data, response } = await client.GET("/health/live");

      if (!response.ok) {
        throw new Error(`API returned HTTP ${response.status}`);
      }

      if (!data) {
        throw new Error("API returned an empty response");
      }

      setHealth(data);
    } catch (cause) {
      setError(
        cause instanceof Error
          ? cause.message
          : "Could not connect to the Rust API",
      );
    } finally {
      setLoading(false);
    }
  }

  return (
    <main style={{ padding: 32, fontFamily: "sans-serif" }}>
      <h1>Grow Map — API Contract Check</h1>

      <p>React and Rust use DTOs generated from OpenAPI.</p>

      <button onClick={checkApi} disabled={loading}>
        {loading ? "Checking..." : "Check Rust API"}
      </button>

      <div aria-live="polite">
        {health && (
          <>
            <p>API status: {health.status}</p>
            <p>Service: {health.service}</p>
            <pre>{JSON.stringify(health, null, 2)}</pre>
          </>
        )}

        {error && <p role="alert">{error}</p>}
      </div>
    </main>
  );
}

export default App;
