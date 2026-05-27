import React, { useState, useEffect, useSyncExternalStore } from "react";
import { createSignals } from "forge-signal-wasm";
import { demoRegistry } from "../state/demoData";
import { DemoOne } from "./demos/DemoOne";
import { DemoTwo } from "./demos/DemoTwo";
import { DemoThree } from "./demos/DemoThree";
import { DemoFour } from "./demos/DemoFour";
import { DemoFive } from "./demos/DemoFive";
import { DemoSix } from "./demos/DemoSix";

// React hook to safely subscribe to Forge signal handles
export function useSignal<T>(signals: any, handle: any): T {
  const cacheRef = React.useRef<{ lastValue: T | undefined }>({ lastValue: undefined });

  const getSnapshot = React.useCallback(() => {
    if (!signals || !handle) return undefined as any;
    const rawValue = typeof handle.value === "function" ? handle.value() : handle;
    
    // Perform structural equality check to keep the returned reference stable
    const last = cacheRef.current.lastValue;
    if (rawValue === last) {
      return last;
    }
    if (
      typeof rawValue === "object" &&
      rawValue !== null &&
      typeof last === "object" &&
      last !== null
    ) {
      if (JSON.stringify(rawValue) === JSON.stringify(last)) {
        return last;
      }
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

export const DemosContainer: React.FC<DemosContainerProps> = ({ demoId, onNavigate }) => {
  const [signals, setSignals] = useState<any>(null);
  const [booting, setBooting] = useState(true);

  useEffect(() => {
    createSignals({ deployment: "mainThreadCompatibility" })
      .then((instance) => {
        setSignals(instance);
        setBooting(false);
      })
      .catch((err) => {
        console.error("Failed to initialize WASM signals", err);
        setBooting(false);
      });
  }, []);

  const demo = demoRegistry.find((d) => d.id === demoId);

  if (booting) {
    return (
      <div style={{ padding: "4rem", textAlign: "center", color: "var(--accent-signals)" }}>
        <div className="spinner" style={{ border: "3px solid rgba(6,182,212,0.1)", borderTop: "3px solid var(--accent-signals)", borderRadius: "50%", width: "40px", height: "40px", animation: "spin 1s linear infinite", margin: "0 auto 1.5rem auto" }} />
        <h3 style={{ fontWeight: 600 }}>BOOTING WEBASSEMBLY RELATIONAL CORE...</h3>
        <style>{`@keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }`}</style>
      </div>
    );
  }

  if (!demo || !signals) {
    return <div style={{ padding: "4rem", textAlign: "center" }}>Demo not found or engine failed to initialize.</div>;
  }

  return (
    <>
      {demoId === 1 && <DemoOne signals={signals} demo={demo} onNavigate={onNavigate} />}
      {demoId === 2 && <DemoTwo signals={signals} demo={demo} onNavigate={onNavigate} />}
      {demoId === 3 && <DemoThree signals={signals} demo={demo} onNavigate={onNavigate} />}
      {demoId === 4 && <DemoFour signals={signals} demo={demo} onNavigate={onNavigate} />}
      {demoId === 5 && <DemoFive signals={signals} demo={demo} onNavigate={onNavigate} />}
      {demoId === 6 && <DemoSix signals={signals} demo={demo} onNavigate={onNavigate} />}
    </>
  );
};
