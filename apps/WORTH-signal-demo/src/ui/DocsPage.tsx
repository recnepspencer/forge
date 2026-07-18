import {
  isValidElement,
  useEffect,
  useMemo,
  useRef,
  useState,
  useSyncExternalStore,
  type ReactNode,
  type RefObject,
} from "react";
import ReactMarkdown, { type Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import { getDocArticle, getDocRedirect, getDocSection } from "../state/docsContent";
import { DocsNavigation } from "./docs-navigation/DocsNavigation";
import "./docsPage.css";

interface DocsPageProps {
  menuOpen: boolean;
  menuTriggerRef: RefObject<HTMLAnchorElement | null>;
  onMenuOpenChange: (open: boolean) => void;
  onNavigate: (path: string) => void;
  subpath: string;
}

const docsMobileQuery = "(max-width: 960px)";

function subscribeToDocsLayout(onChange: () => void): () => void {
  const query = window.matchMedia(docsMobileQuery);
  query.addEventListener("change", onChange);
  return () => query.removeEventListener("change", onChange);
}

function docsLayoutIsMobile(): boolean {
  return window.matchMedia(docsMobileQuery).matches;
}

function normalizeLink(currentSubpath: string, href: string) {
  if (href.startsWith("http://") || href.startsWith("https://") || href.startsWith("#")) return href;
  const parts = currentSubpath.split("/").slice(0, -1);
  let target = href.replace(/\.md$/, "");
  while (target.startsWith("../")) {
    target = target.slice(3);
    parts.pop();
  }
  if (target.startsWith("./")) target = target.slice(2);
  return `#/docs/${parts.length ? `${parts.join("/")}/` : ""}${target}`;
}

function highlightCode(code: string) {
  const pattern = /(".*?"|'.*?'|`.*?`|\b(?:const|let|function|return|if|else|await|async|import|export|from|type|interface|class|new|extends|satisfies)\b|\b\d+(?:\.\d+)?\b|\/\/.*$)/gm;
  return code.split("\n").map((line, lineIndex) => (
    <span className="docs-code-line" key={lineIndex}>
      {line.split(pattern).filter(Boolean).map((part, index) => {
        const className =
          /^\/\//.test(part) ? "comment" :
          /^["'`]/.test(part) ? "string" :
          /^\d/.test(part) ? "number" :
          /^(const|let|function|return|if|else|await|async|import|export|from|type|interface|class|new|extends|satisfies)$/.test(part) ? "keyword" :
          "";
        return className ? <span className={className} key={index}>{part}</span> : <span key={index}>{part}</span>;
      })}
    </span>
  ));
}

function CodeBlock({ code, lang }: { code: string; lang: string }) {
  const [copied, setCopied] = useState(false);
  return (
    <div className="docs-code-block">
      <div className="docs-code-head">
        <span>{lang || "text"}</span>
        <button
          onClick={() => {
            void navigator.clipboard.writeText(code);
            setCopied(true);
            setTimeout(() => setCopied(false), 1600);
          }}
          type="button"
        >
          {copied ? "Copied" : "Copy"}
        </button>
      </div>
      <pre><code>{highlightCode(code)}</code></pre>
    </div>
  );
}

function MarkdownContent({ content, currentSubpath, onNavigate }: { content: string; currentSubpath: string; onNavigate: (path: string) => void }) {
  const components: Components = {
    a({ href = "", children }) {
      const resolvedHref = normalizeLink(currentSubpath, href);
      const internal = resolvedHref.startsWith("#/docs/");
      return (
        <a
          href={resolvedHref}
          onClick={(event) => {
            if (!internal) return;
            event.preventDefault();
            onNavigate(resolvedHref);
          }}
          rel="noreferrer"
          target={internal ? "_self" : "_blank"}
        >
          {children}
        </a>
      );
    },
    code({ children, className }) {
      return <code className={className}>{children}</code>;
    },
    pre({ children }) {
      const child = Array.isArray(children) ? children[0] : children;
      const props = isValidElement<{ children?: ReactNode; className?: string }>(child)
        ? child.props
        : {};
      const code = String(props.children ?? "").replace(/\n$/, "");
      const lang = /language-(\w+)/.exec(props.className ?? "")?.[1] ?? "";
      return <CodeBlock code={code} lang={lang} />;
    },
  };

  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      components={components}
    >
      {content}
    </ReactMarkdown>
  );
}

export function DocsPage({
  menuOpen,
  menuTriggerRef,
  onMenuOpenChange,
  subpath,
  onNavigate,
}: DocsPageProps) {
  const article = useMemo(() => getDocArticle(subpath), [subpath]);
  const redirect = useMemo(() => getDocRedirect(subpath), [subpath]);
  const section = useMemo(() => getDocSection(subpath), [subpath]);
  const mobileLayout = useSyncExternalStore(subscribeToDocsLayout, docsLayoutIsMobile, () => false);
  const closeMenuRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    if (!menuOpen) return undefined;
    const previousOverflow = document.body.style.overflow;
    const menuTrigger = menuTriggerRef.current;
    const closeOnEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape") onMenuOpenChange(false);
    };
    document.body.style.overflow = "hidden";
    closeMenuRef.current?.focus();
    window.addEventListener("keydown", closeOnEscape);
    return () => {
      document.body.style.overflow = previousOverflow;
      window.removeEventListener("keydown", closeOnEscape);
      menuTrigger?.focus();
    };
  }, [menuOpen, menuTriggerRef, onMenuOpenChange]);

  useEffect(() => {
    if (redirect) onNavigate(`#/docs/${redirect}`);
  }, [onNavigate, redirect]);

  return (
    <div className="docs-page">
      {menuOpen ? (
        <button
          aria-label="Close documentation menu"
          className="docs-menu-scrim"
          onClick={() => onMenuOpenChange(false)}
          type="button"
        />
      ) : null}
      <aside
        aria-hidden={mobileLayout && !menuOpen ? true : undefined}
        aria-modal={mobileLayout && menuOpen ? true : undefined}
        className={menuOpen ? "docs-sidebar open" : "docs-sidebar"}
        id="docs-navigation"
        inert={mobileLayout && !menuOpen ? true : undefined}
        role={mobileLayout ? "dialog" : undefined}
      >
        <DocsNavigation
          closeButtonRef={closeMenuRef}
          onClose={() => onMenuOpenChange(false)}
          onNavigate={onNavigate}
          subpath={subpath}
        />
      </aside>
      <section
        aria-hidden={mobileLayout && menuOpen ? true : undefined}
        className="docs-reading-pane"
        inert={mobileLayout && menuOpen ? true : undefined}
      >
        <div className="docs-reading-inner">
          {article ? (
            <div className="docs-article-context" aria-label="Documentation location">
              <span>{section?.title ?? "Documentation"}</span>
              <i aria-hidden="true" />
              <strong>{article.title}</strong>
            </div>
          ) : null}
          {redirect ? (
            <div className="docs-empty">
              <h2>Opening the current guide…</h2>
            </div>
          ) : article ? (
            <article className="docs-article">
              <MarkdownContent content={article.content} currentSubpath={article.subpath} onNavigate={onNavigate} />
            </article>
          ) : (
            <div className="docs-empty">
              <h2>Article not found</h2>
              <p>The docs index could not find that page.</p>
              <button onClick={() => onNavigate("#/docs/start_here")} type="button">Back to start here</button>
            </div>
          )}
        </div>
      </section>
    </div>
  );
}
