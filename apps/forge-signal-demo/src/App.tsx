import { useEffect } from "react";
import { useRouter } from "./ui/router";
import { Layout } from "./ui/Layout";
import { LandingPage } from "./ui/LandingPage";
import { DocsPage } from "./ui/DocsPage";
import { DemosContainer } from "./ui/Demos";
import { createSignals } from "forge-signal-wasm";
import "./ui/landingShell.css";
import "./ui/landingPage.css";
import "./ui/landingMarketing.css";
import "./ui/landingDemoRoute.css";

function App() {
  const { route, navigate } = useRouter();

  useEffect(() => {
    createSignals({ deployment: "mainThreadCompatibility" })
      .then(() => undefined)
      .catch((err) => console.error("Failed to boot WASM signals", err));
  }, []);

  return (
    <Layout currentRoute={route} onNavigate={navigate}>
      {route.type === "landing" && <LandingPage onNavigate={navigate} />}
      {route.type === "docs" && <DocsPage subpath={route.subpath} onNavigate={navigate} />}
      {route.type === "demo-detail" && (
        <DemosContainer demoId={route.demoId} onNavigate={navigate} />
      )}
    </Layout>
  );
}

export default App;
