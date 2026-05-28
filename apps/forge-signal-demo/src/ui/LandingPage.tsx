import React from "react";
import "./landingShell.css";
import "./landingPage.css";
import { FormsSection } from "./FormsSection";
import { SignalsSection } from "./SignalsSection";

interface LandingPageProps {
  onNavigate: (path: string) => void;
}

interface RevealProps {
  children: React.ReactNode;
}

type SectionPattern = "split" | "cards" | "feature";

interface CapabilitySection {
  id: string;
  eyebrow: string;
  title: string;
  body: string;
  accent: string;
  demoHref: string;
  docsHref: string;
  pattern: SectionPattern;
  typical: string[];
  forge: string[];
  visualLabel: string;
  visualTitle: string;
  visualBody: string;
  visualItems: string[];
}

const sectionData: CapabilitySection[] = [
  {
    id: "router",
    eyebrow: "03 / Router",
    title: "Typed session authority",
    body:
      "Routes are typed references with admission, breadcrumbs, browser stories, and session truth instead of raw strings plus parallel navigation policy.",
    accent: "router",
    demoHref: "#/demos/3",
    docsHref: "#/docs/router/index",
    pattern: "split",
    typical: ["string hrefs", "manual breadcrumbs", "custom blockers"],
    forge: ["typed route refs", "admission reports", "first-class browser session"],
    visualLabel: "Session lane",
    visualTitle: "Admit, redirect, explain",
    visualBody:
      "A route attempt carries typed params, admission outcome, and breadcrumb truth through one browser session surface.",
    visualItems: ["push: /products/17/edit", "admitted", "crumbs carried", "story recorded"],
  },
  {
    id: "resources",
    eyebrow: "04 / Resources",
    title: "Reads and writes with async proof",
    body:
      "Resource lines and operations model loading, settlement, recovery, feedback, and revalidation so apps stop writing custom polling and mutation folklore.",
    accent: "resources",
    demoHref: "#/demos/4",
    docsHref: "#/docs/resources/index",
    pattern: "feature",
    typical: ["query hooks", "mutation hooks", "toast mapping", "manual refetch choreography"],
    forge: ["resource line", "await settlement", "recovery summary", "shared feedback bridge"],
    visualLabel: "Lifecycle view",
    visualTitle: "Load, write, recover",
    visualBody:
      "Async truth stays coherent from first fetch through partial write settlement and follow-up refresh policy.",
    visualItems: ["loading", "ready", "pending write", "partial recovery", "refreshed"],
  },
  {
    id: "composition",
    eyebrow: "05 / Composition",
    title: "The primitives share one contract",
    body:
      "The payoff is not six isolated APIs. The payoff is that routes, dialogs, forms, resources, and writes already agree on readiness and lifecycle semantics.",
    accent: "composition",
    demoHref: "#/demos/5",
    docsHref: "#/docs/forms/route-coupling",
    pattern: "cards",
    typical: ["route state", "dialog state", "form state", "write state", "toast state"],
    forge: ["one route-aware dialog flow", "bound form controller", "managed write execution"],
    visualLabel: "Workflow storyboard",
    visualTitle: "Route → dialog → form → write",
    visualBody:
      "A real product workflow can move across multiple concerns without falling out of one runtime model.",
    visualItems: ["detail route active", "edit dialog open", "dirty form protected", "write closes on success"],
  },
  {
    id: "history",
    eyebrow: "06 / History",
    title: "Time and branching are first-class",
    body:
      "Replay, undo, forks, and merge-aware surfaces can live above the same runtime when history is modeled as retained truth rather than ad hoc snapshots.",
    accent: "history",
    demoHref: "#/demos/6",
    docsHref: "#/docs/resources/branch-native-effects",
    pattern: "feature",
    typical: ["undo stacks", "manual snapshots", "no branch semantics", "no merge plan"],
    forge: ["branch-aware runtime", "story/history surfaces", "merge reasoning in-app"],
    visualLabel: "Branch graph",
    visualTitle: "Forks you can reason about",
    visualBody:
      "State history is not just a debug log. It can drive branch exploration and merge-aware UI directly.",
    visualItems: ["main", "feature fork", "review branch", "merge verdict"],
  },
];

const capabilityPills = ["Signals", "Forms", "Router", "Resources", "Composition", "History"];

const RevealSection: React.FC<RevealProps> = ({ children }) => {
  const ref = React.useRef<HTMLDivElement>(null);
  const [visible, setVisible] = React.useState(false);

  React.useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setVisible(true);
          observer.unobserve(entry.target);
        }
      },
      { threshold: 0.12, rootMargin: "0px 0px -64px 0px" },
    );

    if (ref.current) {
      observer.observe(ref.current);
    }

    return () => observer.disconnect();
  }, []);

  return (
    <section ref={ref} className={`landing-reveal ${visible ? "is-visible" : ""}`}>
      {children}
    </section>
  );
};

function ComparisonCard({
  label,
  title,
  items,
  tone,
}: {
  label: string;
  title: string;
  items: string[];
  tone: "typical" | "forge";
}): React.ReactElement {
  return (
    <article className={`xai-compare-card xai-compare-card-${tone}`}>
      <span>{label}</span>
      <h4>{title}</h4>
      <ul>
        {items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </article>
  );
}

function VisualPanel({ section }: { section: CapabilitySection }): React.ReactElement {
  return (
    <article className={`xai-visual-card accent-${section.accent}`}>
      <div className="xai-visual-topline">
        <span>{section.visualLabel}</span>
        <span>{section.eyebrow.split("/")[0]?.trim()}</span>
      </div>
      <h3>{section.visualTitle}</h3>
      <p>{section.visualBody}</p>
      <div className={`xai-visual-items pattern-${section.pattern}`}>
        {section.visualItems.map((item, index) => (
          <div key={item} className="xai-visual-item">
            <strong>{String(index + 1).padStart(2, "0")}</strong>
            <span>{item}</span>
          </div>
        ))}
      </div>
    </article>
  );
}

function SectionActions({
  onNavigate,
  demoHref,
  docsHref,
}: {
  onNavigate: (path: string) => void;
  demoHref: string;
  docsHref: string;
}): React.ReactElement {
  return (
    <div className="xai-section-actions">
      <button className="xai-button xai-button-primary" onClick={() => onNavigate(demoHref)} type="button">
        Open demo
      </button>
      <button className="xai-button xai-button-secondary" onClick={() => onNavigate(docsHref)} type="button">
        Read docs
      </button>
    </div>
  );
}

export const LandingPage: React.FC<LandingPageProps> = ({ onNavigate }) => {
  return (
    <div className="xai-landing">
      <section className="xai-hero xai-hero-centered">
        <div className="container xai-hero-stack">
          <span className="xai-eyebrow">Forge demo ladder</span>
          <h1 className="xai-hero-title">
            One runtime for
            <br />
            forms, routes,
            <br />
            resources, and
            <br />
            history.
          </h1>
          <p className="xai-hero-body">
            Explore the demo ladder from local reactivity to branch-aware workflows,
            all powered by the same retained WASM runtime.
          </p>
          <div className="xai-hero-actions">
            <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos")} type="button">
              Explore demos
            </button>
            <button className="xai-button xai-button-secondary" onClick={() => onNavigate("#/docs")} type="button">
              Read docs
            </button>
          </div>

          <div className="xai-hero-preview accent-composition">
            <div className="xai-hero-preview-header">
              <span>Composed runtime preview</span>
              <span>Live model</span>
            </div>
            <div className="xai-hero-preview-grid">
              <div className="xai-preview-panel">
                <span>Route session</span>
                <strong>/products/17/edit</strong>
                <p>Browser story, breadcrumbs, and route admission all agree.</p>
              </div>
              <div className="xai-preview-panel">
                <span>Form controller</span>
                <strong>draft dirty / submit ready</strong>
                <p>Source, draft, effective, and dialog close policy stay aligned.</p>
              </div>
              <div className="xai-preview-panel">
                <span>Resource write</span>
                <strong>partial → recovery applied</strong>
                <p>Execution truth, feedback, and revalidation share one lifecycle.</p>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className="xai-intro-strip">
        <div className="container xai-intro-strip-inner">
          <p>
            Start with signals, then move through forms, routing, resources,
            composed workflows, and branch-native history.
          </p>
          <div className="xai-capability-rail">
            {capabilityPills.map((capability) => (
              <span key={capability}>{capability}</span>
            ))}
          </div>
        </div>
      </section>

      <div className="container xai-sections">
        <RevealSection>
          <SignalsSection onNavigate={onNavigate} />
        </RevealSection>

        <RevealSection>
          <FormsSection onNavigate={onNavigate} />
        </RevealSection>

        {sectionData.map((section) => (
          <RevealSection key={section.id}>
            <div className={`xai-section-band accent-${section.accent}`}>
              <div className="xai-section-heading">
                <span className="xai-section-eyebrow">{section.eyebrow}</span>
                <h2>{section.title}</h2>
                <p>{section.body}</p>
              </div>

              {section.pattern === "split" && (
                <div className="xai-section-layout xai-section-layout-split">
                  <div className="xai-section-copy">
                    <div className="xai-compare-grid">
                      <ComparisonCard
                        label="Typical"
                        title="What teams usually stitch together"
                        items={section.typical}
                        tone="typical"
                      />
                      <ComparisonCard
                        label="Forge"
                        title="What the runtime gives you"
                        items={section.forge}
                        tone="forge"
                      />
                    </div>
                    <SectionActions onNavigate={onNavigate} demoHref={section.demoHref} docsHref={section.docsHref} />
                  </div>
                  <VisualPanel section={section} />
                </div>
              )}

              {section.pattern === "cards" && (
                <div className="xai-section-layout xai-section-layout-cards">
                  <div className="xai-compare-grid xai-compare-grid-wide">
                    <ComparisonCard
                      label="Typical"
                      title="A stack of separate decisions"
                      items={section.typical}
                      tone="typical"
                    />
                    <ComparisonCard
                      label="Forge"
                      title="One authoritative surface"
                      items={section.forge}
                      tone="forge"
                    />
                  </div>
                  <div className="xai-section-support">
                    <VisualPanel section={section} />
                    <SectionActions onNavigate={onNavigate} demoHref={section.demoHref} docsHref={section.docsHref} />
                  </div>
                </div>
              )}

              {section.pattern === "feature" && (
                <div className="xai-section-layout xai-section-layout-feature">
                  <VisualPanel section={section} />
                  <div className="xai-feature-support">
                    <div className="xai-compare-grid">
                      <ComparisonCard
                        label="Typical"
                        title="App-local async glue"
                        items={section.typical}
                        tone="typical"
                      />
                      <ComparisonCard
                        label="Forge"
                        title="Shared runtime lifecycle"
                        items={section.forge}
                        tone="forge"
                      />
                    </div>
                    <SectionActions onNavigate={onNavigate} demoHref={section.demoHref} docsHref={section.docsHref} />
                  </div>
                </div>
              )}
            </div>
          </RevealSection>
        ))}

        <RevealSection>
          <div className="xai-closing-band">
            <span className="xai-section-eyebrow">Start with the ladder</span>
            <h2>See each surface alone, then see how they compose.</h2>
            <p>
              Use the demo ladder if you want runtime behavior. Use the docs if you
              want the exact public surfaces behind it.
            </p>
            <div className="xai-hero-actions">
              <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos")} type="button">
                Explore demos
              </button>
              <button className="xai-button xai-button-secondary" onClick={() => onNavigate("#/docs")} type="button">
                Browse docs
              </button>
            </div>
          </div>
        </RevealSection>
      </div>
    </div>
  );
};
