import React from "react";
import { motion, useReducedMotion } from "motion/react";
import { ArrowLeft, ArrowRight } from "lucide-react";
import { demoRegistry } from "../state/demoData";
import { CompositionGlueArtifact } from "./CompositionGlueArtifact";
import { FormsComposerField } from "./FormsComposerField";
import { ResourceLineArtifact } from "./ResourceLineArtifact";
import { ResourceOptimisticArtifact } from "./ResourceOptimisticArtifact";
import { RouterRouteField } from "./RouterRouteField";
import { SignalWaveArtifact } from "./SignalWaveArtifact";
import { LandingInstallCommand } from "./LandingInstallCommand";

interface LandingPageProps {
  onNavigate: (path: string) => void;
}

const capabilitySlides = [
  {
    id: 1,
    accent: "signals",
    eyebrow: "Signals",
    headline: "State changes once. Everything derived follows.",
    body: "Start with the smallest Forge primitive: retained local state, derived outputs, and one coherent update path.",
    seed: 11,
    values: [18, 42, 128, 7, 3, 89, 64, 21],
  },
  {
    id: 2,
    accent: "resources",
    eyebrow: "Resource Lines",
    headline: "Every read gets a lifecycle.",
    body: "Materialize catalog and detail lines, then inspect value, status, freshness, diagnostics, and history from the runtime.",
    seed: 17,
    values: [22, 4, 88, 12, 5, 71, 39, 8],
  },
  {
    id: 3,
    accent: "forms",
    eyebrow: "Forms",
    headline: "Form readiness belongs to the runtime.",
    body: "Source truth, draft edits, validation, dirty state, and submit posture stay in one controller instead of scattered component glue.",
    seed: 23,
    values: [12, 62, 4, 98, 2, 71, 31, 9],
  },
  {
    id: 4,
    accent: "router",
    eyebrow: "Router",
    headline: "Routes can own more than matching.",
    body: "Browser ingress, role-sensitive admission, breadcrumbs, and replayable session history live behind one route authority.",
    seed: 37,
    values: [5, 240, 33, 18, 4, 72, 12, 9],
  },
  {
    id: 5,
    accent: "resources",
    eyebrow: "Optimistic Resources",
    headline: "Optimistic writes should be inspectable.",
    body: "Items appear immediately, then the resource line records whether the write was confirmed, reconciled, or rolled back.",
    seed: 41,
    values: [204, 16, 74, 1, 32, 68, 11, 2],
  },
  {
    id: 6,
    accent: "composition",
    eyebrow: "Composition",
    headline: "The adapter layer is the tax.",
    body: "See route, form, resource, write lifecycle, and diagnostics compose without building a second app framework beside React.",
    seed: 59,
    values: [3, 77, 91, 8, 144, 24, 12, 6],
  },
];

const accentGlowColors = {
  composition: "rgba(255, 156, 183, 0.18)",
  forms: "rgba(245, 183, 109, 0.18)",
  resources: "rgba(121, 223, 177, 0.18)",
  router: "rgba(195, 165, 255, 0.18)",
  signals: "rgba(125, 199, 255, 0.18)",
} as const;

type CarouselPosition = {
  filter: string;
  opacity: number;
  pointerEvents: "auto" | "none";
  rotateY: number;
  scale: number;
  x: string;
  z: number;
  zIndex: number;
};

const MotionDiv = motion.div as unknown as React.ElementType;
const MotionArticle = motion.article as unknown as React.ElementType;

export const LandingPage: React.FC<LandingPageProps> = ({ onNavigate }) => {
  const [activeIndex, setActiveIndex] = React.useState(0);
  const prefersReducedMotion = useReducedMotion();
  const active = capabilitySlides[activeIndex];
  const previousSlide = capabilitySlides[(activeIndex - 1 + capabilitySlides.length) % capabilitySlides.length];
  const nextSlide = capabilitySlides[(activeIndex + 1) % capabilitySlides.length];

  const goTo = (nextIndex: number) => {
    setActiveIndex((nextIndex + capabilitySlides.length) % capabilitySlides.length);
  };

  const relativePosition = (index: number) => {
    const raw = index - activeIndex;
    if (raw > capabilitySlides.length / 2) return raw - capabilitySlides.length;
    if (raw < -capabilitySlides.length / 2) return raw + capabilitySlides.length;
    return raw;
  };

  const cardPosition = (relative: number): CarouselPosition => {
    if (prefersReducedMotion) {
      return {
        filter: "blur(0px) saturate(1)",
        opacity: relative === 0 ? 1 : 0,
        pointerEvents: relative === 0 ? "auto" : "none",
        rotateY: 0,
        scale: 1,
        x: relative === 0 ? "0rem" : `${relative * 4}rem`,
        z: 0,
        zIndex: relative === 0 ? 5 : 1,
      };
    }

    if (relative === 0) {
      return {
        filter: "blur(0px) saturate(1)",
        opacity: 1,
        pointerEvents: "auto",
        rotateY: 0,
        scale: 1,
        x: "0rem",
        z: 0,
        zIndex: 5,
      };
    }

    if (relative === -1 || relative === 1) {
      return {
        filter: "blur(5px) saturate(0.78)",
        opacity: 0.42,
        pointerEvents: "none",
        rotateY: relative === -1 ? 68 : -68,
        scale: 0.82,
        x: relative === -1 ? "-42rem" : "42rem",
        z: -240,
        zIndex: 3,
      };
    }

    return {
      filter: "blur(12px) saturate(0.55)",
      opacity: 0.08,
      pointerEvents: "none",
      rotateY: relative < 0 ? 78 : -78,
      scale: 0.68,
      x: relative < 0 ? "-54rem" : "54rem",
      z: -420,
      zIndex: 1,
    };
  };

  return (
    <main className="xai-landing xai-carousel-home">
      <section className="container xai-carousel-shell">
        <div className="xai-carousel-title">
          <span className="xai-eyebrow">Forge signal demo</span>
          <h1>A runtime for UI state that has to stay coordinated.</h1>
          <p>
            Walk through five focused demos, one capability at a time. Start
            with signals, then see how Forge carries the same runtime discipline
            into forms, routes, resources, and composed workflows.
          </p>
          <div className="xai-carousel-actions">
            <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos/1")} type="button">
              Start with signals
              <ArrowRight aria-hidden="true" size={17} />
            </button>
          </div>
          <LandingInstallCommand />
        </div>

        <div className="xai-carousel-stage">
          <MotionDiv
            animate={{ backgroundColor: accentGlowColors[previousSlide.accent as keyof typeof accentGlowColors] }}
            className="xai-card-glow xai-card-glow-left"
            transition={{ duration: 0.7, ease: [0.16, 1, 0.3, 1] }}
          />
          <MotionDiv
            animate={{ backgroundColor: accentGlowColors[nextSlide.accent as keyof typeof accentGlowColors] }}
            className="xai-card-glow xai-card-glow-right"
            transition={{ duration: 0.7, ease: [0.16, 1, 0.3, 1] }}
          />

          <button className="xai-carousel-arrow xai-carousel-arrow-left" onClick={() => goTo(activeIndex - 1)} title="Previous capability" type="button">
            <ArrowLeft aria-hidden="true" size={20} />
          </button>

          <div className={`xai-capability-carousel accent-${active.accent}`}>
            <div className="xai-carousel-window" aria-live="polite" aria-roledescription="carousel">
              {capabilitySlides.map((slide, index) => {
                const relative = relativePosition(index);
                const position = cardPosition(relative);
                const demo = demoRegistry.find((entry) => entry.id === slide.id);

                return (
                  <MotionArticle
                    key={slide.id}
                    animate={position}
                    className={`xai-carousel-slide accent-${slide.accent}`}
                    initial={false}
                    transition={{
                      damping: 26,
                      mass: 0.78,
                      stiffness: 170,
                      type: "spring",
                    }}
                  >
                    <div
                      className={`xai-slide-artifact${slide.id === 3 ? " xai-slide-artifact-form" : ""}${slide.id === 5 ? " xai-slide-artifact-resource" : ""}${slide.id === 6 ? " xai-slide-artifact-composition" : ""}`}
                      aria-hidden="true"
                    >
                      {slide.id === 1 ? (
                        <SignalWaveArtifact />
                      ) : slide.id === 2 ? (
                        <ResourceLineArtifact />
                      ) : slide.id === 3 ? (
                        <FormsComposerField />
                      ) : slide.id === 4 ? (
                        <RouterRouteField />
                      ) : slide.id === 5 ? (
                        <ResourceOptimisticArtifact />
                      ) : slide.id === 6 ? (
                        <CompositionGlueArtifact />
                      ) : (
                        <span />
                      )}
                    </div>

                    <div className="xai-slide-copy">
                      <span>{`0${slide.id} / ${slide.eyebrow}`}</span>
                      <h2>{slide.headline}</h2>
                      <p>{slide.body}</p>
                    </div>

                    <div className="xai-slide-actions">
                      <button className="xai-button xai-button-primary" onClick={() => onNavigate(`#/demos/${slide.id}`)} type="button">
                        Open demo
                      </button>
                      <button
                        className="xai-button xai-button-secondary"
                        onClick={() => onNavigate(`#/docs/${demo?.relatedDocsPath ?? ""}`)}
                        type="button"
                      >
                        Read docs
                      </button>
                    </div>
                  </MotionArticle>
                );
              })}
            </div>
          </div>

          <button className="xai-carousel-arrow xai-carousel-arrow-right" onClick={() => goTo(activeIndex + 1)} title="Next capability" type="button">
            <ArrowRight aria-hidden="true" size={20} />
          </button>
        </div>
      </section>
    </main>
  );
};
