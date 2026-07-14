import React, { useSyncExternalStore } from "react";
import { demoRegistry } from "../state/demoData";
import { CompositionSection } from "./CompositionSection";
import { FormsSection } from "./FormsSection";
import { ResourceLinesSection } from "./ResourceLinesSection";
import { ResourcesSection } from "./ResourcesSection";
import { RouterSection } from "./RouterSection";
import { SignalsSection } from "./SignalsSection";

export function useSignal<T>(signals: any, handle: any): T {
  const cacheRef = React.useRef<{ lastValue: T | undefined }>({ lastValue: undefined });

  const getSnapshot = React.useCallback(() => {
    if (!signals || !handle) return undefined as T;
    const rawValue = typeof handle.value === "function" ? handle.value() : handle;
    const last = cacheRef.current.lastValue;
    if (rawValue === last) return last as T;
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
      return () => signals.nuke(disposable);
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
  2: ResourceLinesSection,
  3: FormsSection,
  4: RouterSection,
  5: ResourcesSection,
  6: CompositionSection,
} as const;

const liveDemoIds = [1, 2, 3, 4, 5, 6] as const;

export const DemosContainer: React.FC<DemosContainerProps> = ({ demoId, onNavigate }) => {
  const demo = demoRegistry.find((entry) => entry.id === demoId);
  const Section = liveSections[demoId as keyof typeof liveSections];
  const demoIndex = liveDemoIds.findIndex((id) => id === demoId);
  const previousDemoId = demoIndex >= 0 ? liveDemoIds[(demoIndex - 1 + liveDemoIds.length) % liveDemoIds.length] : null;
  const nextDemoId = demoIndex >= 0 ? liveDemoIds[(demoIndex + 1) % liveDemoIds.length] : null;

  if (!demo) return <div className="xai-demo-offline">Demo not found.</div>;

  if (!Section) return <div className="xai-demo-offline">This route is not wired yet.</div>;

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
          <div className="xai-demo-route-nav-row">
            {previousDemoId ? (
              <button className="xai-demo-pager-button" onClick={() => onNavigate(`#/demos/${previousDemoId}`)} type="button">
                <span aria-hidden="true">←</span>
                <span>Previous</span>
              </button>
            ) : null}
            {nextDemoId ? (
              <button className="xai-demo-pager-button xai-demo-pager-button-next" onClick={() => onNavigate(`#/demos/${nextDemoId}`)} type="button">
                <span>Next</span>
                <span aria-hidden="true">→</span>
              </button>
            ) : null}
            <div className="xai-demo-route-count" aria-label="Jump to demo">
              {liveDemoIds.map((id) => (
                <button className={id === demoId ? "active" : ""} key={id} onClick={() => onNavigate(`#/demos/${id}`)} type="button">
                  {`0${id}`}
                </button>
              ))}
            </div>
          </div>
        </div>
      </section>

      <div className="container xai-demo-route-body">
        <Section onNavigate={onNavigate} />
      </div>
    </div>
  );
};
