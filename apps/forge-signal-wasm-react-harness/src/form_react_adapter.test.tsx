import React from "react";
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import {
  createFakeForm,
  createFakeStore,
  FormFieldProbe,
} from "./form_react_adapter.fixture";

afterEach(() => {
  cleanup();
});

describe("form React adapter hooks", () => {
  it("binds text and checkbox fields through the form runtime surface", async () => {
    const store = createFakeStore();
    const form = createFakeForm(store);

    render(<FormFieldProbe form={form} store={store} />);

    const titleInput = screen.getByTestId("title") as HTMLInputElement;
    const publishedInput = screen.getByTestId("published") as HTMLInputElement;

    expect(titleInput.value).toBe("");
    expect(publishedInput.checked).toBe(false);

    await act(async () => {
      fireEvent.change(titleInput, { target: { value: "Northstar Jacket" } });
    });
    expect(titleInput.value).toBe("Northstar Jacket");
    expect(screen.getByTestId("title-dirty").textContent).toBe("true");

    await act(async () => {
      fireEvent.blur(titleInput);
    });
    expect(screen.getByTestId("title-messages").textContent).toBe("1");

    await act(async () => {
      publishedInput.click();
    });
    expect(publishedInput.checked).toBe(true);
  });

  it("derives disabled and pending submit state from form action surfaces", async () => {
    const store = createFakeStore();
    const form = createFakeForm(store);

    render(<FormFieldProbe form={form} store={store} />);

    const titleInput = screen.getByTestId("title") as HTMLInputElement;
    const submitButton = screen.getByTestId("submit") as HTMLButtonElement;
    expect(submitButton.disabled).toBe(true);
    expect(submitButton.textContent).toBe("ready");
    expect(screen.getByTestId("submit-result-kind").textContent).toBe("none");

    await act(async () => {
      fireEvent.change(titleInput, { target: { value: "Northstar Jacket" } });
    });

    expect(submitButton.disabled).toBe(false);

    await act(async () => {
      submitButton.click();
    });

    expect(submitButton.disabled).toBe(true);
    expect(submitButton.textContent).toBe("pending");
    expect(screen.getByTestId("submit-result-kind").textContent).toBe("pending");
  });
});
