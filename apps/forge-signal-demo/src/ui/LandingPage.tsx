import React from "react";
import "./landingShell.css";
import "./landingPage.css";
import { CompositionSection } from "./CompositionSection";
import { FormsSection } from "./FormsSection";
import { HistorySection } from "./HistorySection";
import { ResourcesSection } from "./ResourcesSection";
import { RouterSection } from "./RouterSection";
import { SignalsSection } from "./SignalsSection";

interface LandingPageProps {
  onNavigate: (path: string) => void;
}

interface RevealProps {
  children: React.ReactNode;
}

const capabilityPills = ["Signals", "Forms", "Router", "Resources", "Composition", "History"];

const RevealSection: React.FC<RevealProps> = ({ children }) => {
  const ref = React.useRef<HTMLDivElement | null>(null);
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

        <RevealSection>
          <RouterSection onNavigate={onNavigate} />
        </RevealSection>

        <RevealSection>
          <ResourcesSection onNavigate={onNavigate} />
        </RevealSection>

        <RevealSection>
          <CompositionSection onNavigate={onNavigate} />
        </RevealSection>

        <RevealSection>
          <HistorySection onNavigate={onNavigate} />
        </RevealSection>

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
