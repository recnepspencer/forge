import { useState } from "react";
import { demoRegistry, type DemoMetadata } from "../state/demoData";

interface DemoShellProps {
  demo: DemoMetadata;
  onNavigate: (path: string) => void;
  children: any; // Interactive demo component
  inspectorContent?: any; // Telemetry inspector printing active WASM variables
  customCodeBlock?: any;
}

export const DemoShell: React.FC<DemoShellProps> = ({
  demo,
  onNavigate,
  children,
  inspectorContent,
  customCodeBlock,
}) => {
  const [activeTab, setActiveTab] = useState<"WORTH" | "alternative">("WORTH");
  const [copied, setCopied] = useState(false);
  const [detailsOpen, setDetailsOpen] = useState(false);
  const totalDemos = demoRegistry.length;

  const activeAccent = 
    demo.id === 1 ? "var(--accent-signals)" :
    demo.id === 2 ? "var(--accent-forms)" :
    demo.id === 3 ? "var(--accent-router)" :
    demo.id === 4 ? "var(--accent-resources)" :
    demo.id === 5 ? "var(--accent-composed)" :
    "var(--accent-history)";

  const handleCopyCode = () => {
    const textToCopy = activeTab === "WORTH" ? demo.WORTHCode : demo.alternativeCode;
    void navigator.clipboard.writeText(textToCopy);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div
      style={{
        background: "var(--bg-primary)",
        minHeight: "calc(100vh - 72px)",
        display: "flex",
        flexDirection: "column",
      }}
    >
      {/* Top Banner Context */}
      <div
        style={{
          borderBottom: "1px solid var(--border-light)",
          background: "var(--bg-secondary)",
          padding: "1.5rem 2rem",
        }}
      >
        <div className="container" style={{ display: "flex", justifyContent: "space-between", alignItems: "center", gap: "1rem" }}>
          <div>
            <div style={{ display: "flex", alignItems: "center", gap: "0.5rem", marginBottom: "0.3rem", flexWrap: "wrap" }}>
              <span
                style={{
                  fontSize: "0.75rem",
                  fontWeight: 700,
                  color: activeAccent,
                  letterSpacing: "1px",
                }}
              >
                DEMO {demo.id} OF {totalDemos}
              </span>
              <span style={{ color: "var(--text-muted)", fontSize: "0.8rem" }}>•</span>
              <span className="badge" style={{ color: activeAccent, border: `1px solid ${activeAccent}33` }}>
                {demo.difficulty}
              </span>
            </div>
            <h2 style={{ fontSize: "1.5rem", fontWeight: 700, color: "var(--text-primary)" }}>{demo.title}</h2>
          </div>
          <button className="btn" onClick={() => onNavigate("#/")} style={{ fontSize: "0.85rem" }}>
            Back home
          </button>
        </div>
      </div>

      {/* Main Workspace split */}
      <div style={{ display: "flex", flex: 1, flexDirection: "column" }}>
        <div
          style={{
            display: "grid",
            flex: 1,
          }}
          className="workspace-grid"
        >
          {/* LEFT: Live Interactive Demo area */}
          <div
            style={{
              padding: "2rem",
              display: "flex",
              flexDirection: "column",
              gap: "2rem",
              overflowY: "auto",
            }}
            className="demo-primary-column"
          >
            <div>
              <h3 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", marginBottom: "0.5rem" }}>
                PURPOSE
              </h3>
              <p style={{ color: "var(--text-secondary)", fontSize: "0.95rem", lineHeight: "1.6" }}>
                {demo.purpose}
              </p>
            </div>

            {/* Interactive Demo Area */}
            <div>
              <h3 style={{ fontSize: "0.85rem", fontWeight: 700, color: "var(--text-muted)", letterSpacing: "1px", marginBottom: "1rem" }}>
                LIVE RUNTIME INTERACTION AREA
              </h3>
              <div
                className="glass-panel"
                style={{
                  padding: "2rem",
                  background: "rgba(22, 28, 45, 0.5)",
                  border: `1px solid ${activeAccent}15`,
                  boxShadow: `0 4px 30px rgba(0,0,0,0.1), inset 0 0 12px ${activeAccent}08`,
                }}
              >
                {children}
              </div>
            </div>

            {/* "What you get" capabilities checklist */}
            <div>
              <h3 style={{ fontSize: "1rem", fontWeight: 600, color: "var(--text-primary)", marginBottom: "1rem" }}>
                BUILT-IN CAPABILITIES SURFACED
              </h3>
              <div style={{ display: "flex", flexDirection: "column", gap: "0.75rem" }}>
                {demo.whatYouGet.map((cap, idx) => (
                  <div
                    key={idx}
                    className="glass-panel"
                    style={{
                      display: "flex",
                      alignItems: "center",
                      gap: "0.75rem",
                      padding: "0.75rem 1rem",
                      background: "rgba(255, 255, 255, 0.01)",
                    }}
                  >
                    <span style={{ color: activeAccent, fontWeight: 700, fontSize: "1.1rem" }}>✓</span>
                    <span style={{ fontSize: "0.88rem", color: "var(--text-secondary)" }}>{cap}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* RIGHT: Live Code block & Telemetry Inspector */}
          <div
            style={{
              padding: "1.4rem",
              background: "var(--bg-darker)",
              display: "flex",
              flexDirection: "column",
              gap: "1.25rem",
              overflowY: "auto",
            }}
            className="demo-secondary-column"
          >
            <div className="demo-mobile-details-toggle">
              <button
                className="btn"
                onClick={() => setDetailsOpen((open) => !open)}
                style={{ width: "100%", justifyContent: "space-between" }}
              >
                <span>{detailsOpen ? "Hide Code & Proof" : "Show Code & Proof"}</span>
                <span>{detailsOpen ? "−" : "+"}</span>
              </button>
            </div>

            <div className={`demo-secondary-content ${detailsOpen ? "open" : ""}`}>
            {/* Code Tab block */}
            <div>
              <h3 style={{ fontSize: "0.85rem", fontWeight: 700, color: "var(--text-muted)", letterSpacing: "1px", marginBottom: "1rem" }}>
                PROOF RAIL
              </h3>
              {customCodeBlock ? (
                customCodeBlock
              ) : (
                <div style={{ border: "1px solid var(--border-light)", borderRadius: "8px", overflow: "hidden" }}>
                  <div
                    style={{
                      display: "flex",
                      justifyContent: "space-between",
                      alignItems: "center",
                      background: "var(--bg-secondary)",
                      padding: "0 1rem",
                      borderBottom: "1px solid var(--border-light)",
                    }}
                  >
                    <div style={{ display: "flex", gap: "0.25rem" }}>
                      <button
                        onClick={() => setActiveTab("WORTH")}
                        style={{
                          padding: "0.75rem 1rem",
                          background: "transparent",
                          border: "none",
                          borderBottom: activeTab === "WORTH" ? `2px solid ${activeAccent}` : "2px solid transparent",
                          color: activeTab === "WORTH" ? "var(--text-primary)" : "var(--text-muted)",
                          fontWeight: 600,
                          fontSize: "0.85rem",
                          cursor: "pointer",
                        }}
                      >
                        WORTH
                      </button>
                      <button
                        onClick={() => setActiveTab("alternative")}
                        style={{
                          padding: "0.75rem 1rem",
                          background: "transparent",
                          border: "none",
                          borderBottom: activeTab === "alternative" ? `2px solid ${activeAccent}` : "2px solid transparent",
                          color: activeTab === "alternative" ? "var(--text-primary)" : "var(--text-muted)",
                          fontWeight: 600,
                          fontSize: "0.85rem",
                          cursor: "pointer",
                        }}
                      >
                        Compare
                      </button>
                    </div>
                    <button
                      onClick={handleCopyCode}
                      style={{
                        background: "transparent",
                        border: "none",
                        color: copied ? "var(--accent-resources)" : "var(--text-secondary)",
                        cursor: "pointer",
                        fontSize: "0.75rem",
                        fontWeight: 600,
                      }}
                    >
                      {copied ? "COPIED!" : "COPY"}
                    </button>
                  </div>

                  {/* Code text */}
                  <pre
                    style={{
                      margin: 0,
                      padding: "1.25rem",
                      background: "var(--bg-darker)",
                      overflowX: "auto",
                      fontSize: "0.76rem",
                      lineHeight: "1.55",
                    }}
                  >
                    <code style={{ color: activeTab === "WORTH" ? "#cffafe" : "#fed7aa" }}>
                      {activeTab === "WORTH" ? demo.WORTHCode : demo.alternativeCode}
                    </code>
                  </pre>

                  {/* Explanatory footer */}
                  <div style={{ padding: "1rem", borderTop: "1px solid var(--border-light)", fontSize: "0.85rem", color: "var(--text-secondary)", background: "rgba(255,255,255,0.01)" }}>
                    <strong>Why it matters</strong>: {activeTab === "WORTH" ? demo.explanationWORTH : demo.explanationAlternative}
                  </div>
                </div>
              )}
            </div>

            {/* Telemetry Inspector Area */}
            {inspectorContent && (
              <div>
                <h3 style={{ fontSize: "0.78rem", fontWeight: 700, color: "var(--text-muted)", letterSpacing: "1px", marginBottom: "0.75rem" }}>
                  LIVE ROUTE PROOF
                </h3>
                <div
                  style={{
                    background: "#030303",
                    border: "1px solid rgba(255, 255, 255, 0.05)",
                    borderRadius: "6px",
                    padding: "1rem",
                    fontFamily: '"JetBrains Mono", monospace',
                    fontSize: "0.78rem",
                    lineHeight: "1.6",
                    color: "#e2e8f0",
                  }}
                >
                  {inspectorContent}
                </div>
              </div>
            )}
            </div>
          </div>
        </div>

        {/* Bottom bar ladder navigation */}
        <div
          style={{
            borderTop: "1px solid var(--border-light)",
            background: "var(--bg-secondary)",
            padding: "1.25rem 2rem",
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
          }}
        >
          {demo.id > 1 ? (
            <button className="btn" onClick={() => onNavigate(`#/demos/${demo.id - 1}`)}>
              ← Previous: Demo {demo.id - 1}
            </button>
          ) : (
            <div />
          )}

          <a
            href={`#/docs/${demo.relatedDocsPath}`}
            onClick={(e) => {
              e.preventDefault();
              onNavigate(`#/docs/${demo.relatedDocsPath}`);
            }}
            style={{ fontSize: "0.9rem", color: activeAccent, textDecoration: "underline", fontWeight: 600 }}
          >
            Read related documentation guide →
          </a>

          {demo.id < totalDemos ? (
            <button
              className="btn btn-primary"
              onClick={() => onNavigate(`#/demos/${demo.id + 1}`)}
              style={{
                background: activeAccent,
                color: "var(--bg-primary)",
                border: "none",
                fontWeight: 700,
              }}
            >
              Next: Demo {demo.id + 1} →
            </button>
          ) : (
            <button
              className="btn"
              onClick={() => onNavigate("#/")}
              style={{
                background: "linear-gradient(135deg, var(--accent-signals), var(--accent-router))",
                color: "white",
                border: "none",
              }}
            >
              Finish & Return Home
            </button>
          )}
        </div>
      </div>
      
      <style>{`
        .workspace-grid {
          grid-template-columns: 1fr;
        }
        .demo-secondary-column {
          border-top: 1px solid var(--border-light);
        }
        .demo-secondary-content {
          display: none;
          flex-direction: column;
          gap: 2rem;
        }
        .demo-secondary-content.open {
          display: flex;
        }
        @media (min-width: 1024px) {
          .workspace-grid {
            grid-template-columns: minmax(0, 1.8fr) minmax(260px, 0.52fr);
          }
          .demo-primary-column {
            border-right: 1px solid var(--border-light);
          }
          .demo-secondary-column {
            border-top: none;
          }
          .demo-mobile-details-toggle {
            display: none;
          }
          .demo-secondary-content {
            display: flex;
          }
        }
      `}</style>
    </div>
  );
};
