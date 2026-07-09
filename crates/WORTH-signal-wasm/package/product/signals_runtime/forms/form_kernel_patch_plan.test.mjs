import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form creates semantic dirty and patch artifacts without mutating the source", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);
    const source = signals.input({
      id: "task-7",
      title: "Ship docs",
      nested: {
        count: 1,
      },
      tags: ["regulated"],
    });

    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
        count: field("nested.count", {
          parse: (rawValue) => Number(rawValue),
        }),
        firstTag: field(["tags", 0], { id: "firstTag" }),
      }),
    });

    assert.equal(form.dirty().isDirty, false);
    assert.equal(form.patchPlan().empty, true);
    assert.equal(form.readiness().canSubmit, false);
    assert.deepEqual(source(), {
      id: "task-7",
      title: "Ship docs",
      nested: {
        count: 1,
      },
      tags: ["regulated"],
    });

    form.fields.title.set("Ready to ship");
    assert.equal(form.dirty().isDirty, true);
    assert.deepEqual(form.effective(), {
      id: "task-7",
      title: "Ready to ship",
      nested: {
        count: 1,
      },
      tags: ["regulated"],
    });
    assert.deepEqual(source(), {
      id: "task-7",
      title: "Ship docs",
      nested: {
        count: 1,
      },
      tags: ["regulated"],
    });
    assert.deepEqual(stripPatchEquality(form.patchPlan().operations), [
      {
        kind: "set",
        field: "title",
        locus: {
          path: "title",
          segments: ["title"],
        },
        value: "Ready to ship",
        valueDigest: JSON.stringify("Ready to ship"),
      },
    ]);
    assert.equal(form.patchPlan().breadth.sourceSnapshots, 1);
    assert.equal(form.patchPlan().breadth.effectiveSnapshots, 1);
    assert.equal(form.readiness().canSubmit, true);

    form.fields.title.set("Ship docs");
    assert.equal(form.dirty().isDirty, false);
    assert.equal(form.patchPlan().empty, true);
    assert.equal(form.readiness().canSubmit, false);

    form.fields.count.input("2");
    assert.equal(form.fields.count.diagnostics().pendingRawInput, true);
    assert.equal(form.patchPlan().empty, true);
    assert.equal(form.patchPlan().breadth.comparedFields, 2);
    assert.equal(form.patchPlan().breadth.skippedRawInputFields, 1);
    assert.deepEqual(form.patchPlan().blocked, [
      {
        kind: "uncommittedRawInput",
        field: "count",
        reason: "raw input has not crossed a parse/commit boundary",
      },
    ]);
    assert.equal(form.readiness().canSubmit, false);

    form.fields.count.commitInput();
    assert.equal(form.readiness().canSubmit, true);
    assert.deepEqual(stripPatchEquality(form.patchPlan().operations), [
      {
        kind: "set",
        field: "count",
        locus: {
          path: "nested.count",
          segments: ["nested", "count"],
        },
        value: 2,
        valueDigest: "2",
      },
    ]);

    form.fields.firstTag.set("released");
    assert.deepEqual(
      form.patchPlan().operations.map((operation) => operation.field),
      ["count", "firstTag"],
    );
  } finally {
    await cleanup();
  }
});

test("signals.form exposes deep collection equality breadth without repeated source snapshots", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const source = signals.input({
      title: "Ship docs",
      collection: Array.from({ length: 48 }, (_, index) => ({
        id: `item-${index}`,
        nested: {
          count: index,
          label: `Item ${index}`,
        },
      })),
    });

    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
        collection: field("collection"),
      }),
    });

    assert.equal(form.dirty().isDirty, false);
    assert.equal(form.dirty().breadth.sourceSnapshots, 1);
    assert.equal(form.dirty().breadth.effectiveSnapshots, 1);
    assert.equal(form.dirty().equality.fieldComparisons, 2);
    assert.equal(form.dirty().equality.deepCollectionFields, 1);
    assert.ok(form.dirty().equality.arrayEntries >= 48);

    const nextCollection = source().collection.slice();
    nextCollection[47] = {
      ...nextCollection[47],
      nested: {
        ...nextCollection[47].nested,
        label: "Updated item",
      },
    };
    form.fields.collection.set(nextCollection);
    const patchPlan = form.patchPlan();
    assert.equal(patchPlan.breadth.sourceSnapshots, 1);
    assert.equal(patchPlan.breadth.effectiveSnapshots, 1);
    assert.equal(patchPlan.equality.deepCollectionFields, 1);
    assert.equal(patchPlan.operations[0].field, "collection");
    assert.ok(patchPlan.operations[0].equality.arrayEntries >= 48);
    assert.equal(form.fields.collection.dirty().equality.costBasis, "fieldLocusStructuralCompare");
  } finally {
    await cleanup();
  }
});

test("signals.form compares non-record field values semantically instead of treating keyless objects as equal", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: {
        reviewedAt: new Date("2026-01-01T00:00:00.000Z"),
        approvals: new Map([["qa", "pending"]]),
      },
      fields: ({ field }) => ({
        reviewedAt: field("reviewedAt"),
        approvals: field("approvals"),
      }),
    });

    form.fields.reviewedAt.set(new Date("2026-01-02T00:00:00.000Z"));
    form.fields.approvals.set(new Map([["qa", "approved"]]));

    const patchPlan = form.patchPlan();
    assert.deepEqual(
      patchPlan.operations.map((operation) => operation.field),
      ["reviewedAt", "approvals"],
    );
    assert.match(patchPlan.operations[0].valueDigest, /"WORTHFormValueType":"Date"/);
    assert.match(patchPlan.operations[1].valueDigest, /"WORTHFormValueType":"Map"/);
  } finally {
    await cleanup();
  }
});

test("signals.form emits attachment attach detach posture and preserves declared resource loci in field diagnostics", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: {
        profile: {
          displayName: "Ship docs",
        },
        evidence: { digest: "file-0", name: "draft.pdf" },
      },
      fields: ({ field, evidence }) => ({
        displayName: field("profile.displayName", {
          resourceLocus: { kind: "jsonPath", path: "$.profile.displayName" },
        }),
        evidence: evidence("evidence", {
          attachmentIdentity: "digest",
          resourceLocus: { kind: "region", region: "evidenceRegion" },
        }),
      }),
    });

    form.fields.displayName.set("Published docs");
    form.fields.evidence.set({ digest: "file-1", name: "audit.pdf" });
    assert.deepEqual(stripPatchEquality(form.patchPlan().operations), [
      {
        kind: "set",
        field: "displayName",
        locus: {
          path: "profile.displayName",
          segments: ["profile", "displayName"],
        },
        value: "Published docs",
        valueDigest: JSON.stringify("Published docs"),
      },
      {
        kind: "attach",
        field: "evidence",
        locus: {
          path: "evidence",
          segments: ["evidence"],
        },
        value: { digest: "file-1", name: "audit.pdf" },
        valueDigest: JSON.stringify({ digest: "file-1", name: "audit.pdf" }),
      },
    ]);
    assert.equal(form.fieldContract()[0]?.resourceLocus?.kind, "jsonPath");
    assert.equal(form.fieldContract()[1]?.family, "evidence");
    assert.equal(form.fieldContract()[1]?.resourceLocus?.kind, "region");

    form.fields.evidence.set(null);
    assert.deepEqual(stripPatchEquality(form.patchPlan().operations), [
      {
        kind: "set",
        field: "displayName",
        locus: {
          path: "profile.displayName",
          segments: ["profile", "displayName"],
        },
        value: "Published docs",
        valueDigest: JSON.stringify("Published docs"),
      },
      {
        kind: "detach",
        field: "evidence",
        locus: {
          path: "evidence",
          segments: ["evidence"],
        },
      },
    ]);
    assert.equal(form.fields.evidence.attachmentIdentity(), null);
    assert.equal(form.fields.evidence.diagnostics().attachment, null);
  } finally {
    await cleanup();
  }
});

test("signals.form rejects unsafe declarations and reports non-native input adapters", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    assert.throws(
      () =>
        signals.form({
          source: {},
          fields: ({ field }) => ({
            polluted: field("__proto__.polluted"),
          }),
        }),
      /unsafe object segment/,
    );

    assert.throws(
      () =>
        signals.form({
          source: {},
          fields: ({ field }) => ({
            first: field("name", { id: "name" }),
            second: field("displayName", { id: "name" }),
          }),
        }),
      /field ids must be unique/,
    );

    const form = signals.form({
      source: {
        title: "Ship docs",
      },
      fields: ({ field }) => ({
        title: field("title", {
          adapter: {
            tier: "externalImperative",
            reportsComposition: false,
            reportsFocus: false,
          },
        }),
      }),
    });

    assert.deepEqual(form.fields.title.locus(), {
      field: "title",
      path: "title",
      segments: ["title"],
    });
    assert.deepEqual(
      form.fields.title.diagnostics().inputAdapter.unavailable,
      [
        {
          capability: "reportsComposition",
          reason: "externalImperative adapter did not declare reportsComposition",
        },
        {
          capability: "reportsFocus",
          reason: "externalImperative adapter did not declare reportsFocus",
        },
      ],
    );
    const inputCapabilities = form.inputCapabilities();
    assert.equal(inputCapabilities.summary.unavailableFields, 1);
    assert.equal(inputCapabilities.fields[0].posture, "unavailable");
    assert.deepEqual(
      inputCapabilities.fields[0].unavailableCapabilities,
      [
        {
          capability: "reportsComposition",
          reason: "externalImperative adapter did not declare reportsComposition",
        },
        {
          capability: "reportsFocus",
          reason: "externalImperative adapter did not declare reportsFocus",
        },
      ],
    );
  } finally {
    await cleanup();
  }
});

function stripPatchEquality(operations) {
  return operations.map(({ equality: _equality, ...operation }) => operation);
}
