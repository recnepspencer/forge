import type { RouteState } from "./router";
import { WorthLogo } from "./WorthLogo";
import type { RefObject } from "react";

interface LayoutProps {
  currentRoute: RouteState;
  docsLinkRef: RefObject<HTMLAnchorElement | null>;
  docsMenuOpen: boolean;
  onDocsMenuToggle: () => void;
  onNavigate: (path: string) => void;
  children: React.ReactNode;
}

const navItems = [
  { href: "#/", label: "Home", match: (route: RouteState) => route.type === "landing" },
  { href: "#/docs", label: "Docs", match: (route: RouteState) => route.type === "docs" },
];

export function Layout({
  currentRoute,
  docsLinkRef,
  docsMenuOpen,
  onDocsMenuToggle,
  onNavigate,
  children,
}: LayoutProps) {
  return (
    <div className="site-shell">
      <header className="nav-bar">
        <button
          aria-label="Worth Signals home"
          className="brand brand-button"
          onClick={() => onNavigate("#/")}
          type="button"
        >
          <WorthLogo />
        </button>

        <nav className="nav-links" aria-label="Primary">
          {navItems.map((item) => (
            <a
              aria-controls={item.href === "#/docs" ? "docs-navigation" : undefined}
              aria-expanded={item.href === "#/docs" && currentRoute.type === "docs" ? docsMenuOpen : undefined}
              key={item.href}
              href={item.href}
              className={`nav-link ${item.match(currentRoute) ? "active" : ""}`}
              onClick={(event) => {
                event.preventDefault();
                if (item.href === "#/docs") {
                  onDocsMenuToggle();
                  if (currentRoute.type !== "docs") onNavigate(item.href);
                  return;
                }
                onNavigate(item.href);
              }}
              ref={item.href === "#/docs" ? docsLinkRef : undefined}
            >
              {item.label}
            </a>
          ))}
        </nav>
      </header>

      <main className="site-main">{children}</main>

      <footer className="site-footer">
        <div className="container site-footer-grid">
          <div>
            <strong>Worth Signals</strong>
            <p>
              A demo site for the Worth Signals WASM runtime: signals, forms, routes,
              resources, dialogs, and history behaving like one product system.
            </p>
          </div>
        </div>
      </footer>
    </div>
  );
}
