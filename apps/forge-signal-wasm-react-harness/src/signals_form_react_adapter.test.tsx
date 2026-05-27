import React from "react";
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import {
  createFakeForm,
  createFakeStore,
  SignalsFormProbe,
} from "./form_react_adapter.fixture";

afterEach(() => {
  cleanup();
});

describe("useSignalsForm", () => {
  it("supports a first-class direct CRUD field and action binding lane", async () => {
    let formBuildCount = 0;
    const store = createFakeStore(() => {
      formBuildCount += 1;
      return createFakeForm(store);
    });

    render(<SignalsFormProbe store={store} />);

    const titleInput = screen.getByTestId("signals-title") as HTMLInputElement;
    const publishedInput = screen.getByTestId("signals-published") as HTMLInputElement;
    const roleSelect = screen.getByTestId("signals-role") as HTMLSelectElement;
    const appIdsSelect = screen.getByTestId("signals-appIds") as HTMLSelectElement;
    const submitButton = screen.getByTestId("signals-submit") as HTMLButtonElement;
    const resetButton = screen.getByTestId("signals-reset") as HTMLButtonElement;

    expect(formBuildCount).toBe(1);
    expect(store.performanceSummary().diagnosticsSubscriberCount).toBe(0);
    expect(store.performanceSummary().activeSignalSubscriptionCount).toBe(1);
    expect(titleInput.value).toBe("");
    expect(publishedInput.checked).toBe(false);
    expect(roleSelect.value).toBe("editor");
    expect([...appIdsSelect.selectedOptions]).toHaveLength(0);
    expect(screen.getByTestId("signals-dirty").textContent).toBe("false");
    expect(screen.getByTestId("signals-patch-empty").textContent).toBe("true");
    expect(submitButton.disabled).toBe(true);
    expect(screen.getByTestId("signals-submit-result-kind").textContent).toBe("none");

    await act(async () => {
      fireEvent.change(titleInput, { target: { value: "Northstar Jacket" } });
      fireEvent.change(roleSelect, { target: { value: "admin" } });
      fireEvent.change(appIdsSelect, {
        target: {
          value: ["northstar", "orbit"],
        },
      });
    });

    expect(titleInput.value).toBe("Northstar Jacket");
    expect(roleSelect.value).toBe("admin");
    expect(screen.getByTestId("signals-dirty").textContent).toBe("true");
    expect(screen.getByTestId("signals-effective-title").textContent).toBe("Northstar Jacket");
    expect(screen.getByTestId("signals-patch-empty").textContent).toBe("false");
    expect(submitButton.disabled).toBe(false);

    await act(async () => {
      fireEvent.blur(titleInput);
    });

    expect(screen.getByTestId("signals-title-messages").textContent).toBe("1");

    await act(async () => {
      publishedInput.click();
      submitButton.click();
    });

    expect(publishedInput.checked).toBe(true);
    expect(submitButton.disabled).toBe(true);
    expect(submitButton.textContent).toBe("pending");
    expect(screen.getByTestId("signals-submit-result-kind").textContent).toBe("pending");

    await act(async () => {
      resetButton.click();
    });

    expect(titleInput.value).toBe("");
    expect(publishedInput.checked).toBe(false);
    expect(roleSelect.value).toBe("editor");
    expect(screen.getByTestId("signals-dirty").textContent).toBe("false");
    expect(screen.getByTestId("signals-patch-empty").textContent).toBe("true");
    expect(screen.getByTestId("signals-submit-result-kind").textContent).toBe("none");
    expect(formBuildCount).toBe(1);
  });

  it("rebuilds the mounted signals form only when remountKey changes", async () => {
    let formBuildCount = 0;
    const store = createFakeStore(() => {
      formBuildCount += 1;
      return createFakeForm(store);
    });

    const rendered = render(<SignalsFormProbe store={store} remountKey="alpha" />);

    try {
      expect(formBuildCount).toBe(1);

      rendered.rerender(<SignalsFormProbe store={store} remountKey="alpha" />);
      expect(formBuildCount).toBe(1);

      rendered.rerender(<SignalsFormProbe store={store} remountKey="beta" />);
      expect(formBuildCount).toBe(2);
    } finally {
      rendered.unmount();
    }
  });
});
