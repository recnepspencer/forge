import { useEffect, useRef, useState } from "react";
import { useRouter } from "./ui/router";
import { Layout } from "./ui/Layout";
import { LandingPage } from "./ui/LandingPage";
import { DocsPage } from "./ui/DocsPage";
import { DemosContainer } from "./ui/Demos";
import { createSignals } from "worth-signals-wasm";
import "./ui/landingShell.css";
import "./ui/landingPage.css";
import "./ui/landingMarketing.css";
import "./ui/landingDemoRoute.css";
import "./ui/worthTheme.css";
import "./ui/landingMobileCarousel.css";

function App() {
  const { route, navigate: navigateRoute } = useRouter();
  const [docsMenuOpen, setDocsMenuOpen] = useState(false);
  const docsLinkRef = useRef<HTMLAnchorElement>(null);

  const navigate = (path: string) => {
    if (!path.startsWith("#/docs")) setDocsMenuOpen(false);
    navigateRoute(path);
  };

  useEffect(() => {
    createSignals({ deployment: "mainThreadCompatibility" })
      .then(() => undefined)
      .catch((err) => console.error("Failed to boot WASM signals", err));
  }, []);

  return (
    <Layout
      currentRoute={route}
      docsLinkRef={docsLinkRef}
      docsMenuOpen={docsMenuOpen}
      onDocsMenuToggle={() => setDocsMenuOpen((open) => !open)}
      onNavigate={navigate}
    >
      {route.type === "landing" && <LandingPage onNavigate={navigate} />}
      {route.type === "docs" && (
        <DocsPage
          menuOpen={docsMenuOpen}
          menuTriggerRef={docsLinkRef}
          onMenuOpenChange={setDocsMenuOpen}
          subpath={route.subpath}
          onNavigate={navigate}
        />
      )}
      {route.type === "demo-detail" && (
        <DemosContainer demoId={route.demoId} onNavigate={navigate} />
      )}
    </Layout>
  );
}

export default App;
