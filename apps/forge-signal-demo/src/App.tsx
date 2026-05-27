import { useEffect, useState } from "react";
import { useRouter } from "./ui/router";
import { Layout } from "./ui/Layout";
import { LandingPage } from "./ui/LandingPage";
import { DocsPage } from "./ui/DocsPage";
import { DemosIndex } from "./ui/DemosIndex";
import { DemosContainer } from "./ui/Demos";
import { createSignals } from "forge-signal-wasm";

function App() {
  const { route, navigate } = useRouter();
  const [isWasmBooted, setIsWasmBooted] = useState(false);

  useEffect(() => {
    createSignals({ deployment: "mainThreadCompatibility" })
      .then(() => setIsWasmBooted(true))
      .catch((err) => console.error("Failed to boot WASM signals", err));
  }, []);

  return (
    <Layout currentRoute={route} onNavigate={navigate} isWasmBooted={isWasmBooted}>
      {route.type === "landing" && <LandingPage onNavigate={navigate} />}
      {route.type === "docs" && <DocsPage subpath={route.subpath} onNavigate={navigate} />}
      {route.type === "demos" && <DemosIndex onNavigate={navigate} />}
      {route.type === "demo-detail" && (
        <DemosContainer demoId={route.demoId} onNavigate={navigate} />
      )}
    </Layout>
  );
}

export default App;
