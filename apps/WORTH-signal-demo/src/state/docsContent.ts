import { publicDocsManifest } from "./publicDocsManifest";

const rawDocs = import.meta.glob([
  "../../../../crates/worth-signal-wasm/docs/**/*.md",
  "!../../../../crates/worth-signal-wasm/docs/app-surface/**/*.md",
  "!../../../../crates/worth-signal-wasm/docs/learn/**/*.md",
  "!../../../../crates/worth-signal-wasm/docs/resource-contracts/**/*.md",
  "!../../../../crates/worth-signal-wasm/docs/**/milestone-crosswalk.md",
  "!../../../../crates/worth-signal-wasm/docs/README.md",
], {
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
  item?: { kind: string; subpath: string; title: string };
  key: string;
  title: string;
  type: "folder" | "doc";
}

export interface DocSearchEntry {
  sectionId: string;
  sectionTitle: string;
  subpath: string;
  title: string;
}

function cleanPath(key: string) {
  return key.replace("../../../../crates/worth-signal-wasm/docs/", "").replace(/\.md$/, "");
}

function titleFromPath(subpath: string, content: string) {
  return content.match(/^#\s+(.+)$/m)?.[1]?.trim() ?? subpath;
}

const allArticles = Object.entries(rawDocs).map(([key, content]) => {
  const subpath = cleanPath(key);
  return { content, subpath, title: titleFromPath(subpath, content) };
});

const articleByPath = new Map(allArticles.map((article) => [article.subpath, article]));
const redirectByPath = new Map(
  publicDocsManifest.redirects.map((redirect) => [redirect.from, redirect.to]),
);

function normalizeDocTarget(currentSubpath: string, href: string): string | null {
  const targetWithoutFragment = href.split("#", 1)[0]?.split("?", 1)[0] ?? "";
  if (
    !targetWithoutFragment
    || /^[a-z]+:/iu.test(targetWithoutFragment)
    || targetWithoutFragment.startsWith("/")
  ) {
    return null;
  }

  const segments = [...currentSubpath.split("/").slice(0, -1)];
  for (const segment of targetWithoutFragment.replace(/\.md$/u, "").split("/")) {
    if (!segment || segment === ".") continue;
    if (segment === "..") segments.pop();
    else segments.push(segment);
  }
  return segments.join("/");
}

function localDocTargets(article: DocArticle): string[] {
  return [...article.content.matchAll(/\[[^\]]+\]\(([^)]+)\)/gu)]
    .map((match) => normalizeDocTarget(article.subpath, match[1]))
    .filter((target): target is string => Boolean(target))
    .filter((target) => articleByPath.has(target) || redirectByPath.has(target));
}

function canonicalPath(subpath: string): string {
  const visited = new Set<string>();
  let current = subpath;
  while (redirectByPath.has(current)) {
    if (visited.has(current)) throw new Error(`Public documentation redirect cycle at ${current}`);
    visited.add(current);
    current = redirectByPath.get(current)!;
  }
  return current;
}

interface PendingPublicArticle {
  path: string;
  sectionId: string;
}

const publicSectionByPath = new Map<string, string>();

function discoverPublicArticles(): Map<string, DocArticle> {
  const discovered = new Map<string, DocArticle>();
  const pending: PendingPublicArticle[] = publicDocsManifest.sections.flatMap((section) =>
    section.items.map((item) => ({ path: item.path, sectionId: section.id }))
  );

  while (pending.length > 0) {
    const request = pending.shift()!;
    const requestedPath = request.path;
    const path = canonicalPath(requestedPath);
    if (discovered.has(path)) continue;
    const article = articleByPath.get(path);
    if (!article) throw new Error(`Public documentation references missing article: ${path}`);
    discovered.set(path, article);
    publicSectionByPath.set(path, request.sectionId);
    pending.push(...localDocTargets(article).map((target) => ({
      path: target,
      sectionId: request.sectionId,
    })));
  }
  return discovered;
}

const publicArticleByPath = discoverPublicArticles();

function buildNavigation(): DocNavNode[] {
  const articlePaths = new Set(publicArticleByPath.keys());
  return publicDocsManifest.sections.map((section) => ({
    children: section.items.map((item) => {
      if (!articlePaths.has(item.path)) {
        throw new Error(`Public documentation manifest references missing article: ${item.path}`);
      }
      return {
        children: [],
        depth: 1,
        item: { kind: item.kind, subpath: item.path, title: item.title },
        key: item.path,
        title: item.title,
        type: "doc" as const,
      };
    }),
    depth: 0,
    key: section.id,
    title: section.title,
    type: "folder" as const,
  }));
}

export function getDocArticle(subpath: string): DocArticle | null {
  const normalized = subpath.replace(/\.md$/, "");
  return publicArticleByPath.get(normalized) ?? null;
}

export function getDocRedirect(subpath: string): string | null {
  const normalized = subpath.replace(/\.md$/, "");
  const target = redirectByPath.get(normalized);
  return target ? canonicalPath(target) : null;
}

export function getDocSection(subpath: string): { id: string; title: string } | null {
  const normalized = canonicalPath(subpath.replace(/\.md$/, ""));
  const sectionId = publicSectionByPath.get(normalized);
  if (!sectionId) return null;
  const section = publicDocsManifest.sections.find((candidate) => candidate.id === sectionId);
  return section ? { id: section.id, title: section.title } : null;
}

export const docsNavigation: DocNavNode[] = buildNavigation();

export const docsSearchEntries: DocSearchEntry[] = [...publicArticleByPath.values()]
  .map((article) => {
    const section = getDocSection(article.subpath);
    return {
      sectionId: section?.id ?? "reference",
      sectionTitle: section?.title ?? "Reference",
      subpath: article.subpath,
      title: article.title,
    };
  })
  .sort((left, right) => left.title.localeCompare(right.title));
