import React from "react";
import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { createSignals } from "@aust-group/forge-signal-wasm";
import {
  createReactSignalsStore,
  ReactSignalsStoreProvider,
  useBrowserHistoryStory,
  useSignalsHistory,
} from "@aust-group/forge-signal-wasm/react";

function HistoryProbe({
  history,
}: {
  history: ReturnType<Awaited<ReturnType<typeof createSignals>>["history"]>;
}): JSX.Element {
  const view = useSignalsHistory(history);
  return (
    <div data-testid="history-proof">
      {JSON.stringify({
        currentBranch: view.currentBranch.name,
        branchCount: view.branches.length,
        branchNames: view.branches.map((branch) => branch.name),
        canUndo: view.canUndo,
        canRedo: view.canRedo,
      })}
    </div>
  );
}

function StoryProbe({
  story,
}: {
  story: ReturnType<Awaited<ReturnType<typeof createSignals>>["router"]["browserHistory"]["story"]>;
}): JSX.Element {
  const view = useBrowserHistoryStory(story);
  return (
    <div data-testid="story-proof">
      {JSON.stringify({
        currentRouteId: view.current?.routeId ?? null,
        entryCount: view.entries.length,
        eventCount: view.events.length,
        breadcrumbLabels: view.breadcrumbTrail.entries.map((entry) => entry.label),
        hasBackProvenance: view.backProvenance.available,
      })}
    </div>
  );
}

afterEach(() => {
  cleanup();
});

describe("history react adapter", () => {
  it("renders branch truth from useSignalsHistory without local mirror state", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const store = createReactSignalsStore(signals);
    const history = signals.history();
    const rendered = render(
      <ReactSignalsStoreProvider store={store}>
        <HistoryProbe history={history} />
      </ReactSignalsStoreProvider>,
    );

    try {
      let createdBranch: ReturnType<typeof history.create_branch>;
      act(() => {
        createdBranch = history.create_branch("feature-review");
      });
      await waitFor(() => {
        const text = screen.getByTestId("history-proof").textContent ?? "";
        expect(text).toContain('"branchCount":2');
        expect(text).toContain('"feature-review"');
        expect(text).toContain('"canRedo":true');
      });

      act(() => {
        history.switch_branch((createdBranch as ReturnType<typeof history.create_branch>).id);
      });

      await waitFor(() => {
        const text = screen.getByTestId("history-proof").textContent ?? "";
        expect(text).toContain('"currentBranch":"feature-review"');
        expect(text).toContain('"canUndo":true');
        expect(text).toContain('"canRedo":false');
      });
    } finally {
      rendered.unmount();
      store.dispose();
      signals.free();
    }
  });

  it("renders router story truth from useBrowserHistoryStory without manual React mirrors", async () => {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const story = signals.router.browserHistory.story();
    const routes = signals.router.define({
      home: signals.router.route("/"),
      products: signals.router.route("/products"),
    });
    const rendered = render(<StoryProbe story={story} />);

    try {
      const homeReport = await routes.admitBrowserHistoryIngress(
        signals.router.browserHistory.load("/"),
      );
      act(() => {
        story.record(homeReport);
      });

      await waitFor(() => {
        expect(screen.getByTestId("story-proof").textContent).toContain('"currentRouteId":"home"');
      });

      const productsReport = await routes.admitBrowserHistoryIngress(
        signals.router.browserHistory.push("/products"),
      );
      act(() => {
        story.record(productsReport);
      });

      await waitFor(() => {
        const text = screen.getByTestId("story-proof").textContent ?? "";
        expect(text).toContain('"currentRouteId":"products"');
        expect(text).toContain('"entryCount":2');
        expect(text).toContain('"hasBackProvenance":true');
      });
    } finally {
      rendered.unmount();
      signals.free();
    }
  });
});
