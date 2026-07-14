import { useEffect, useMemo, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { docsNavigation, getDocArticle, type DocNavNode } from "../state/docsContent";
import "./docsPage.css";

interface DocsPageProps {
  onNavigate: (path: string) => void;
  subpath: string;
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
  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      components={{
        a({ href = "", children }: any) {
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
        code({ children, className }: any) {
          return <code className={className}>{children}</code>;
        },
        pre({ children }: any) {
          const child = Array.isArray(children) ? children[0] : children;
          const props = typeof child === "object" && child && "props" in child ? (child as any).props : {};
          const code = String(props.children ?? "").replace(/\n$/, "");
          const lang = /language-(\w+)/.exec(props.className ?? "")?.[1] ?? "";
          return <CodeBlock code={code} lang={lang} />;
        },
      }}
    >
      {content}
    </ReactMarkdown>
  );
}

function nodeContains(node: DocNavNode, activeSubpath: string): boolean {
  return node.item?.subpath === activeSubpath || node.children.some((child) => nodeContains(child, activeSubpath));
}

function DocsNavNodeView({
  node,
  onNavigate,
  openNodes,
  setOpenNodes,
  subpath,
}: {
  node: DocNavNode;
  onNavigate: (path: string) => void;
  openNodes: Record<string, boolean>;
  setOpenNodes: (updater: (state: Record<string, boolean>) => Record<string, boolean>) => void;
  subpath: string;
}) {
  const open = openNodes[node.key] ?? nodeContains(node, subpath);
  if (node.type === "doc") {
    return (
      <a
        className={node.item?.subpath === subpath ? "active" : ""}
        href={`#/docs/${node.item?.subpath}`}
        onClick={(event) => {
          event.preventDefault();
          if (node.item) onNavigate(`#/docs/${node.item.subpath}`);
        }}
        style={{ "--depth": node.depth } as any}
      >
        {node.title}
      </a>
    );
  }

  return (
    <section className="docs-nav-folder">
      <button className="docs-nav-toggle" onClick={() => setOpenNodes((state) => ({ ...state, [node.key]: !open }))} style={{ "--depth": node.depth } as any} type="button">
        <span>{node.title}</span>
        <span>{open ? "−" : "+"}</span>
      </button>
      {open ? (
        <div className="docs-nav-items">
          {node.children.map((child) => (
            <DocsNavNodeView key={child.key} node={child} onNavigate={onNavigate} openNodes={openNodes} setOpenNodes={setOpenNodes} subpath={subpath} />
          ))}
        </div>
      ) : null}
    </section>
  );
}

export function DocsPage({ subpath, onNavigate }: DocsPageProps) {
  const article = useMemo(() => getDocArticle(subpath), [subpath]);
  const [openNodes, setOpenNodes] = useState<Record<string, boolean>>({});

  useEffect(() => {
    const openActive = (nodes: DocNavNode[], state: Record<string, boolean>) => {
      for (const node of nodes) {
        if (nodeContains(node, subpath)) state[node.key] = true;
        openActive(node.children, state);
      }
    };
    setOpenNodes((state) => {
      const next = { ...state };
      openActive(docsNavigation, next);
      return next;
    });
  }, [subpath]);

  return (
    <div className="docs-page">
      <aside className="docs-sidebar">
        {docsNavigation.map((node) => (
          <DocsNavNodeView key={node.key} node={node} onNavigate={onNavigate} openNodes={openNodes} setOpenNodes={setOpenNodes} subpath={subpath} />
        ))}
      </aside>
      <section className="docs-reading-pane">
        {article ? (
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
      </section>
    </div>
  );
}
