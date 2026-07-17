import {
  BookOpen,
  Boxes,
  ChevronDown,
  Database,
  FileText,
  GitBranch,
  Library,
  Network,
  Route,
  Search,
  Sparkles,
  X,
  type LucideIcon,
} from "lucide-react";
import {
  useEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type MouseEvent,
  type RefObject,
} from "react";

import {
  docsNavigation,
  docsSearchEntries,
  getDocSection,
  type DocNavNode,
  type DocSearchEntry,
} from "../../state/docsContent";
import "./docsNavigation.css";

interface DocsNavigationProps {
  closeButtonRef: RefObject<HTMLButtonElement | null>;
  onClose: () => void;
  onNavigate: (path: string) => void;
  subpath: string;
}

const sectionIcons: Record<string, LucideIcon> = {
  core: Network,
  forms: FileText,
  integrations: Boxes,
  "local-truth": GitBranch,
  reference: Library,
  resources: Database,
  router: Route,
  start: Sparkles,
};

function nodeContains(node: DocNavNode, subpath: string): boolean {
  return node.item?.subpath === subpath
    || node.children.some((child) => nodeContains(child, subpath));
}

function descendantCount(node: DocNavNode): number {
  return node.type === "doc"
    ? 1
    : node.children.reduce((total, child) => total + descendantCount(child), 0);
}

function searchScore(entry: DocSearchEntry, query: string): number {
  const title = entry.title.toLocaleLowerCase();
  const path = entry.subpath.toLocaleLowerCase();
  if (title === query) return 0;
  if (title.startsWith(query)) return 1;
  if (title.includes(query)) return 2;
  if (path.includes(query)) return 3;
  return Number.POSITIVE_INFINITY;
}

function searchDocs(query: string): DocSearchEntry[] {
  const normalized = query.trim().toLocaleLowerCase();
  if (!normalized) return [];
  return docsSearchEntries
    .map((entry) => ({ entry, score: searchScore(entry, normalized) }))
    .filter((candidate) => Number.isFinite(candidate.score))
    .sort((left, right) => left.score - right.score || left.entry.title.localeCompare(right.entry.title))
    .slice(0, 32)
    .map((candidate) => candidate.entry);
}

function navigationDepth(depth: number): CSSProperties {
  return { "--depth": depth } as CSSProperties;
}

function DocsLink({
  active,
  depth,
  onNavigate,
  onSelect,
  subpath,
  title,
}: {
  active: boolean;
  depth: number;
  onNavigate: (path: string) => void;
  onSelect: () => void;
  subpath: string;
  title: string;
}) {
  const href = `#/docs/${subpath}`;
  const follow = (event: MouseEvent<HTMLAnchorElement>) => {
    event.preventDefault();
    onNavigate(href);
    onSelect();
  };

  return (
    <a
      aria-current={active ? "page" : undefined}
      className={active ? "docs-nav-link active" : "docs-nav-link"}
      href={href}
      onClick={follow}
      style={navigationDepth(depth)}
    >
      <span>{title}</span>
    </a>
  );
}

function NavigationTreeNode({
  activeSectionId,
  node,
  onNavigate,
  onSelect,
  openFolders,
  setFolderOpen,
  subpath,
}: {
  node: DocNavNode;
  onNavigate: (path: string) => void;
  onSelect: () => void;
  openFolders: Record<string, boolean>;
  activeSectionId: string | null;
  setFolderOpen: (key: string, open: boolean) => void;
  subpath: string;
}) {
  if (node.type === "doc" && node.item) {
    return (
      <DocsLink
        active={node.item.subpath === subpath}
        depth={node.depth}
        onNavigate={onNavigate}
        onSelect={onSelect}
        subpath={node.item.subpath}
        title={node.item.title}
      />
    );
  }

  const open = openFolders[node.key]
    ?? (node.key === activeSectionId || nodeContains(node, subpath));
  const Icon = sectionIcons[node.key] ?? BookOpen;
  return (
    <section className={open ? "docs-nav-section open" : "docs-nav-section"}>
      <button
        aria-expanded={open}
        className="docs-nav-section-toggle"
        onClick={() => setFolderOpen(node.key, !open)}
        type="button"
      >
        <Icon aria-hidden="true" size={15} strokeWidth={1.8} />
        <span>{node.title}</span>
        <small>{descendantCount(node)}</small>
        <ChevronDown aria-hidden="true" className="docs-nav-chevron" size={14} />
      </button>
      {open ? (
        <div className="docs-nav-section-items">
          {node.children.map((child) => (
            <NavigationTreeNode
              key={child.key}
              node={child}
              onNavigate={onNavigate}
              onSelect={onSelect}
              openFolders={openFolders}
              activeSectionId={activeSectionId}
              setFolderOpen={setFolderOpen}
              subpath={subpath}
            />
          ))}
        </div>
      ) : null}
    </section>
  );
}

function SearchResults({
  onNavigate,
  onSelect,
  query,
  results,
  subpath,
}: {
  onNavigate: (path: string) => void;
  onSelect: () => void;
  query: string;
  results: DocSearchEntry[];
  subpath: string;
}) {
  if (results.length === 0) {
    return (
      <div className="docs-search-empty">
        <strong>No page matches “{query.trim()}”.</strong>
        <span>Try the capability, not the exact API spelling.</span>
      </div>
    );
  }

  return (
    <div className="docs-search-results" aria-live="polite">
      <p>{results.length === 32 ? "Top 32 matches" : `${results.length} matches`}</p>
      {results.map((result) => (
        <a
          aria-current={result.subpath === subpath ? "page" : undefined}
          className={result.subpath === subpath ? "docs-search-result active" : "docs-search-result"}
          href={`#/docs/${result.subpath}`}
          key={result.subpath}
          onClick={(event) => {
            event.preventDefault();
            onNavigate(`#/docs/${result.subpath}`);
            onSelect();
          }}
        >
          <span>{result.title}</span>
          <small>{result.sectionTitle}</small>
        </a>
      ))}
    </div>
  );
}

export function DocsNavigation({ closeButtonRef, onClose, onNavigate, subpath }: DocsNavigationProps) {
  const [query, setQuery] = useState("");
  const [openFolders, setOpenFolders] = useState<Record<string, boolean>>({});
  const searchRef = useRef<HTMLInputElement>(null);
  const results = useMemo(() => searchDocs(query), [query]);
  const activeSectionId = getDocSection(subpath)?.id ?? null;

  useEffect(() => {
    const focusSearch = (event: globalThis.KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      const editing = target?.matches("input, textarea, select, [contenteditable='true']") ?? false;
      if ((!editing && event.key === "/") || ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k")) {
        event.preventDefault();
        searchRef.current?.focus();
      }
    };
    window.addEventListener("keydown", focusSearch);
    return () => window.removeEventListener("keydown", focusSearch);
  }, []);

  const setFolderOpen = (key: string, open: boolean) => {
    setOpenFolders((current) => ({ ...current, [key]: open }));
  };

  return (
    <div className="docs-navigation-shell">
      <header className="docs-navigation-header">
        <div>
          <span>Worth Signals</span>
          <h2>Documentation</h2>
        </div>
        <button
          aria-label="Close documentation menu"
          className="docs-menu-close"
          onClick={onClose}
          ref={closeButtonRef}
          type="button"
        >
          <X aria-hidden="true" size={18} />
        </button>
        <p>Start with the easy path. The deeper machinery is here when your problem gets rude.</p>
      </header>

      <label className="docs-search-box">
        <Search aria-hidden="true" size={16} />
        <input
          onChange={(event) => setQuery(event.currentTarget.value)}
          placeholder="Search every public page"
          ref={searchRef}
          type="search"
          value={query}
        />
        {query ? (
          <button aria-label="Clear documentation search" onClick={() => setQuery("")} type="button">
            <X aria-hidden="true" size={14} />
          </button>
        ) : <kbd>/</kbd>}
      </label>

      <nav aria-label="Documentation" className="docs-navigation-tree">
        {query.trim() ? (
          <SearchResults
            onNavigate={onNavigate}
            onSelect={onClose}
            query={query}
            results={results}
            subpath={subpath}
          />
        ) : docsNavigation.map((node) => (
          <NavigationTreeNode
            key={node.key}
            node={node}
            onNavigate={onNavigate}
            onSelect={onClose}
            openFolders={openFolders}
            activeSectionId={activeSectionId}
            setFolderOpen={setFolderOpen}
            subpath={subpath}
          />
        ))}
      </nav>

      <footer className="docs-navigation-footer">
        <span>{docsSearchEntries.length} public pages</span>
        <span><kbd>Ctrl</kbd><kbd>K</kbd> to search</span>
      </footer>
    </div>
  );
}
