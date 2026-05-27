import React, { useRef } from "react";
import { act, cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";
import {
  createReactSignalsStore,
  ReactSignalsStoreProvider,
  useOutputValue,
  useSignalsDiagnosticsValue,
} from "@aust-group/forge-signal-wasm/react";

async function buildFixture() {
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = signals.input(1);
  const panel = signals.output({
    reads: [count.id],
    expr: {
      kind: "object",
      fields: [["count", { kind: "read", id: count.id }]],
    },
  });
  const store = createReactSignalsStore(signals);

  function dispose(): void {
    store.dispose();
    signals.free();
  }

  return { signals, count, panel, store, dispose };
}

async function flushReact(): Promise<void> {
  await act(async () => {
    await Promise.resolve();
    await Promise.resolve();
  });
}

function OutputReferenceProbe({
  output,
  store,
  revision,
  references,
}: {
  output: ReturnType<ReturnType<typeof createSignals>["output"]>;
  store: ReturnType<typeof createReactSignalsStore>;
  revision: number;
  references: Array<{ revision: number; value: { count: number } }>;
}): JSX.Element {
  const value = useOutputValue<{ count: number }>(output, store);
  const lastRevision = useRef<number | null>(null);

  if (lastRevision.current !== revision) {
    references.push({ revision, value });
    lastRevision.current = revision;
  }

  return <div>{value.count}</div>;
}

function DiagnosticsSelectorProbe(): JSX.Element {
  const latestObservation = useSignalsDiagnosticsValue(
    (snapshot) => snapshot.latestObservation,
  );
  return (
    <div data-testid="diagnostics-selector">
      {latestObservation === null ? "none" : "present"}
    </div>
  );
}

function DiagnosticsSelectorStabilityProbe({
  renderCountRef,
}: {
  renderCountRef: { current: number };
}): JSX.Element {
  renderCountRef.current += 1;
  const observationState = useSignalsDiagnosticsValue(
    (snapshot) => (snapshot.latestObservation === null ? "none" : "present"),
  );
  return <div data-testid="diagnostics-selector-stability">{observationState}</div>;
}

afterEach(() => {
  cleanup();
});

describe("React snapshot stability", () => {
  it("keeps object-valued output snapshots referentially stable across parent rerenders", async () => {
    const fixture = await buildFixture();
    const references: Array<{ revision: number; value: { count: number } }> = [];
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});

    try {
      const rendered = render(
        <OutputReferenceProbe
          output={fixture.panel}
          store={fixture.store}
          revision={0}
          references={references}
        />,
      );

      rendered.rerender(
        <OutputReferenceProbe
          output={fixture.panel}
          store={fixture.store}
          revision={1}
          references={references}
        />,
      );

      expect(references).toHaveLength(2);
      expect(references[0]?.value).toBe(references[1]?.value);

      act(() => {
        fixture.count.set(5);
      });
      await flushReact();

      rendered.rerender(
        <OutputReferenceProbe
          output={fixture.panel}
          store={fixture.store}
          revision={2}
          references={references}
        />,
      );

      expect(references).toHaveLength(3);
      expect(references[1]?.value).not.toBe(references[2]?.value);
      expect(references[2]?.value).toEqual({ count: 5 });
      expect(
        consoleError.mock.calls.some((call) =>
          call.some(
            (entry) =>
              typeof entry === "string" &&
              entry.includes("getSnapshot should be cached"),
          ),
        ),
      ).toBe(false);
      expect(
        consoleError.mock.calls.some((call) =>
          call.some(
            (entry) =>
              typeof entry === "string" &&
              entry.includes("Maximum update depth exceeded"),
          ),
        ),
      ).toBe(false);

      rendered.unmount();
    } finally {
      consoleError.mockRestore();
      fixture.dispose();
    }
  });

  it("supports selector-based diagnostics reads through the provider-backed store lane", async () => {
    const fixture = await buildFixture();

    try {
      const rendered = render(
        <ReactSignalsStoreProvider store={fixture.store}>
          <DiagnosticsSelectorProbe />
        </ReactSignalsStoreProvider>,
      );

      expect(rendered.getByTestId("diagnostics-selector").textContent).toBe("none");

      act(() => {
        fixture.count.set(2);
      });
      await flushReact();

      expect(rendered.getByTestId("diagnostics-selector").textContent).toBe("present");
      rendered.unmount();
    } finally {
      fixture.dispose();
    }
  });

  it("does not rerender selector consumers when diagnostics updates do not change the selected value", async () => {
    const fixture = await buildFixture();
    const renderCountRef = { current: 0 };

    try {
      const rendered = render(
        <ReactSignalsStoreProvider store={fixture.store}>
          <DiagnosticsSelectorStabilityProbe renderCountRef={renderCountRef} />
        </ReactSignalsStoreProvider>,
      );

      expect(rendered.getByTestId("diagnostics-selector-stability").textContent).toBe("none");
      expect(renderCountRef.current).toBe(1);

      act(() => {
        fixture.count.set(2);
      });
      await flushReact();

      expect(rendered.getByTestId("diagnostics-selector-stability").textContent).toBe("present");
      expect(renderCountRef.current).toBe(2);

      act(() => {
        fixture.count.set(3);
      });
      await flushReact();

      expect(rendered.getByTestId("diagnostics-selector-stability").textContent).toBe("present");
      expect(renderCountRef.current).toBe(2);
      rendered.unmount();
    } finally {
      fixture.dispose();
    }
  });
});
