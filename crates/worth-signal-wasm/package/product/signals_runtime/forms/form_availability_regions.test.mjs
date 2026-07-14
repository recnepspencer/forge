import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form derives control group and section availability topology", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);
    const source = signals.input({
      title: "Ship docs",
      owner: "Ada",
      mode: "editing",
    });

    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
        owner: field("owner"),
        mode: field("mode"),
      }),
      availability: ({ control, group, section, action }) => ({
        saveControl: control("saveButton", ["mode"], (values, context) => {
          assert.equal(context.scope, "control");
          return values.mode === "readonly"
            ? { state: "disabled", reason: "save control is disabled" }
            : "enabled";
        }),
        detailsGroup: group("details", ["title", "owner"], ["mode"], (values, context) => {
          assert.deepEqual(context.dependencies, ["mode"]);
          return values.mode === "locked"
            ? { state: "blocked", reason: "details group is locked" }
            : "enabled";
        }),
        reviewSection: section("review", ["owner"], ["mode"], (values) => (
          values.mode === "review"
            ? { state: "unavailable", reason: "review section is unavailable" }
            : "enabled"
        )),
        submitAction: action("submit", ["mode"], (values) => (
          values.mode === "archived"
            ? { state: "blocked", reason: "archived records cannot submit" }
            : "enabled"
        )),
      }),
      steps: ({ step }) => ({
        titleStep: step("titleStep", ["title"], { order: 1 }),
        ownerStep: step("ownerStep", ["owner"], { order: 2 }),
      }),
    });

    assert.deepEqual(form.availability().summary.byScope, {
      field: 0,
      action: 1,
      control: 1,
      group: 1,
      section: 1,
    });
    assert.deepEqual(form.availability().counters, {
      costBasis: "derivedFullReportScan",
      incrementalStatus: "notIncremental",
      declarations: 4,
      dependencyReads: 4,
      fieldRegionMemberships: 3,
      blockingArtifacts: 0,
      scopeCounts: {
        field: 0,
        action: 1,
        control: 1,
        group: 1,
        section: 1,
      },
    });
    assert.deepEqual(
      form.availability().dependencyBreadth.find((entry) => entry.id === "group:details"),
      {
        id: "group:details",
        scope: "group",
        ownerId: "details",
        fields: ["title", "owner"],
        dependencies: ["mode"],
      },
    );

    form.fields.mode.set("locked");
    assert.equal(form.availability().counters.blockingArtifacts, 1);
    assert.deepEqual(
      form.steps().artifacts.map((step) => [
        step.id,
        step.readiness.blockers.map((blocker) => blocker.group ?? blocker.section ?? blocker.control ?? null),
      ]),
      [
        ["titleStep", ["details"]],
        ["ownerStep", ["details"]],
      ],
    );

    form.fields.mode.set("review");
    assert.deepEqual(
      form.steps().artifacts.map((step) => [
        step.id,
        step.readiness.blockers.map((blocker) => blocker.section ?? null),
      ]),
      [
        ["titleStep", []],
        ["ownerStep", ["review"]],
      ],
    );

    form.fields.mode.set("archived");
    assert.deepEqual(
      form.actionReadiness("submit").blockers.map((blocker) => blocker.action),
      ["submit"],
    );

    form.fields.mode.set("readonly");
    const saveControl = form.availability().artifacts.find((artifact) => artifact.ownerId === "saveButton");
    assert.equal(saveControl.state, "disabled");
    assert.equal(saveControl.scope, "control");
    assert.deepEqual(saveControl.fields, []);
    assert.equal(form.controlAvailability("saveButton")?.reason, "save control is disabled");
    assert.equal(form.controlAvailabilities().length, 1);
    assert.equal(form.diagnostics().availability.summary.byScope.control, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form denies invalid availability region declarations", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = createGraphOperationalRuntime();
    const signals = wrapSignals(rawSignals);

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          availability: ({ group }) => ({
            missing: group("details", ["missing"], ["title"], () => "enabled"),
          }),
        }),
      /availability region references an undeclared field/,
    );

    assert.throws(
      () =>
        signals.form({
          source: { title: "Ship docs" },
          fields: ({ field }) => ({
            title: field("title"),
          }),
          availability: ({ section }) => ({
            duplicate: section("details", ["title", "title"], ["title"], () => "enabled"),
          }),
        }),
      /availability region fields must be unique/,
    );

    const malformed = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      availability: ({ group }) => ({
        draftPolicy: group("details", ["title"], ["title"], () => ({
          state: "blocked",
          draftPolicy: "clear",
        })),
      }),
    });
    assert.throws(
      () => malformed.availability(),
      /availability draft policy only applies to fields/,
    );
  } finally {
    await cleanup();
  }
});
