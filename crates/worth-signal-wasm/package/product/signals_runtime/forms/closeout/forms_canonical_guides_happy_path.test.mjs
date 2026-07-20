import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { withSignals } from "../action_execution_test_helpers.mjs";
import { createDetailPatchLineFixture } from "../resource_source/fixtures/resource_line_fixture.mjs";
import { formsDocsRoot } from "./forms_docs_root.mjs";

const guidePaths = Object.freeze({
  local: "forms/getting-started/your-first-form.md",
  submit: "forms/actions/README.md",
  resource: "forms/resource-backed/README.md",
  asyncValidation: "forms/validation/README.md",
  layout: "forms/layout/layout-measurement.md",
  collaboration: "forms/collaboration/collaboration-overview.md",
});

test("canonical forms guides cover local, resource, async, host, collaboration, and submit entrypoints", async () => {
  const guides = Object.fromEntries(Object.entries(guidePaths).map(([name, docPath]) => [
    name,
    fs.readFileSync(path.join(formsDocsRoot, docPath), "utf8"),
  ]));

  assert.match(guides.local, /signals\.form\(/);
  assert.match(guides.submit, /executeAction\("submit"\)/);
  assert.match(guides.resource, /signals\.form\.source\.resourceLine/);
  assert.match(guides.asyncValidation, /startAsyncValidation/);
  assert.match(guides.layout, /recordLayoutMeasurement/);
  assert.match(guides.collaboration, /reportCollaboration/);

  await withSignals((signals) => {
    const source = signals.input({ title: "Ship docs", done: false });
    const ordinaryForm = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
        done: field("done"),
      }),
    });
    ordinaryForm.fields.title.set("Ship docs today");
    assert.equal(ordinaryForm.effective().title, "Ship docs today");

    const submitForm = signals.form({
      source: { title: "", status: "draft" },
      fields: ({ field }) => ({
        title: field("title"),
        status: field("status"),
      }),
      validation: ({ field }) => ({
        titleRequired: field("title", (value) => (
          value.length > 0 || {
            kind: "invalid",
            message: {
              code: "title.required",
              severity: "error",
              audience: "user",
              visibility: "visible",
            },
          }
        )),
      }),
      actions: ({ submit }) => ({ submit: submit() }),
    });
    submitForm.fields.title.set("Ship docs");
    const pending = submitForm.executeAction("submit");
    assert.equal(pending.resultKind, "pending");
    submitForm.fulfillAction(pending.operationId, {
      canonicalValue: { title: "Ship docs", status: "published" },
    });
    assert.equal(submitForm.source().status, "published");

    const resourceLine = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: { title: "Ship docs" },
    });
    const resourceForm = signals.form({
      source: signals.form.source.resourceLine(resourceLine, { id: "recipes-resource" }),
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          resourceEffectProfile: signals.resource.effects.branchNative(),
        }),
      }),
    });
    assert.equal(resourceForm.resourceSource()?.sourceKind, "resourceLine");

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
    const validation = asyncForm.startAsyncValidation("slugUnique");
    asyncForm.fulfillAsyncValidation(validation.operationId, {
      reason: "slug is unique",
    });
    assert.equal(asyncForm.asyncValidationHistory().at(-1)?.resultKind, "fulfilled");

    const hostForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title", { row: "hero" }),
      }),
      host: {
        focus: "title",
        online: true,
        viewport: { width: 1280, height: 720 },
      },
      measurement: {
        observe: ["animationFrame"],
      },
    });
    hostForm.recordLayoutMeasurement([{ row: "hero", controlHeight: 32 }], {
      cause: "animationFrame",
    });
    assert.equal(hostForm.host().facts.online.state, "online");
    assert.equal(hostForm.layoutMeasurement().latestSnapshot?.rows[0]?.row, "hero");

    const collaborationForm = signals.form({
      source: { title: "Ship docs" },
      collaboration: {
        mode: "fieldLease",
        actorId: "me",
        supportsPresence: true,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });
    collaborationForm.reportCollaboration({
      posture: "blocked",
      leasedFields: [{ field: "title", ownerId: "peer-1" }],
      reason: "peer-1 owns the title lease",
    });
    assert.equal(collaborationForm.fieldWritePosture("title").canWrite, false);
  });
});
