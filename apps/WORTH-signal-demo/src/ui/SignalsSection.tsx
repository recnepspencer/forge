import React from "react";
import type {
  CallableSignalDiagnostics,
  CallableSignals,
} from "worth-signals-wasm";
import { createReactSignalsStore } from "worth-signals-wasm/react";

import { createDemoSignals } from "../platform/createDemoSignals";

import {
  createTransferGraph,
  disposeTransferGraph,
  type TransferGraph,
} from "./signals-demo/signalsTransferRuntime";
import { SignalsTransferWorkbench } from "./signals-demo/SignalsTransferWorkbench";
import "./signals-demo/signalsDemoNarrative.css";
import "./signalsSection.css";

interface SignalsSectionProps {
  onNavigate: (path: string) => void;
}

interface DemoOneGraphs {
  diagnostics: CallableSignalDiagnostics;
  reactStore: ReturnType<typeof createReactSignalsStore>;
  signals: CallableSignals;
  transfer: TransferGraph;
}

export function SignalsSection({ onNavigate }: SignalsSectionProps): React.ReactElement {
  const [graphs, setGraphs] = React.useState<DemoOneGraphs | null>(null);
  const [bootError, setBootError] = React.useState<string | null>(null);

  React.useEffect(() => {
    let active = true;
    let createdGraphs: DemoOneGraphs | null = null;

    createDemoSignals({ deployment: "mainThreadCompatibility" })
      .then((signals) => {
        const diagnostics = signals.diagnostics();
        const reactStore = createReactSignalsStore(signals);
        let transfer: TransferGraph | null = null;
        try {
          transfer = createTransferGraph(signals, diagnostics);
          createdGraphs = {
            diagnostics,
            reactStore,
            signals,
            transfer,
          };
        } catch (error) {
          if (transfer) disposeTransferGraph(transfer);
          reactStore.dispose();
          diagnostics.free();
          signals.free();
          throw error;
        }
        if (!active) {
          disposeDemoOneGraphs(createdGraphs);
          return;
        }
        setGraphs(createdGraphs);
      })
      .catch((error: unknown) => {
        if (active) {
          setBootError(error instanceof Error ? error.message : "Could not start the Worth runtime.");
        }
      });

    return () => {
      active = false;
      if (createdGraphs) disposeDemoOneGraphs(createdGraphs);
    };
  }, []);

  return (
    <div className="accent-signals signals-section">
      {bootError ? <div className="signals-runtime-message">{bootError}</div> : null}
      {!graphs && !bootError ? <div className="signals-runtime-message">Connecting to the Worth runtime…</div> : null}
      {graphs ? <SignalsTransferWorkbench graph={graphs.transfer} store={graphs.reactStore} /> : null}

      <div className="signals-docs-row">
        <button
          onClick={() => onNavigate("#/docs/core/diagnostics")}
          type="button"
        >
          Read the diagnostics guide <span aria-hidden="true">→</span>
        </button>
      </div>
    </div>
  );
}

function disposeDemoOneGraphs(graphs: DemoOneGraphs): void {
  graphs.reactStore.dispose();
  disposeTransferGraph(graphs.transfer);
  graphs.diagnostics.free();
  graphs.signals.free();
}
