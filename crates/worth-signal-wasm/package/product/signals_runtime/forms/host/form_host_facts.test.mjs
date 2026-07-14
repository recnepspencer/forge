import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form host report is explicit and reusable across validation and availability", async () => {
  const loaded = await loadSignalsModule();
  const {
    cleanup,
    hostCapabilityPlan,
    onlineCapability,
    persistenceCapability,
    viewportCapability,
    visibilityCapability,
    wrapSignals,
  } = loaded;
  try {
    const state = {
      online: false,
      visible: true,
      viewport: { width: 1280, height: 720 },
      persistedDraft: { revision: 3 },
      credentialsAvailable: false,
      autofillAvailable: true,
      focusedField: "title",
    };
    const signals = wrapSignals(createGraphOperationalRuntime(), {
      hostCapabilities: hostCapabilityPlan({
        online: onlineCapability({
          source: {
            current() {
              return state.online;
            },
            subscribe() {
              return () => {};
            },
          },
        }),
        visibility: visibilityCapability({
          source: {
            current() {
              return state.visible;
            },
            subscribe() {
              return () => {};
            },
          },
        }),
        viewport: viewportCapability({
          source: {
            current() {
              return state.viewport;
            },
            subscribe() {
              return () => {};
            },
          },
        }),
        persistence: persistenceCapability({
          source: {
            current() {
              return state.persistedDraft;
            },
          },
        }),
      }),
    });

    const form = signals.form({
      source: { title: "Ship docs" },
      host: {
        focus: () => state.focusedField,
        visibility: () => (state.visible ? "visible" : "hidden"),
        viewport: signals.host.viewport,
        online: signals.host.online,
        persistence: signals.host.persistence,
        credentials: () => state.credentialsAvailable,
        autofill: () => state.autofillAvailable,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      validation: ({ form: validateForm }) => ({
        hostVisibility: validateForm("hostVisibility", ["title"], (_, context) => (
          context.form.host().facts.visibility.state === "hidden"
            ? {
                kind: "warning",
                message: {
                  code: "host.hidden",
                  severity: "warning",
                  audience: "user",
                  visibility: "summary",
                },
              }
            : true
        )),
      }),
      availability: ({ action }) => ({
        publishAvailability: action("publish", ["title"], (_, context) => (
          context.form.host().facts.credentials.available === false
            ? {
                state: "blocked",
                reason: "credentials capability must be available before publish",
              }
            : "enabled"
        )),
      }),
    });

    const host = form.host();
    assert.equal(host.facts.focus.focusedField, "title");
    assert.equal(host.facts.online.state, "offline");
    assert.equal(host.facts.visibility.state, "visible");
    assert.deepEqual(host.facts.viewport.size, { width: 1280, height: 720 });
    assert.equal(host.facts.persistence.available, true);
    assert.equal(host.facts.credentials.available, false);
    assert.equal(host.facts.autofill.available, true);
    assert.equal(form.validation().host.digest, host.digest);
    assert.equal(form.availability().host.digest, host.digest);
    assert.equal(form.availability().artifacts[0].state, "blocked");

    state.visible = false;
    assert.equal(form.validation().summary.warning, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form host requirements deny submit and action plans before effects", async () => {
  const loaded = await loadSignalsModule();
  const {
    cleanup,
    wrapSignals,
  } = loaded;
  try {
    const state = {
      online: false,
      persistedDraft: { revision: 1 },
      credentialsAvailable: false,
      autofillAvailable: false,
    };
    const signals = wrapSignals(createGraphOperationalRuntime());

    const form = signals.form({
      source: { title: "Ship docs" },
      host: {
        online: () => state.online,
        persistence: () => state.persistedDraft !== null,
        credentials: () => state.credentialsAvailable,
        autofill: () => state.autofillAvailable,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit, action }) => ({
        submit: submit({
          hostRequirements: ["online", "credentials"],
        }),
        saveDraft: action("saveDraft", {
          patchPolicy: "allowEmpty",
          hostEffect: "draft.save",
          hostRequirements: ["persistence", "autofill"],
        }),
      }),
    });

    form.fields.title.set("Ship docs now");
    const readiness = form.readiness();
    assert.equal(readiness.canSubmit, false);
    assert.equal(readiness.blockers.some((blocker) => blocker.kind === "host:offline"), true);
    assert.equal(readiness.blockers.some((blocker) => blocker.capability === "credentials"), true);

    const submitPlan = form.actionPlan("submit");
    const saveDraftPlan = form.actionPlan("saveDraft");
    assert.equal(submitPlan.status, "denied");
    assert.deepEqual(submitPlan.host.requirements, ["online", "credentials"]);
    assert.equal(saveDraftPlan.status, "denied");
    assert.equal(saveDraftPlan.readiness.blockers.some((blocker) => blocker.capability === "autofill"), true);
    assert.equal(
      saveDraftPlan.readiness.blockers.find((blocker) => blocker.capability === "autofill")?.reason,
      "autofill host capability is unavailable at the declared host boundary",
    );

    state.online = true;
    state.credentialsAvailable = true;
    state.autofillAvailable = true;
    assert.equal(form.readiness().canSubmit, true);
    assert.equal(form.actionPlan("submit").status, "accepted");
    assert.equal(form.actionPlan("saveDraft").status, "accepted");
  } finally {
    await cleanup();
  }
});

test("signals.form action planning snapshots host facts once per derived report", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    let onlineReadCount = 0;
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      host: {
        online: () => {
          onlineReadCount += 1;
          return onlineReadCount === 1 ? false : true;
        },
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          hostRequirements: ["online"],
        }),
      }),
    });

    form.fields.title.set("Ship docs now");
    const actionReport = form.actions();
    const submitPlan = actionReport.plans.find((plan) => plan.id === "submit");
    assert.ok(submitPlan);
    const plannedOnlineState = JSON.parse(submitPlan.host.digest).facts.online.state;
    assert.equal(submitPlan.host.digest, actionReport.host.digest);
    if (plannedOnlineState === "offline") {
      assert.equal(submitPlan.status, "denied");
      assert.equal(submitPlan.host.blockers.some((blocker) => blocker.kind === "host:offline"), true);
    } else {
      assert.equal(plannedOnlineState, "online");
      assert.equal(submitPlan.status, "accepted");
      assert.equal(submitPlan.host.blockers.length, 0);
    }
  } finally {
    await cleanup();
  }
});

test("signals.form host declarations deny unsupported host requirements and malformed bindings", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({
          title: field("title"),
        }),
        actions: ({ action }) => ({
          publish: action("publish", {
            hostRequirements: ["clipboard"],
          }),
        }),
      }),
      /host requirement is not supported/,
    );

    const malformed = signals.form({
      source: { title: "Ship docs" },
      host: {
        online: () => "sometimes",
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });
    assert.throws(() => malformed.host(), /online binding must resolve/);
  } finally {
    await cleanup();
  }
});
