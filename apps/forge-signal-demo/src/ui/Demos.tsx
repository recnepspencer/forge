import React, { useSyncExternalStore } from "react";
import { demoRegistry } from "../state/demoData";
import { CompositionSection } from "./CompositionSection";
import { FormsSection } from "./FormsSection";
import { ResourcesSection } from "./ResourcesSection";
import { RouterSection } from "./RouterSection";
import { SignalsSection } from "./SignalsSection";

export function useSignal<T>(signals: any, handle: any): T {
  const cacheRef = React.useRef<{ lastValue: T | undefined }>({ lastValue: undefined });

  const getSnapshot = React.useCallback(() => {
    if (!signals || !handle) return undefined as T;
    const rawValue = typeof handle.value === "function" ? handle.value() : handle;
    const last = cacheRef.current.lastValue;
    if (rawValue === last) {
      return last as T;
    }
    if (
      typeof rawValue === "object" &&
      rawValue !== null &&
      typeof last === "object" &&
      last !== null &&
      JSON.stringify(rawValue) === JSON.stringify(last)
    ) {
      return last as T;
    }

    cacheRef.current.lastValue = rawValue;
    return rawValue;
  }, [signals, handle]);

  return useSyncExternalStore(
    React.useCallback((onChange) => {
      if (!signals || !handle) return () => {};
      const disposable = signals.watch(handle, onChange);
      return () => {
        signals.nuke(disposable);
      };
    }, [signals, handle]),
    getSnapshot,
    getSnapshot,
  );
}

interface DemosContainerProps {
  demoId: number;
  onNavigate: (path: string) => void;
}

const liveSections = {
  1: SignalsSection,
  2: FormsSection,
  3: RouterSection,
  4: ResourcesSection,
  5: CompositionSection,
} as const;

export const DemosContainer: React.FC<DemosContainerProps> = ({ demoId, onNavigate }) => {
  const demo = demoRegistry.find((entry) => entry.id === demoId);
  const Section = liveSections[demoId as keyof typeof liveSections];

  if (!demo) {
    return <div className="xai-demo-offline">Demo not found.</div>;
  }

  if (demoId === 6) {
    return (
      <div className="xai-demo-offline">
        <h2>Demo 6 is temporarily offline.</h2>
        <p>
          The branching history demo is waiting on runtime fixes, so we have it
          out of the live rotation for now. The rest of the demo ladder is still
          available.
        </p>
        <div className="xai-demo-route-actions">
          <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos")} type="button">
            Back to ladder
          </button>
          <button
            className="xai-button xai-button-secondary"
            onClick={() => onNavigate(`#/docs/${demo.relatedDocsPath}`)}
            type="button"
          >
            Read related docs
          </button>
        </div>
      </div>
    );
  }

  if (!Section) {
    return <div className="xai-demo-offline">This route is not wired yet.</div>;
  }

  return (
    <div className="xai-landing xai-demo-route">
      <section className="xai-hero xai-demo-route-hero">
        <div className="container xai-demo-route-shell">
          <div className="xai-demo-route-copy">
            <span className="xai-eyebrow">{`Demo 0${demo.id}`}</span>
            <h1>{demo.title}</h1>
            <p>{demo.purpose}</p>
            <code>{demo.primaryMessage}</code>
          </div>
          <div className="xai-demo-route-actions">
            <button className="xai-button xai-button-primary" onClick={() => onNavigate("#/demos")} type="button">
              Back to ladder
            </button>
            <button
              className="xai-button xai-button-secondary"
              onClick={() => onNavigate(`#/docs/${demo.relatedDocsPath}`)}
              type="button"
            >
              Read related docs
            </button>
          </div>
        </div>
      </section>

      <div className="container xai-demo-route-body">
        <Section onNavigate={onNavigate} />
      </div>
    </div>
  );
};
