import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { withSignals } from "../action_execution_test_helpers.mjs";
import { formsDocsRoot } from "./forms_docs_root.mjs";

const docsRoot = path.join(formsDocsRoot, "forms");

test("forms feature docs preserve first-batch guidance for core state patching and validation", async () => {
  const gettingStartedDoc = fs.readFileSync(
    path.join(docsRoot, "getting-started/your-first-form.md"),
    "utf8",
  );
  const patchingDoc = fs.readFileSync(
    path.join(docsRoot, "changes/patching-complex-edit-forms.md"),
    "utf8",
  );
  const asyncDoc = fs.readFileSync(
    path.join(docsRoot, "validation/async-validation.md"),
    "utf8",
  );
  const compatibilityDoc = fs.readFileSync(
    path.join(docsRoot, "validation/source-compatibility-and-draft-migration.md"),
    "utf8",
  );

  assert.match(gettingStartedDoc, /signals\.form\(/);
  assert.match(patchingDoc, /patchPlan\(\)\.operations/);
  assert.match(asyncDoc, /startAsyncValidation\(/);
  assert.match(asyncDoc, /fulfillAsyncValidation\(/);
  assert.match(compatibilityDoc, /sourceCompatibility\(\)/);
  assert.match(compatibilityDoc, /migrateDraft/);

  await withSignals((signals) => {
    const localForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    localForm.fields.title.set("Publish docs");
    assert.equal(localForm.effective().title, "Publish docs");
    assert.equal(localForm.dirty().isDirty, true);

    const patchForm = signals.form({
      source: {
        profile: { displayName: "Ship docs" },
        evidence: { digest: "file-0", name: "draft.pdf" },
      },
      fields: ({ field, evidence }) => ({
        displayName: field("profile.displayName"),
        evidence: evidence("evidence", { attachmentIdentity: "digest" }),
      }),
    });

    patchForm.fields.displayName.set("Published docs");
    patchForm.fields.evidence.set({ digest: "file-1", name: "audit.pdf" });
    assert.deepEqual(
      patchForm.patchPlan().operations.map((operation) => operation.kind),
      ["set", "attach"],
    );

    const asyncForm = signals.form({
      source: { slug: "ship-docs" },
      fields: ({ field }) => ({
        slug: field("slug"),
      }),
      validation: ({ asyncField }) => ({
        slugUnique: asyncField("slug", {
          id: "slugUnique",
          triggers: ["submit"],
        }),
      }),
    });

    const pending = asyncForm.startAsyncValidation("slugUnique");
    asyncForm.fulfillAsyncValidation(pending.operationId, {
      reason: "slug is unique",
    });
    assert.equal(asyncForm.asyncValidationHistory().at(-1)?.resultKind, "fulfilled");
  });
});
