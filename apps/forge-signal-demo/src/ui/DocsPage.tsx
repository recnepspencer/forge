import React, { useMemo } from "react";
import { docsNavigation, getDocArticle } from "../state/docsContent";

interface DocsPageProps {
  subpath: string;
  onNavigate: (path: string) => void;
}

// Map subpaths to related demo IDs to show contextual CTA boxes
function getRelatedDemoId(subpath: string): number | null {
  if (subpath.includes("learn/recipes") || subpath.includes("feature-index")) return 1; // Signals
  if (subpath.includes("forms/getting-started/your-first-form") || subpath.includes("forms/index")) return 2; // Form
  if (subpath.includes("router/")) return 3; // Router
  if (subpath.includes("resources/start-here/your-first-resource") || subpath.includes("resources/index")) return 4; // Resource
  if (subpath.includes("forms/route-coupling") || subpath.includes("router/forms")) return 5; // Coupled
  if (subpath.includes("branch-native-effects") || subpath.includes("effects")) return 6; // History
  return null;
}

export const DocsPage: React.FC<DocsPageProps> = ({ subpath, onNavigate }) => {
  const article = useMemo(() => getDocArticle(subpath), [subpath]);

  // Clean relative markdown paths to hash paths
  // e.g. link "./state/source-truth.md" from "forms/index" -> "#/docs/forms/state/source-truth"
  const resolveMarkdownLink = (href: string): string => {
    if (href.startsWith("http://") || href.startsWith("https://") || href.startsWith("#")) {
      return href;
    }
    
    // Clean up relative markers
    let currentDir = subpath.split("/").slice(0, -1).join("/");
    let target = href.replace(/\.md$/, "");
    
    if (target.startsWith("./")) {
      target = target.substring(2);
    } else if (target.startsWith("../")) {
      const parts = currentDir.split("/");
      while (target.startsWith("../") && parts.length > 0) {
        target = target.substring(3);
        parts.pop();
      }
      currentDir = parts.join("/");
    }
    
    const resolvedSubpath = currentDir ? `${currentDir}/${target}` : target;
    return `#/docs/${resolvedSubpath}`;
  };

  // Custom Markdown parser to output styled HTML elements directly
  const renderMarkdown = (md: string) => {
    const lines = md.split("\n");
    const elements: React.ReactNode[] = [];
    let inCodeBlock = false;
    let codeContent: string[] = [];
    let codeLang = "";
    let listItems: string[] = [];
    let keyCounter = 0;

    const flushList = () => {
      if (listItems.length > 0) {
        elements.push(
          <ul key={`ul-${keyCounter++}`} style={{ margin: "0 0 1.25rem 1.5rem", color: "var(--text-secondary)" }}>
            {listItems.map((item, idx) => (
              <li key={idx} style={{ marginBottom: "0.4rem" }}>
                {parseInlineFormatting(item)}
              </li>
            ))}
          </ul>
        );
        listItems = [];
      }
    };

    const parseInlineFormatting = (text: string): React.ReactNode[] => {
      // Very basic parser for **bold**, *italic*, `code`, and [links]
      const parts: React.ReactNode[] = [];
      let i = 0;
      let buffer = "";

      while (i < text.length) {
        // Bold **text**
        if (text.startsWith("**", i)) {
          if (buffer) { parts.push(buffer); buffer = ""; }
          const endIdx = text.indexOf("**", i + 2);
          if (endIdx !== -1) {
            parts.push(<strong key={i} style={{ color: "var(--text-primary)", fontWeight: 600 }}>{text.substring(i + 2, endIdx)}</strong>);
            i = endIdx + 2;
            continue;
          }
        }
        
        // Inline code `text`
        if (text[i] === "`") {
          if (buffer) { parts.push(buffer); buffer = ""; }
          const endIdx = text.indexOf("`", i + 1);
          if (endIdx !== -1) {
            parts.push(
              <code
                key={i}
                style={{
                  background: "var(--bg-tertiary)",
                  color: "var(--accent-signals)",
                  padding: "0.15rem 0.35rem",
                  borderRadius: "4px",
                  fontSize: "0.85em",
                }}
              >
                {text.substring(i + 1, endIdx)}
              </code>
            );
            i = endIdx + 1;
            continue;
          }
        }

        // Markdown Link [text](url)
        if (text[i] === "[") {
          const closeBracketIdx = text.indexOf("]", i);
          if (closeBracketIdx !== -1) {
            const openParenIdx = text.indexOf("(", closeBracketIdx);
            if (openParenIdx === closeBracketIdx + 1) {
              const closeParenIdx = text.indexOf(")", openParenIdx);
              if (closeParenIdx !== -1) {
                if (buffer) { parts.push(buffer); buffer = ""; }
                const linkText = text.substring(i + 1, closeBracketIdx);
                const linkHref = text.substring(openParenIdx + 1, closeParenIdx);
                const isHashLink = linkHref.startsWith(".") || !linkHref.startsWith("http");
                
                parts.push(
                  <a
                    key={i}
                    href={isHashLink ? resolveMarkdownLink(linkHref) : linkHref}
                    onClick={(e) => {
                      if (isHashLink) {
                        e.preventDefault();
                        onNavigate(resolveMarkdownLink(linkHref));
                      }
                    }}
                    style={{
                      color: "var(--accent-signals)",
                      textDecoration: "underline",
                      fontWeight: 500,
                    }}
                    target={isHashLink ? "_self" : "_blank"}
                    rel="noreferrer"
                  >
                    {linkText}
                  </a>
                );
                i = closeParenIdx + 1;
                continue;
              }
            }
          }
        }

        buffer += text[i];
        i++;
      }

      if (buffer) parts.push(buffer);
      return parts;
    };

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];

      // Code blocks ```
      if (line.startsWith("```")) {
        if (inCodeBlock) {
          // Flush code block
          const finalCode = codeContent.join("\n");
          elements.push(<CodeBlock key={`code-${keyCounter++}`} code={finalCode} lang={codeLang} />);
          codeContent = [];
          inCodeBlock = false;
        } else {
          flushList();
          inCodeBlock = true;
          codeLang = line.substring(3).trim();
        }
        continue;
      }

      if (inCodeBlock) {
        codeContent.push(line);
        continue;
      }

      // Headers #, ##, ###
      if (line.startsWith("#")) {
        flushList();
        const level = line.match(/^#+/)?.[0].length || 1;
        const text = line.replace(/^#+\s+/, "");
        
        if (level === 1) {
          elements.push(
            <h1 key={`h1-${keyCounter++}`} style={{ fontSize: "2rem", fontWeight: 700, margin: "2rem 0 1rem 0", color: "var(--text-primary)" }}>
              {parseInlineFormatting(text)}
            </h1>
          );
        } else if (level === 2) {
          elements.push(
            <h2 key={`h2-${keyCounter++}`} style={{ fontSize: "1.45rem", fontWeight: 600, margin: "2rem 0 1rem 0", paddingBottom: "0.5rem", borderBottom: "1px solid var(--border-light)", color: "var(--text-primary)" }}>
              {parseInlineFormatting(text)}
            </h2>
          );
        } else {
          elements.push(
            <h3 key={`h3-${keyCounter++}`} style={{ fontSize: "1.15rem", fontWeight: 600, margin: "1.5rem 0 0.75rem 0", color: "var(--text-primary)" }}>
              {parseInlineFormatting(text)}
            </h3>
          );
        }
        continue;
      }

      // Lists
      if (line.trim().startsWith("- ") || line.trim().startsWith("* ")) {
        const itemText = line.replace(/^\s*[-*]\s+/, "");
        listItems.push(itemText);
        continue;
      }

      // Quote / Alert
      if (line.startsWith("> ")) {
        flushList();
        const text = line.substring(2);
        elements.push(
          <blockquote
            key={`quote-${keyCounter++}`}
            style={{
              padding: "1rem 1.25rem",
              background: "rgba(6, 182, 212, 0.03)",
              borderLeft: "4px solid var(--accent-signals)",
              borderRadius: "0 8px 8px 0",
              margin: "0 0 1.25rem 0",
              fontSize: "0.95rem",
              lineHeight: "1.6",
              color: "var(--text-secondary)",
            }}
          >
            {parseInlineFormatting(text)}
          </blockquote>
        );
        continue;
      }

      // Paragraph / Spacer
      if (line.trim() === "") {
        flushList();
        continue;
      }

      // Default text paragraph
      flushList();
      elements.push(
        <p key={`p-${keyCounter++}`} style={{ margin: "0 0 1.25rem 0", lineHeight: "1.7", color: "var(--text-secondary)" }}>
          {parseInlineFormatting(line)}
        </p>
      );
    }

    flushList();
    return elements;
  };

  const relatedDemoId = article ? getRelatedDemoId(article.subpath) : null;

  return (
    <div className="docs-container" style={{ display: "flex", flex: 1, minHeight: "calc(100vh - 72px)" }}>
      {/* Sidebar Navigation */}
      <aside
        className="docs-sidebar"
        style={{
          width: "280px",
          borderRight: "1px solid var(--border-light)",
          background: "var(--bg-primary)",
          padding: "2rem 1.5rem",
          display: "flex",
          flexDirection: "column",
          gap: "1.5rem",
          overflowY: "auto",
          maxHeight: "calc(100vh - 72px)",
          position: "sticky",
          top: "72px",
          zIndex: 10,
        }}
      >
        {docsNavigation.map((cat, catIdx) => (
          <div key={catIdx} style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
            <span
              style={{
                fontSize: "0.75rem",
                fontWeight: 700,
                color: "var(--text-muted)",
                textTransform: "uppercase",
                letterSpacing: "1px",
                marginBottom: "0.25rem",
              }}
            >
              {cat.title}
            </span>
            {cat.items.map((item, itemIdx) => (
              <a
                key={itemIdx}
                href={`#/docs/${item.subpath}`}
                onClick={(e) => {
                  e.preventDefault();
                  onNavigate(`#/docs/${item.subpath}`);
                }}
                style={{
                  fontSize: "0.9rem",
                  color: subpath === item.subpath ? "var(--text-primary)" : "var(--text-secondary)",
                  fontWeight: subpath === item.subpath ? 600 : 400,
                  padding: "0.35rem 0.5rem",
                  borderRadius: "6px",
                  background: subpath === item.subpath ? "rgba(255, 255, 255, 0.05)" : "transparent",
                  transition: "all 0.15s ease",
                  display: "block",
                  borderLeft: subpath === item.subpath ? "2px solid var(--accent-signals)" : "2px solid transparent",
                }}
              >
                {item.title}
              </a>
            ))}
          </div>
        ))}
      </aside>

      {/* Main Content Reading Area */}
      <section
        style={{
          flex: 1,
          padding: "3rem 4rem",
          maxWidth: "960px",
          overflowY: "auto",
          maxHeight: "calc(100vh - 72px)",
        }}
      >
        {article ? (
          <div>
            {/* Contextual Demo Alert Box */}
            {relatedDemoId && (
              <div
                className="glass-panel"
                style={{
                  padding: "1.25rem 1.5rem",
                  marginBottom: "2rem",
                  borderLeft: "4px solid var(--accent-signals)",
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "center",
                  gap: "1.5rem",
                }}
              >
                <div>
                  <h4 style={{ fontWeight: 600, color: "var(--text-primary)", fontSize: "0.95rem" }}>
                    Interactive Demo Available
                  </h4>
                  <p style={{ fontSize: "0.85rem", color: "var(--text-secondary)", marginTop: "0.2rem" }}>
                    This guide corresponds to **Demo {relatedDemoId}** inside our progression ladder.
                  </p>
                </div>
                <button
                  className="btn"
                  onClick={() => onNavigate(`#/demos/${relatedDemoId}`)}
                  style={{
                    background: "var(--accent-signals)",
                    color: "var(--bg-primary)",
                    borderColor: "transparent",
                    fontSize: "0.85rem",
                    padding: "0.5rem 1rem",
                  }}
                >
                  Run Live Demo
                </button>
              </div>
            )}

            {/* Markdown Elements */}
            <article style={{ fontSize: "1.05rem" }}>{renderMarkdown(article.content)}</article>
          </div>
        ) : (
          <div style={{ textAlign: "center", padding: "4rem 0" }}>
            <h2 style={{ color: "var(--text-primary)" }}>Article Not Found</h2>
            <p style={{ color: "var(--text-secondary)", marginTop: "1rem" }}>
              The dynamic document could not be loaded. Please return to the Start Here guide.
            </p>
            <button
              className="btn btn-primary"
              style={{ marginTop: "1.5rem" }}
              onClick={() => onNavigate("#/docs/start_here")}
            >
              Back to Start Here
            </button>
          </div>
        )}
      </section>
    </div>
  );
};

// Premium Styled Code Block Component with Copy to Clipboard support
const CodeBlock: React.FC<{ code: string; lang: string }> = ({ code, lang }) => {
  const [copied, setCopied] = React.useState(false);

  const handleCopy = () => {
    void navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div
      style={{
        margin: "1.5rem 0",
        borderRadius: "8px",
        overflow: "hidden",
        border: "1px solid var(--border-light)",
        position: "relative",
      }}
    >
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          padding: "0.5rem 1rem",
          background: "var(--bg-secondary)",
          borderBottom: "1px solid var(--border-light)",
          fontSize: "0.8rem",
          fontWeight: 600,
          color: "var(--text-muted)",
        }}
      >
        <span>{lang.toUpperCase() || "CODE"}</span>
        <button
          onClick={handleCopy}
          style={{
            background: "transparent",
            border: "none",
            color: copied ? "var(--accent-resources)" : "var(--text-secondary)",
            cursor: "pointer",
            fontSize: "0.75rem",
            fontWeight: 500,
          }}
        >
          {copied ? "COPIED" : "COPY CODE"}
        </button>
      </div>
      <pre style={{ margin: 0, padding: "1.25rem", overflowX: "auto", background: "var(--bg-darker)" }}>
        <code style={{ color: "#a5f3fc" }}>{code}</code>
      </pre>
    </div>
  );
};
