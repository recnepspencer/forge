// Import all markdown files under crates/forge-signal-wasm/docs eagerly as raw strings
const rawDocs = import.meta.glob(
  "../../crates/forge-signal-wasm/docs/**/*.md",
  {
    query: "?raw",
    import: "default",
    eager: true,
  }
) as Record<string, string>;

export interface DocArticle {
  title: string;
  subpath: string; // e.g. "forms/getting-started/your-first-form"
  content: string;
}

export interface DocCategory {
  title: string;
  items: { title: string; subpath: string }[];
}

// Helper to clean key paths
// e.g. "../../crates/forge-signal-wasm/docs/forms/index.md" -> "forms/index"
function cleanPath(key: string): string {
  return key
    .replace("../../crates/forge-signal-wasm/docs/", "")
    .replace(/\.md$/, "");
}

// Expose a helper to fetch any document by clean subpath
export function getDocArticle(subpath: string): DocArticle | null {
  const matchKey = Object.keys(rawDocs).find(
    (key) => cleanPath(key) === subpath
  );

  if (!matchKey) return null;

  const content = rawDocs[matchKey];
  
  // Try to parse the first header as the title
  const headerMatch = content.match(/^#\s+(.+)$/m);
  const title = headerMatch ? headerMatch[1].trim() : subpath.split("/").pop() || "Untitled";

  return {
    title,
    subpath,
    content,
  };
}

// Generate structured navigation hierarchy for the docs sidebar
export const docsNavigation: DocCategory[] = [
  {
    title: "Getting Started",
    items: [
      { title: "Start Here", subpath: "start_here" },
      { title: "Readme Overview", subpath: "README" },
    ],
  },
  {
    title: "Core Mechanics",
    items: [
      { title: "Feature Index", subpath: "learn/feature-index" },
      { title: "State Recipes", subpath: "learn/recipes" },
    ],
  },
  {
    title: "Forms Stack",
    items: [
      { title: "Forms Overview", subpath: "forms/index" },
      { title: "Your First Form", subpath: "forms/getting-started/your-first-form" },
      { title: "Form Source Selection", subpath: "forms/getting-started/choosing-a-form-source" },
      { title: "Source vs Draft vs Effective", subpath: "forms/state/source-truth-draft-and-effective-values" },
      { title: "Semantic Dirty State", subpath: "forms/changes/dirty-state" },
      { title: "Validation Engine", subpath: "forms/validation/validation-overview" },
      { title: "Route-Coupled Forms", subpath: "forms/route-coupling/route-authority-handoff" },
    ],
  },
  {
    title: "Routing Primitives",
    items: [
      { title: "Router Overview", subpath: "router/index" },
      { title: "URL Authority", subpath: "router/authority/README" },
      { title: "Route Projection", subpath: "router/projection/README" },
      { title: "Route Admission", subpath: "router/admission/README" },
      { title: "Route History & Story", subpath: "router/history/README" },
      { title: "Route-Coupled Forms", subpath: "router/forms/README" },
    ],
  },
  {
    title: "Server Resources",
    items: [
      { title: "Resource Overview", subpath: "resources/index" },
      { title: "Your First Resource", subpath: "resources/start-here/your-first-resource" },
      { title: "Choose A Resource Shape", subpath: "resources/start-here/choose-a-resource-shape" },
      { title: "Fetching Single Records", subpath: "resources/fetching/fetch-a-single-record" },
      { title: "Fetching Collections", subpath: "resources/fetching/fetch-a-collection" },
      { title: "Updating & Writing", subpath: "resources/updating/write-a-resource" },
      { title: "Branch-Native Effects", subpath: "resources/branch-native-effects" },
    ],
  },
];

// Fallback search to find any unindexed files dynamically
export function getAllSubpaths(): string[] {
  return Object.keys(rawDocs).map(cleanPath);
}
