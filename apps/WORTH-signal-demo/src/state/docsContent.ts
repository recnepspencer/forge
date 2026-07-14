const rawDocs = import.meta.glob("../../../../crates/worth-signal-wasm/docs/**/*.md", {
  eager: true,
  import: "default",
  query: "?raw",
}) as Record<string, string>;

export interface DocArticle {
  content: string;
  subpath: string;
  title: string;
}

export interface DocNavNode {
  children: DocNavNode[];
  depth: number;
  item?: { subpath: string; title: string };
  key: string;
  title: string;
  type: "folder" | "doc";
}

type InternalNavNode = DocNavNode & { childrenMap: Map<string, InternalNavNode> };

const folderTitles: Record<string, string> = {
  "api-reference": "API Reference",
  "app-surface": "App Surface",
  forms: "Forms",
  learn: "Learn",
  resources: "Resources",
  router: "Router",
};

function cleanPath(key: string) {
  return key.replace("../../../../crates/worth-signal-wasm/docs/", "").replace(/\.md$/, "");
}

function titleFromSlug(slug: string) {
  return (folderTitles[slug] ?? slug)
    .replace(/^README$/i, "Overview")
    .replace(/^index$/i, "Overview")
    .replace(/[-_]/g, " ")
    .replace(/\b\w/g, (char) => char.toUpperCase());
}

function titleFromPath(subpath: string, content: string) {
  return content.match(/^#\s+(.+)$/m)?.[1]?.trim() ?? titleFromSlug(subpath.split("/").pop() ?? subpath);
}

const allArticles = Object.entries(rawDocs).map(([key, content]) => {
  const subpath = cleanPath(key);
  return { content, subpath, title: titleFromPath(subpath, content) };
});

function docRank(node: DocNavNode) {
  const leaf = node.item?.subpath.split("/").pop()?.toLowerCase();
  if (node.item?.subpath === "start_here") return -4;
  if (leaf === "readme") return -3;
  if (leaf === "index") return -2;
  return node.type === "folder" ? -1 : 0;
}

function sortTree(nodes: DocNavNode[]): DocNavNode[] {
  return nodes
    .map((node) => ({ ...node, children: sortTree(node.children) }))
    .sort((left, right) => docRank(left) - docRank(right) || left.title.localeCompare(right.title));
}

function rootOrder(node: DocNavNode) {
  const order = ["start_here", "README", "learn", "app-surface", "forms", "router", "resources", "api-reference"];
  const index = order.indexOf(node.key);
  return index === -1 ? 99 : index;
}

function buildNavigation() {
  const roots = new Map<string, InternalNavNode>();
  const folders = new Map<string, InternalNavNode>();
  const ensureFolder = (segments: string[]): InternalNavNode => {
    const key = segments.join("/");
    const existing = folders.get(key);
    if (existing) return existing;
    const node: InternalNavNode = {
      children: [],
      childrenMap: new Map<string, InternalNavNode>(),
      depth: segments.length - 1,
      key,
      title: titleFromSlug(segments[segments.length - 1] ?? key),
      type: "folder",
    };
    folders.set(key, node);
    if (segments.length === 1) {
      roots.set(key, node);
    } else {
      ensureFolder(segments.slice(0, -1)).childrenMap.set(key, node);
    }
    return node;
  };

  for (const article of allArticles) {
    const segments = article.subpath.split("/");
    const docNode: DocNavNode = {
      children: [],
      depth: segments.length - 1,
      item: { subpath: article.subpath, title: article.title },
      key: article.subpath,
      title: article.title,
      type: "doc",
    };
    if (segments.length === 1) {
      roots.set(article.subpath, { ...docNode, childrenMap: new Map() });
      continue;
    }
    ensureFolder(segments.slice(0, -1)).childrenMap.set(article.subpath, { ...docNode, childrenMap: new Map() });
  }

  const materialize = (node: InternalNavNode): DocNavNode => ({
    children: sortTree(Array.from(node.childrenMap.values()).map(materialize)),
    depth: node.depth,
    item: node.item,
    key: node.key,
    title: node.title,
    type: node.type,
  });

  return sortTree(Array.from(roots.values()).map(materialize)).sort(
    (left, right) => rootOrder(left) - rootOrder(right) || left.title.localeCompare(right.title),
  );
}

export function getDocArticle(subpath: string): DocArticle | null {
  const normalized = subpath.replace(/\.md$/, "");
  return allArticles.find((article) => article.subpath === normalized) ?? null;
}

export function getAllSubpaths(): string[] {
  return allArticles.map((article) => article.subpath);
}

export const docsNavigation: DocNavNode[] = buildNavigation();
