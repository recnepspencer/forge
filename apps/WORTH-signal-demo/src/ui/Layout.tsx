import type { RouteState } from "./router";
import { WorthLogo } from "./WorthLogo";

interface LayoutProps {
  currentRoute: RouteState;
  onNavigate: (path: string) => void;
  children: React.ReactNode;
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
