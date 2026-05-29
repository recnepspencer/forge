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

function CompositeRuntimeProbe({
  output,
  store,
  revision,
}: {
  output: ReturnType<ReturnType<typeof createSignals>["output"]>;
  store: ReturnType<typeof createReactSignalsStore>;
  revision: number;
}): JSX.Element {
  const value = useOutputValue<{ count: number }>(output, store);
  const observationState = useSignalsDiagnosticsValue(
    (snapshot) => (snapshot.latestObservation === null ? "none" : "present"),
    store,
  );

  return (
    <div data-testid="composite-runtime-probe">
      {JSON.stringify({
        revision,
        count: value.count,
        observationState,
      })}
    </div>
  );
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

  it("stays stable when React output reads, diagnostics selectors, watch, and effect all run together", async () => {
    const fixture = await buildFixture();
    const notices: Array<{ signalId: string; meaningfulChange: boolean }> = [];
    let effectCount = 0;
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});
    const watchHandle = fixture.signals.watch(fixture.panel, (notice) => {
      notices.push({
        signalId: notice.signalId,
        meaningfulChange: notice.meaningfulChange,
      });
    });
    const effectHandle = fixture.signals.effect(fixture.panel, () => {
      effectCount += 1;
    });

    try {
      const rendered = render(
        <ReactSignalsStoreProvider store={fixture.store}>
          <CompositeRuntimeProbe
            output={fixture.panel}
            store={fixture.store}
            revision={0}
          />
        </ReactSignalsStoreProvider>,
      );

      for (const nextCount of [2, 3, 4]) {
        act(() => {
          fixture.count.set(nextCount);
        });
        await flushReact();
      }

      rendered.rerender(
        <ReactSignalsStoreProvider store={fixture.store}>
          <CompositeRuntimeProbe
            output={fixture.panel}
            store={fixture.store}
            revision={1}
          />
        </ReactSignalsStoreProvider>,
      );
      await flushReact();

      expect(rendered.getByTestId("composite-runtime-probe").textContent).toContain(
        "\"count\":4",
      );
      expect(rendered.getByTestId("composite-runtime-probe").textContent).toContain(
        "\"observationState\":\"present\"",
      );
      expect(notices.length).toBeGreaterThanOrEqual(1);
      expect(notices.every((notice) => notice.signalId === fixture.panel.id)).toBe(true);
      expect(effectCount).toBeGreaterThanOrEqual(1);
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
      fixture.signals.nuke(watchHandle);
      fixture.signals.nuke(effectHandle);
      consoleError.mockRestore();
      fixture.dispose();
    }
  });
});
