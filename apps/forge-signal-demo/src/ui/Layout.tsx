import React from "react";
import type { RouteState } from "./router";

interface LayoutProps {
  currentRoute: RouteState;
  onNavigate: (path: string) => void;
  children: React.ReactNode;
  isWasmBooted: boolean;
}

export const Layout: React.FC<LayoutProps> = ({
  currentRoute,
  onNavigate,
  children,
  isWasmBooted,
}) => {
  return (
    <div className="site-shell">
      <header className="nav-bar">
        <div className="brand" style={{ cursor: "pointer" }} onClick={() => onNavigate("#/")}>
          <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="url(#brandGrad)"
            strokeWidth="3"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <defs>
              <linearGradient id="brandGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stopColor="#06b6d4" />
                <stop offset="100%" stopColor="#a855f7" />
              </linearGradient>
            </defs>
            <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
          </svg>
          <span style={{ letterSpacing: "-0.5px" }}>forge</span>
          <span style={{ fontWeight: 300, color: "var(--text-secondary)", opacity: 0.8 }}>signal</span>
        </div>

        <nav className="nav-links">
          <a
            href="#/"
            className={`nav-link ${currentRoute.type === "landing" ? "active" : ""}`}
            onClick={(e) => {
              e.preventDefault();
              onNavigate("#/");
            }}
          >
            Features
          </a>
          <a
            href="#/docs"
            className={`nav-link ${currentRoute.type === "docs" ? "active" : ""}`}
            onClick={(e) => {
              e.preventDefault();
              onNavigate("#/docs");
            }}
          >
            Docs
          </a>
          <a
            href="#/demos"
            className={`nav-link ${currentRoute.type === "demos" || currentRoute.type === "demo-detail" ? "active" : ""}`}
            onClick={(e) => {
              e.preventDefault();
              onNavigate("#/demos");
            }}
          >
            Demos
          </a>
          
          <div
            className="wasm-status-badge"
            style={{
              display: "flex",
              alignItems: "center",
              gap: "0.5rem",
              padding: "0.4rem 0.8rem",
              borderRadius: "8px",
              background: isWasmBooted ? "rgba(6, 182, 212, 0.08)" : "rgba(245, 158, 11, 0.08)",
              border: `1px solid ${isWasmBooted ? "rgba(6, 182, 212, 0.3)" : "rgba(245, 158, 11, 0.3)"}`,
              fontSize: "0.8rem",
              fontWeight: 600,
              color: isWasmBooted ? "var(--accent-signals)" : "var(--accent-forms)",
              transition: "all 0.3s ease",
            }}
          >
            <span
              style={{
                width: "6px",
                height: "6px",
                borderRadius: "50%",
                background: isWasmBooted ? "var(--accent-signals)" : "var(--accent-forms)",
                boxShadow: isWasmBooted ? "0 0 8px var(--accent-signals)" : "none",
                display: "inline-block",
                animation: isWasmBooted ? "pulse 2s infinite ease-in-out" : "none",
              }}
            />
            {isWasmBooted ? "WASM CORE ACTIVE" : "BOOTING CORE..."}
          </div>
        </nav>
      </header>

      <main style={{ flex: 1, display: "flex", flexDirection: "column" }}>
        {children}
      </main>

      <footer
        style={{
          padding: "3rem 2rem",
          background: "var(--bg-darker)",
          borderTop: "1px solid var(--border-light)",
          color: "var(--text-muted)",
          fontSize: "0.85rem",
          textAlign: "center",
          position: "relative",
          zIndex: 10,
        }}
      >
        <div className="container" style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
          <p>© {new Date().getFullYear()} Forge Framework. Built on the Certified Relational WASM Kernel.</p>
          <div style={{ display: "flex", justifyContent: "center", gap: "1.5rem" }}>
            <a href="#/docs">Documentation</a>
            <span>•</span>
            <a href="#/demos">Interactive Demos</a>
            <span>•</span>
            <a href="https://github.com" target="_blank" rel="noreferrer">GitHub</a>
          </div>
          <p style={{ fontSize: "0.75rem", opacity: 0.6 }}>
            Venture-backed high-fidelity reactive engineering. Shipped fully static.
          </p>
        </div>
      </footer>
      
      <style>{`
        @keyframes pulse {
          0%, 100% { opacity: 0.6; transform: scale(1); }
          50% { opacity: 1; transform: scale(1.2); }
        }
      `}</style>
    </div>
  );
};
