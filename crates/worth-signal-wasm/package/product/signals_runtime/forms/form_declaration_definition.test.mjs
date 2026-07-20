import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "./action_execution_test_helpers.mjs";

test("signals.form.define preserves a reusable declaration without creating form authority", async () => {
  await withSignals((signals) => {
    const source = signals.input({ email: "ada@example.com" });
    const declaration = signals.form.define({
      id: "profile-editor",
      source: signals.form.source.signal(source, { id: "profile" }),
      fields: ({ field }) => ({
        email: field("email"),
      }),
    });

    assert.equal(Object.isFrozen(declaration), true);
    assert.equal(declaration.id, "profile-editor");
    assert.equal(typeof declaration.fields, "function");

    const form = signals.form(declaration);
    assert.equal(form.field("email").value(), "ada@example.com");
  });
});

test("signals.form.define denies non-object declarations before controller construction", async () => {
  await withSignals((signals) => {
    assert.throws(
      () => signals.form.define(null),
      /expects a form declaration object/,
    );
  });
});
