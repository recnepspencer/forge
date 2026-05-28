import React from "react";
import { act, cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";
import { createReactSignalsStore, useSignalValue } from "@aust-group/forge-signal-wasm/react";

async function flushReact(): Promise<void> {
  await act(async () => {
    await Promise.resolve();
    await Promise.resolve();
  });
}

function DialogSummaryProbe({
  dialog,
  store,
}: {
  dialog: ReturnType<Awaited<ReturnType<typeof createSignals>>["local"]["dialogState"]>;
  store: ReturnType<typeof createReactSignalsStore>;
}): JSX.Element {
  const summary = useSignalValue<{
    readiness: {
      blockers: ReadonlyArray<{ kind: string }>;
    };
  }>(dialog.summarySignal(), store);
  const dirtyBlocked = summary.readiness.blockers.some((blocker) => blocker.kind === "dialog:dirty");
  return <div data-testid="dialog-summary">{dirtyBlocked ? "blocked" : "clear"}</div>;
}

afterEach(() => {
  cleanup();
});

describe("local dialog React reactivity", () => {
  it("updates React consumers from a bound real form summary path without snapshot instability", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const store = createReactSignalsStore(signals);
    const consoleError = vi.spyOn(console, "error").mockImplementation(() => {});

    try {
      const dialog = signals.local.dialogState({
        identity: "reactive-edit-product-dialog",
        initial: {
          isOpen: true,
          mode: "edit" as const,
        },
      });
      const form = signals.form({
        source: {
          title: "",
          status: "draft",
        },
        fields: ({ field }) => ({
          title: field("title"),
          status: field("status"),
        }),
      });
      dialog.bindForm(form, {
        closeOnSuccess: true,
      });

      const rendered = render(
        <DialogSummaryProbe dialog={dialog} store={store} />,
      );

      expect(rendered.getByTestId("dialog-summary").textContent).toBe("clear");

      await act(async () => {
        await form.fields.title.set("changed");
      });
      await flushReact();
      expect(rendered.getByTestId("dialog-summary").textContent).toBe("blocked");

      act(() => {
        form.fields.title.clearDraft();
      });
      await flushReact();
      expect(rendered.getByTestId("dialog-summary").textContent).toBe("clear");

      expect(
        consoleError.mock.calls.some((call) =>
          call.some(
            (entry) =>
              typeof entry === "string"
              && entry.includes("getSnapshot should be cached"),
          ),
        ),
      ).toBe(false);
      expect(
        consoleError.mock.calls.some((call) =>
          call.some(
            (entry) =>
              typeof entry === "string"
              && entry.includes("Maximum update depth exceeded"),
          ),
        ),
      ).toBe(false);

      rendered.unmount();
    } finally {
      consoleError.mockRestore();
      store.dispose();
      signals.free();
    }
  });

  it("allows watch callbacks to read dialog summary truth after a bound real form mutation", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    try {
      const dialog = signals.local.dialogState({
        identity: "watched-edit-product-dialog",
        initial: {
          isOpen: true,
          mode: "edit" as const,
        },
      });
      const form = signals.form({
        source: {
          title: "",
          status: "draft",
        },
        fields: ({ field }) => ({
          title: field("title"),
          status: field("status"),
        }),
      });
      dialog.bindForm(form, {
        closeOnSuccess: true,
      });

      const observedDirtyStates: boolean[] = [];
      const watchHandle = signals.watch(dialog.summarySignal(), () => {
        const summary = dialog.summarySignal().get() as {
          readiness: {
            blockers: ReadonlyArray<{ kind: string }>;
          };
        };
        observedDirtyStates.push(
          summary.readiness.blockers.some((blocker) => blocker.kind === "dialog:dirty"),
        );
      });

      try {
        await act(async () => {
          await form.fields.title.set("changed");
        });
        await flushReact();

        act(() => {
          form.fields.title.clearDraft();
        });
        await flushReact();

        expect(observedDirtyStates).toContain(true);
        expect(observedDirtyStates.at(-1)).toBe(false);
      } finally {
        watchHandle[Symbol.dispose]();
      }
    } finally {
      signals.free();
    }
  });
});
