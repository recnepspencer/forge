import React from "react";
import { act, cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "@aust-group/forge-signal-wasm/react";

type RenderCounts = Record<string, number>;

type DiagnosticsCapture = {
  current: ReturnType<ReturnType<typeof createSignals>["diagnostics"]> extends infer T
    ? T extends { latestObservation(): unknown; latestFlow(): unknown; performanceSummary(): unknown }
      ? {
          latestObservation: ReturnType<T["latestObservation"]>;
          latestFlow: ReturnType<T["latestFlow"]>;
          performanceSummary: ReturnType<T["performanceSummary"]>;
        } | null
      : never
    : never;
};

function buildFixture() {
  const signals = createSignals();
  const store = createReactSignalsStore(signals);
  const count = signals.input("count", 1);
  const doubled = signals.computed("doubled", {
    reads: ["count"],
    expr: {
      kind: "multiply",
      args: [
        { kind: "read", id: "count" },
        { kind: "value", value: 2 },
      ],
    },
  });
  const panel = signals.output("panel", {
    reads: ["count", "doubled"],
    expr: {
      kind: "object",
      fields: [
        ["count", { kind: "read", id: "count" }],
        ["doubled", { kind: "read", id: "doubled" }],
      ],
    },
  });

  function dispose(): void {
    store.dispose();
  }

  return { signals, store, count, doubled, panel, dispose };
}

function buildLargeParallelFixture(size = 1024) {
  const signals = createSignals();
  const inputs: Array<ReturnType<typeof signals.input>> = [];
  const leaves: Array<ReturnType<typeof signals.computed>> = [];

  for (let index = 0; index < size; index += 1) {
    const input = signals.input(`source:${index}`, index + 1);
    inputs.push(input);
    leaves.push(
      signals.computed(`leaf:${index}`, {
        reads: [input.id],
        expr: {
          kind: "multiply",
          args: [
            { kind: "read", id: input.id },
            { kind: "value", value: 2 },
          ],
        },
      }),
    );
  }

  const total = signals.computed("total", {
    reads: leaves.map((leaf) => leaf.id),
    expr: {
      kind: "sum",
      args: leaves.map((leaf) => ({ kind: "read", id: leaf.id })),
    },
  });

  function dispose(): void {
    total.free();
    for (const leaf of leaves) {
      leaf.free();
    }
    for (const input of inputs) {
      input.free();
    }
    signals.free();
  }

  return { signals, total, dispose };
}

async function flushQueuedDiagnostics(): Promise<void> {
  await act(async () => {
    await Promise.resolve();
    await Promise.resolve();
  });
}

function ValueProbe({
  label,
  signal,
  store,
  renderCounts,
}: {
  label: string;
  signal: ReturnType<ReturnType<typeof createSignals>["input"]>;
  store: ReturnType<typeof createReactSignalsStore>;
  renderCounts: RenderCounts;
}): JSX.Element {
  renderCounts[label] = (renderCounts[label] ?? 0) + 1;
  const value = useSignalValue<number>(signal, store);
  return <div data-testid={label}>{String(value)}</div>;
}

function OutputProbe({
  label,
  output,
  store,
  renderCounts,
}: {
  label: string;
  output: ReturnType<ReturnType<typeof createSignals>["output"]>;
  store: ReturnType<typeof createReactSignalsStore>;
  renderCounts: RenderCounts;
}): JSX.Element {
  renderCounts[label] = (renderCounts[label] ?? 0) + 1;
  const value = useOutputValue<{ count: number; doubled: number }>(output, store);
  return <div data-testid={label}>{JSON.stringify(value)}</div>;
}

function DiagnosticsProbe({
  store,
  capture,
  renderCounts,
}: {
  store: ReturnType<typeof createReactSignalsStore>;
  capture: DiagnosticsCapture;
  renderCounts: RenderCounts;
}): JSX.Element {
  renderCounts.diagnostics = (renderCounts.diagnostics ?? 0) + 1;
  const diagnostics = useSignalsDiagnostics(store);
  capture.current = diagnostics;
  return (
    <div data-testid="diagnostics">
      {String(diagnostics.latestObservation?.deliveredEventCount ?? 0)}
    </div>
  );
}

afterEach(() => {
  cleanup();
});

describe("forge-signal-wasm React harness", () => {
  it("supports callable input/output handles on the default package surface", async () => {
    const fixture = buildFixture();

    try {
      expect(fixture.count()).toBe(1);
      expect(fixture.doubled()).toBe(2);
      expect(fixture.panel()).toEqual({ count: 1, doubled: 2 });

      act(() => {
        fixture.count.set(4);
      });
      await flushQueuedDiagnostics();

      expect(fixture.count()).toBe(4);
      expect(fixture.doubled()).toBe(8);
      expect(fixture.panel()).toEqual({ count: 4, doubled: 8 });
    } finally {
      fixture.dispose();
    }
  });

  it("supports constant callback computed authoring through the wasm callback lane", async () => {
    const signals = createSignals();
    const answer = signals.computed("answer", () => 42);
    const generated = signals.computed(() => 7);

    try {
      expect(answer()).toBe(42);
      expect(generated()).toBe(7);
      const why = signals.diagnostics().why(answer.id);
      expect(why.recipeFamily).toBe("callbackConstantized");
      expect(why.callback?.purityPosture).toBe("constantizedNoSignalReads");
      expect(why.callback?.registered).toBe(false);

      const summary = signals.diagnostics().performanceSummary();
      expect(summary.activeComputeCallbackCount).toBe(0);
      expect(summary.computeCallbackRegistrationCount).toBeGreaterThanOrEqual(2);
      expect(summary.computeCallbackDisposalCount).toBeGreaterThanOrEqual(2);
      expect(summary.computeCallbackInvocationCount).toBeGreaterThanOrEqual(2);
      expect(summary.computeCallbackReturnSerializationBreadth).toBeGreaterThanOrEqual(2);
      expect(summary.computeCallbackConstantNoSignalReadClassificationCount).toBeGreaterThanOrEqual(2);
    } finally {
      signals.free();
    }
  });

  it("supports stable callback computed reads through committed runtime truth", async () => {
    const signals = createSignals();
    const count = signals.input("count", 1);
    const doubled = signals.computed("doubled", () => count() * 2);

    try {
      expect(doubled()).toBe(2);

      act(() => {
        count.set(5);
      });
      await flushQueuedDiagnostics();

      expect(doubled()).toBe(10);
      const why = signals.diagnostics().why(doubled.id);
      expect(why.recipeFamily).toBe("callback");
      expect(why.callback?.purityPosture).toBe("signalTracked");
      expect(why.callback?.currentReads).toEqual(["count"]);

      const summary = signals.diagnostics().performanceSummary();
      expect(summary.activeComputeCallbackCount).toBeGreaterThanOrEqual(1);
      expect(summary.computeCallbackCollectorInstallationCount).toBeGreaterThanOrEqual(2);
      expect(summary.computeCallbackCaptureCount).toBeGreaterThanOrEqual(2);
      expect(summary.computeCallbackCapturedReadCount).toBeGreaterThanOrEqual(2);
      expect(summary.computeCallbackRuntimeReadBreadth).toBe(2);
      expect(summary.computeCallbackSignalTrackedClassificationCount).toBeGreaterThanOrEqual(1);
    } finally {
      signals.free();
    }
  });

  it("keeps callback runtime read breadth bounded to actual reads instead of whole-store preload", async () => {
    const signals = createSignals();
    const count = signals.input("count", 1);
    for (let index = 0; index < 24; index += 1) {
      signals.input(`unrelated:${index}`, index);
    }
    const doubled = signals.computed("doubled", () => count() * 2);

    try {
      expect(doubled()).toBe(2);

      act(() => {
        count.set(5);
      });
      await flushQueuedDiagnostics();

      expect(doubled()).toBe(10);

      const summary = signals.diagnostics().performanceSummary();
      expect(summary.computeCallbackRuntimeReadBreadth).toBe(2);
    } finally {
      signals.free();
    }
  });

  it("preserves outer callback evaluation when nested callback reads run inside it", async () => {
    const signals = createSignals();
    const count = signals.input("count", 1);
    const offset = signals.input("offset", 1);
    const inner = signals.computed("inner", () => count() * 2);
    const outer = signals.computed("outer", () => inner() + offset());

    try {
      expect(outer()).toBe(3);

      act(() => {
        count.set(4);
      });
      await flushQueuedDiagnostics();
      expect(outer()).toBe(9);

      act(() => {
        offset.set(5);
      });
      await flushQueuedDiagnostics();
      expect(outer()).toBe(13);
    } finally {
      signals.free();
    }
  });

  it("rewires callback computed dependency branches through committed runtime truth", async () => {
    const signals = createSignals();
    const enabled = signals.input("enabled", true);
    const name = signals.input("name", "Ada");
    const label = signals.computed("label", () => (enabled() ? name() : "disabled"));

    try {
      expect(label()).toBe("Ada");

      act(() => {
        enabled.set(false);
      });
      await flushQueuedDiagnostics();
      expect(label()).toBe("disabled");

      const beforeNameOnly = signals.diagnostics().performanceSummary();

      act(() => {
        name.set("Grace");
      });
      await flushQueuedDiagnostics();
      expect(label()).toBe("disabled");

      const afterNameOnly = signals.diagnostics().performanceSummary();
      expect(afterNameOnly.computeCallbackInvocationCount).toBe(
        beforeNameOnly.computeCallbackInvocationCount,
      );

      act(() => {
        enabled.set(true);
      });
      await flushQueuedDiagnostics();
      expect(label()).toBe("Grace");

      const summary = signals.diagnostics().performanceSummary();
      expect(summary.computeCallbackDependencyPatchCount).toBeGreaterThanOrEqual(2);
      expect(summary.computeCallbackDependencyPatchAddedCount).toBeGreaterThanOrEqual(1);
      expect(summary.computeCallbackDependencyPatchRemovedCount).toBeGreaterThanOrEqual(1);
      expect(summary.computeCallbackRuntimeReadBreadth).toBe(5);
    } finally {
      signals.free();
    }
  });

  it("exposes callback why diagnostics for branch rewires and retained failures", async () => {
    const signals = createSignals();
    const enabled = signals.input("enabled", true);
    const name = signals.input("name", "Ada");
    const count = signals.input("count", 1);
    let shouldFail = false;
    const label = signals.computed(() => (enabled() ? name() : "disabled"));
    const fragile = signals.computed(() => {
      if (shouldFail) {
        const error = new Error("boom") as Error & { code?: string };
        error.code = "fragileBoom";
        throw error;
      }
      return count() * 2;
    });

    try {
      expect(label()).toBe("Ada");
      expect(fragile()).toBe(2);

      act(() => {
        enabled.set(false);
      });
      await flushQueuedDiagnostics();
      expect(label()).toBe("disabled");

      const labelWhy = signals.diagnostics().why(label.id);
      expect(labelWhy.apiFamily).toBe("computed");
      expect(labelWhy.recipeFamily).toBe("callback");
      expect(labelWhy.callback?.currentReads).toEqual(["enabled"]);
      expect(labelWhy.callback?.lastDependencyPatch?.previousReads).toEqual([
        "enabled",
        "name",
      ]);
      expect(labelWhy.callback?.lastDependencyPatch?.currentReads).toEqual(["enabled"]);
      expect(labelWhy.callback?.lastDependencyPatch?.removedCount).toBe(1);

      shouldFail = true;
      expect(() => {
        act(() => {
          count.set(2);
        });
      }).toThrow(/boom/);

      const fragileWhy = signals.diagnostics().why(fragile.id);
      expect(fragileWhy.callback?.lastFailure?.class).toBe("CallbackThrew");
      expect(fragileWhy.callback?.lastFailure?.message).toBe("boom");
      expect(fragileWhy.callback?.lastFailure?.code).toBe("fragileBoom");
    } finally {
      signals.free();
    }
  });

  it("denies callback computed reads from a different Signals runtime", () => {
    const left = createSignals();
    const right = createSignals();
    const count = left.input("count", 1);

    try {
      expect(() => right.computed("badCrossRuntime", () => count() * 2)).toThrow(
        /different Signals runtime/,
      );
    } finally {
      left.free();
      right.free();
    }
  });

  it("denies mutations during callback computed authoring", () => {
    const signals = createSignals();
    const count = signals.input("count", 1);

    try {
      expect(() =>
        signals.computed("badMutation", () => {
          count.set(2);
          return 1;
        }),
      ).toThrow(/cannot mutate signals or transactions/);
    } finally {
      signals.free();
    }
  });

  it("denies promise-returning callback computed authoring with a typed runtime failure", () => {
    const signals = createSignals();

    try {
      expect(() =>
        signals.computed("future", () => Promise.resolve(5) as unknown as number),
      ).toThrow(/returned a Promise/);

      const summary = signals.diagnostics().performanceSummary();
      expect(summary.computeCallbackFailureCount).toBeGreaterThanOrEqual(1);
    } finally {
      signals.free();
    }
  });

  it("surfaces output callback deferral explicitly on the package callback-first API", () => {
    const signals = createSignals();
    const count = signals.input("count", 1);

    try {
      const fromOutputForm = (() => {
        try {
          signals.output("panel", () => ({ count: count() }));
          return null;
        } catch (error) {
          return error;
        }
      })() as { code?: string; message?: string; context?: string | null } | null;

      expect(fromOutputForm?.code).toBe("outputCallbackDeferred");
      expect(fromOutputForm?.message).toMatch(/intentionally deferred/i);
      expect(fromOutputForm?.context).toBe("panel");

      const fromExplicitMethod = (() => {
        try {
          signals.outputCallback("panelToo", () => ({ count: count() }));
          return null;
        } catch (error) {
          return error;
        }
      })() as { code?: string; message?: string; context?: string | null } | null;

      expect(fromExplicitMethod?.code).toBe("outputCallbackDeferred");
      expect(fromExplicitMethod?.message).toMatch(/use outputSpec/i);
      expect(fromExplicitMethod?.context).toBe("panelToo");
    } finally {
      signals.free();
    }
  });

  it("keeps React values and shared fanout metrics aligned with committed runtime truth", async () => {
    const fixture = buildFixture();
    const renderCounts: RenderCounts = {};
    let rendered: ReturnType<typeof render> | undefined;

    try {
      rendered = render(
        <>
          <ValueProbe
            label="count-a"
            signal={fixture.count}
            store={fixture.store}
            renderCounts={renderCounts}
          />
          <ValueProbe
            label="count-b"
            signal={fixture.count}
            store={fixture.store}
            renderCounts={renderCounts}
          />
          <OutputProbe
            label="panel-a"
            output={fixture.panel}
            store={fixture.store}
            renderCounts={renderCounts}
          />
          <OutputProbe
            label="panel-b"
            output={fixture.panel}
            store={fixture.store}
            renderCounts={renderCounts}
          />
        </>,
      );

      expect(screen.getByTestId("count-a").textContent).toBe("1");
      expect(screen.getByTestId("count-b").textContent).toBe("1");
      expect(screen.getByTestId("panel-a").textContent).toBe(
        JSON.stringify({ count: 1, doubled: 2 }),
      );
      expect(screen.getByTestId("panel-b").textContent).toBe(
        JSON.stringify({ count: 1, doubled: 2 }),
      );

      expect(fixture.store.performanceSummary()).toEqual({
        activeSignalSubscriptionCount: 2,
        activeReactSubscriberCount: 4,
        activeRuntimeWatchHandleCount: 2,
        diagnosticsSubscriberCount: 0,
        sharedFanoutRatio: 2,
      });

      act(() => {
        fixture.signals.transaction((tx) => {
          tx.set(fixture.count, 3);
        });
      });
      await flushQueuedDiagnostics();

      expect(screen.getByTestId("count-a").textContent).toBe("3");
      expect(screen.getByTestId("count-b").textContent).toBe("3");
      expect(screen.getByTestId("panel-a").textContent).toBe(
        JSON.stringify({ count: 3, doubled: 6 }),
      );
      expect(screen.getByTestId("panel-b").textContent).toBe(
        JSON.stringify({ count: 3, doubled: 6 }),
      );
      expect(renderCounts["count-a"]).toBe(2);
      expect(renderCounts["count-b"]).toBe(2);
      expect(renderCounts["panel-a"]).toBe(2);
      expect(renderCounts["panel-b"]).toBe(2);
    } finally {
      rendered?.unmount();
      fixture.dispose();
    }
  });

  it("keeps diagnostics snapshots aligned with direct runtime diagnostics after committed transactions", async () => {
    const fixture = buildFixture();
    const renderCounts: RenderCounts = {};
    const diagnosticsCapture: DiagnosticsCapture = { current: null };
    let rendered: ReturnType<typeof render> | undefined;

    try {
      rendered = render(
        <>
          <ValueProbe
            label="count"
            signal={fixture.count}
            store={fixture.store}
            renderCounts={renderCounts}
          />
          <DiagnosticsProbe
            store={fixture.store}
            capture={diagnosticsCapture}
            renderCounts={renderCounts}
          />
        </>,
      );

      act(() => {
        fixture.signals.transaction((tx) => {
          tx.set(fixture.count, 5);
        });
      });
      await flushQueuedDiagnostics();

      expect(screen.getByTestId("count").textContent).toBe("5");

      const directDiagnostics = fixture.signals.diagnostics();
      expect(diagnosticsCapture.current?.latestObservation).toEqual(
        directDiagnostics.latestObservation(),
      );
      expect(diagnosticsCapture.current?.latestFlow).toEqual(
        directDiagnostics.latestFlow(),
      );
      expect(
        diagnosticsCapture.current?.performanceSummary.deliveredObservationCount,
      ).toBeGreaterThanOrEqual(1);
      expect(diagnosticsCapture.current?.performanceSummary.activeHandleCount).toBe(1);
    } finally {
      rendered?.unmount();
      fixture.dispose();
    }
  });

  it("suppresses rerenders and diagnostics churn for aborted transactions", async () => {
    const fixture = buildFixture();
    const renderCounts: RenderCounts = {};
    const diagnosticsCapture: DiagnosticsCapture = { current: null };
    let rendered: ReturnType<typeof render> | undefined;

    try {
      rendered = render(
        <>
          <ValueProbe
            label="count"
            signal={fixture.count}
            store={fixture.store}
            renderCounts={renderCounts}
          />
          <DiagnosticsProbe
            store={fixture.store}
            capture={diagnosticsCapture}
            renderCounts={renderCounts}
          />
        </>,
      );

      expect(screen.getByTestId("count").textContent).toBe("1");
      expect(renderCounts.count).toBe(1);

      expect(() => {
        act(() => {
          fixture.signals.transaction((tx) => {
            tx.set(fixture.count, 9);
            throw new Error("abort transaction");
          });
        });
      }).toThrow("abort transaction");

      await flushQueuedDiagnostics();

      expect(screen.getByTestId("count").textContent).toBe("1");
      expect(renderCounts.count).toBe(1);
      expect(renderCounts.diagnostics).toBe(1);
      expect(diagnosticsCapture.current?.latestObservation).toBeNull();
      expect(
        diagnosticsCapture.current?.performanceSummary.deliveredObservationCount,
      ).toBe(0);
      expect(
        diagnosticsCapture.current?.performanceSummary.rollbackSuppressedDeliveryCount,
      ).toBe(0);
    } finally {
      rendered?.unmount();
      fixture.dispose();
    }
  });

  it("tears down subscriptions under mount churn without resurrecting stale listeners", async () => {
    const fixture = buildFixture();
    const renderCounts: RenderCounts = {};

    function ChurnHarness({
      showCount,
      showPanel,
    }: {
      showCount: boolean;
      showPanel: boolean;
    }): JSX.Element {
      return (
        <>
          {showCount ? (
            <ValueProbe
              label="count"
              signal={fixture.count}
              store={fixture.store}
              renderCounts={renderCounts}
            />
          ) : null}
          {showPanel ? (
            <OutputProbe
              label="panel"
              output={fixture.panel}
              store={fixture.store}
              renderCounts={renderCounts}
            />
          ) : null}
        </>
      );
    }

    let rendered: ReturnType<typeof render> | undefined;
    try {
      rendered = render(<ChurnHarness showCount={true} showPanel={true} />);

      expect(fixture.store.performanceSummary().activeRuntimeWatchHandleCount).toBe(2);

      rendered.rerender(<ChurnHarness showCount={false} showPanel={false} />);
      await flushQueuedDiagnostics();

      expect(fixture.store.performanceSummary()).toEqual({
        activeSignalSubscriptionCount: 0,
        activeReactSubscriberCount: 0,
        activeRuntimeWatchHandleCount: 0,
        diagnosticsSubscriberCount: 0,
        sharedFanoutRatio: 0,
      });
      expect(
        fixture.signals.diagnostics().performanceSummary().activeHandleCount,
      ).toBe(0);

      act(() => {
        fixture.signals.transaction((tx) => {
          tx.set(fixture.count, 4);
        });
      });
      await flushQueuedDiagnostics();

      expect(renderCounts.count).toBe(1);
      expect(renderCounts.panel).toBe(1);

      rendered.rerender(<ChurnHarness showCount={true} showPanel={false} />);

      expect(fixture.store.performanceSummary()).toEqual({
        activeSignalSubscriptionCount: 1,
        activeReactSubscriberCount: 1,
        activeRuntimeWatchHandleCount: 1,
        diagnosticsSubscriberCount: 0,
        sharedFanoutRatio: 1,
      });

      act(() => {
        fixture.signals.transaction((tx) => {
          tx.set(fixture.count, 6);
        });
      });
      await flushQueuedDiagnostics();

      expect(screen.getByTestId("count").textContent).toBe("6");
      expect(renderCounts.count).toBe(3);
    } finally {
      rendered?.unmount();
      fixture.dispose();
    }
  });

  it("falls back to serial execution in the current web package even for wide graphs", () => {
    const fixture = buildLargeParallelFixture();

    try {
      expect(fixture.total.get()).toBe(2 * ((1024 * 1025) / 2));

      const summary = fixture.signals.diagnostics().performanceSummary();
      expect(summary.serialExecutorUsageCount).toBeGreaterThanOrEqual(1);
      expect(summary.parallelExecutorUsageCount).toBe(0);
    } finally {
      fixture.dispose();
    }
  });
});
