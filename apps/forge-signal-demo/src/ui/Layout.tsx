import type { RouteState } from "./router";

interface LayoutProps {
  currentRoute: RouteState;
  onNavigate: (path: string) => void;
  children: any;
}

const navItems = [
  { href: "#/", label: "Home", match: (route: RouteState) => route.type === "landing" },
  { href: "#/docs", label: "Docs", match: (route: RouteState) => route.type === "docs" },
];

export function Layout({
  currentRoute,
  onNavigate,
  children,
}: LayoutProps) {
  return (
    <div className="site-shell">
      <header className="nav-bar">
        <button className="brand brand-button" onClick={() => onNavigate("#/")} type="button">
          <span className="brand-mark" aria-hidden="true" />
          <span className="brand-wordmark">Forge</span>
        </button>

        <nav className="nav-links" aria-label="Primary">
          {navItems.map((item) => (
            <a
              key={item.href}
              href={item.href}
              className={`nav-link ${item.match(currentRoute) ? "active" : ""}`}
              onClick={(event) => {
                event.preventDefault();
                onNavigate(item.href);
              }}
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
            <strong>Forge Signal</strong>
            <p>
              A demo site for the Forge WASM runtime: signals, forms, routes,
              resources, dialogs, and history behaving like one product system.
            </p>
          </div>
          <div className="site-footer-links">
            <a href="#/" onClick={(event) => { event.preventDefault(); onNavigate("#/"); }}>
              Home
            </a>
            <a href="#/docs" onClick={(event) => { event.preventDefault(); onNavigate("#/docs"); }}>
              Docs
            </a>
          </div>
        </div>
      </footer>
    </div>
  );
}
