import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("wrapSignals accepts string-valued metadata-style inputs without misparsing them as id-first authoring", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawSignals = {
      input(id, initial, options) {
        calls.push(["input", id, initial, options]);
        return createRawReadableHandle(id, initial);
      },
      computedSpec() {
        throw new Error("computedSpec not needed");
      },
      computedCallback() {
        throw new Error("computedCallback not needed");
      },
      outputSpec() {
        throw new Error("outputSpec not needed");
      },
      read(target) {
        return typeof target === "string" ? target : target.id;
      },
      watch() {
        throw new Error("watch not needed");
      },
      effect() {
        throw new Error("effect not needed");
      },
      transaction() {
        throw new Error("transaction not needed");
      },
      batch() {
        throw new Error("batch not needed");
      },
      nuke() {
        return true;
      },
      diagnostics() {
        throw new Error("diagnostics not needed");
      },
      history() {
        throw new Error("history not needed");
      },
      specialist() {
        throw new Error("specialist not needed");
      },
      adapters() {
        throw new Error("adapters not needed");
      },
      compatibilityApp() {
        throw new Error("compatibilityApp not needed");
      },
      compatibilityRuntime() {
        throw new Error("compatibilityRuntime not needed");
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const emptyStringInput = signals.spec.input("emptyStringInput", "");
    const namedStringInput = signals.spec.input("name", "Ada");
    const objectWithOwnIdValue = signals.input(
      { id: "gear-7", name: "Gear 7" },
      { debugName: "draft" },
    );

    assert.equal(emptyStringInput.id, "emptyStringInput");
    assert.equal(emptyStringInput(), "");
    assert.equal(namedStringInput.id, "name");
    assert.equal(namedStringInput(), "Ada");
    assert.notEqual(objectWithOwnIdValue.id, "gear-7");
    assert.deepEqual(objectWithOwnIdValue(), { id: "gear-7", name: "Gear 7" });

    assert.deepEqual(calls, [
      ["input", "emptyStringInput", "", undefined],
      ["input", "name", "Ada", undefined],
      ["input", objectWithOwnIdValue.id, { id: "gear-7", name: "Gear 7" }, {}],
    ]);
  } finally {
    await cleanup();
  }
});

test("scoped input accepts ordinary string values without treating them as explicit local ids", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawSignals = {
      input(id, initial, options) {
        calls.push(["input", id, initial, options]);
        return createRawReadableHandle(id, initial);
      },
      computedSpec() {
        throw new Error("computedSpec not needed");
      },
      computedCallback() {
        throw new Error("computedCallback not needed");
      },
      outputSpec() {
        throw new Error("outputSpec not needed");
      },
      read(target) {
        return typeof target === "string" ? target : target.id;
      },
      watch() {
        throw new Error("watch not needed");
      },
      effect() {
        throw new Error("effect not needed");
      },
      transaction() {
        throw new Error("transaction not needed");
      },
      batch() {
        throw new Error("batch not needed");
      },
      nuke() {
        return true;
      },
      diagnostics() {
        throw new Error("diagnostics not needed");
      },
      history() {
        throw new Error("history not needed");
      },
      specialist() {
        throw new Error("specialist not needed");
      },
      adapters() {
        throw new Error("adapters not needed");
      },
      compatibilityApp() {
        throw new Error("compatibilityApp not needed");
      },
      compatibilityRuntime() {
        throw new Error("compatibilityRuntime not needed");
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const scope = signals.scope("editSession");
    const empty = scope.input("", { debugName: "empty" });
    const value = scope.input("value", { debugName: "value" });

    assert.equal(empty(), "");
    assert.equal(value(), "value");
    assert.notEqual(empty.id, "editSession");
    assert.notEqual(value.id, "value");
    assert.deepEqual(
      calls.map(([kind, id, initial]) => [kind, id, initial]),
      [
        ["input", empty.id, ""],
        ["input", value.id, "value"],
      ],
    );
  } finally {
    await cleanup();
  }
});


