import assert from "node:assert/strict";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { stripTypeScriptTypes } from "node:module";
import test from "node:test";
import { fileURLToPath, pathToFileURL } from "node:url";

import { withSignals } from "./action_execution_test_helpers.mjs";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const inputEventsPath = path.resolve(
  testDir,
  "..",
  "..",
  "..",
  "..",
  "react",
  "form_input_events.ts",
);

test("React text input commits the draft and applies declared client validation", async () => {
  const inputEvents = await loadInputEvents();
  try {
    await withSignals(async (signals) => {
      const form = signals.form({
        source: { email: "ada@example.com" },
        fields: ({ field }) => ({
          email: field("email"),
        }),
        validation: ({ field }) => ({
          email: field("email", (value) => (
            value.includes("@")
              ? true
              : invalidEmailArtifact()
          )),
        }),
        actions: ({ submit }) => ({
          submit: submit(),
        }),
      });

      inputEvents.commitTextInput(
        form.bindInput("email"),
        { currentTarget: { value: "not-an-email" } },
      );

      assert.equal(form.effective().email, "not-an-email");
      assert.equal(form.validation().summary.invalid, 1);
      assert.equal(form.visibleMessages()[0].code, "email.invalid");
      assert.equal(form.actionReadiness("submit").canRun, false);
    });
  } finally {
    await inputEvents.cleanup();
  }
});

function invalidEmailArtifact() {
  return {
    kind: "invalid",
    field: "email",
    message: {
      code: "email.invalid",
      message: "Enter a complete email.",
      severity: "error",
      target: "email",
      audience: "user",
      visibility: "visible",
    },
  };
}

async function loadInputEvents() {
  const tempDir = await mkdtemp(path.join(tmpdir(), "worth-form-input-events-"));
  try {
    const source = await readFile(inputEventsPath, "utf8");
    const transformed = stripTypeScriptTypes(source, { mode: "transform" });
    const outputPath = path.join(tempDir, "form_input_events.mjs");
    await writeFile(outputPath, transformed, "utf8");
    const loaded = await import(pathToFileURL(outputPath).href);
    return {
      ...loaded,
      cleanup: () => rm(tempDir, { recursive: true, force: true }),
    };
  } catch (error) {
    await rm(tempDir, { recursive: true, force: true });
    throw error;
  }
}
