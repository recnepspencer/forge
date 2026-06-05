import { useState, useEffect } from "react";

export type RouteState = 
  | { type: "landing" }
  | { type: "docs"; subpath: string }
  | { type: "demo-detail"; demoId: number };

function parseHash(hash: string): RouteState {
  const cleanHash = hash.replace(/^#\/?/, "");

  if (!cleanHash) {
    return { type: "landing" };
  }

  if (cleanHash === "docs") {
    return { type: "docs", subpath: "start_here" };
  }

  if (cleanHash.startsWith("docs/")) {
    const subpath = cleanHash.substring(5).replace(/\.md$/, "");
    return { type: "docs", subpath: subpath || "start_here" };
  }

  if (cleanHash === "demos") {
    return { type: "demo-detail", demoId: 1 };
  }

  if (cleanHash.startsWith("demos/")) {
    const demoIdStr = cleanHash.substring(6);
    const demoId = parseInt(demoIdStr, 10);
    if (!isNaN(demoId)) {
      return { type: "demo-detail", demoId };
    }
  }

  return { type: "landing" };
}

export function useRouter() {
  const [route, setRoute] = useState<RouteState>(parseHash(window.location.hash));

  useEffect(() => {
    const handleHashChange = () => {
      setRoute(parseHash(window.location.hash));
    };

    window.addEventListener("hashchange", handleHashChange);
    return () => window.removeEventListener("hashchange", handleHashChange);
  }, []);

  const navigate = (path: string) => {
    // Ensure path starts with #
    const targetHash = path.startsWith("#") ? path : `#${path}`;
    window.location.hash = targetHash;
  };

  return {
    route,
    navigate,
    currentPath: window.location.hash || "#/",
  };
}
