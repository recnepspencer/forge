import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import { withSignals } from "../action_execution_test_helpers.mjs";
import {
  createDetailPatchLineFixture,
  createReadOnlyResourceLineFixture,
} from "../resource_source/fixtures/resource_line_fixture.mjs";
import { formsDocsRoot } from "./forms_docs_root.mjs";

const docsRoot = path.join(formsDocsRoot, "forms");

test("forms feature docs preserve entry bootstrap, resource transfer, and verification guidance", async () => {
  const presentationDoc = fs.readFileSync(
    path.join(docsRoot, "presentation-and-external-lanes.md"),
    "utf8",
  );
  const resourceDoc = fs.readFileSync(
    path.join(docsRoot, "resource-line-forms.md"),
    "utf8",
  );
  const diagnosticsDoc = fs.readFileSync(
    path.join(docsRoot, "diagnostics-history-and-verification.md"),
    "utf8",
  );

  assert.match(presentationDoc, /presentationLifecycle\("entry"\)/);
  assert.match(presentationDoc, /layoutMeasurement/i);
  assert.match(resourceDoc, /attachmentTransfers\(\)/);
  assert.match(resourceDoc, /mapping-unavailable/i);
  assert.match(diagnosticsDoc, /semanticEqualityDigest/);
  assert.match(diagnosticsDoc, /performanceEnvelope/);

  await withSignals((signals) => {
    const localForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title", { row: "hero" }),
      }),
      presentation: {
        entry: {
          bootstrap: {
            layoutMeasurement: true,
          },
        },
      },
    });

    assert.equal(localForm.presentationLifecycle("entry").status, "pending");
    localForm.recordLayoutMeasurement([{ row: "hero", controlHeight: 32 }], {
      cause: "animationFrame",
    });
    assert.equal(localForm.presentationLifecycle("entry").status, "ready");

    const detailLine = createDetailPatchLineFixture({
      effectProfile: signals.resource.effects.branchNative(),
      initialValue: { title: "Ship docs" },
    });
    const resourceForm = signals.form({
      source: signals.form.source.resourceLine(detailLine, { id: "doc-resource-form" }),
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
    assert.equal(typeof resourceForm.verification().digests.resourceSourceDigest, "string");

    const noTransferForm = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: { kind: "fulfilled", operation: "initialLoad" },
          freshness: { kind: "fresh" },
        }),
        { id: "doc-read-only-line" },
      ),
      fields: ({ evidence }) => ({
        evidence: evidence("evidence", { attachmentIdentity: "digest" }),
      }),
    });

    assert.ok([
      "outsideTransferSurface",
      "mappingUnavailable",
      "noAttachment",
      "resourceTransfer",
    ].includes(noTransferForm.attachmentTransfers().fields[0]?.bindingKind ?? ""));
  });
});
