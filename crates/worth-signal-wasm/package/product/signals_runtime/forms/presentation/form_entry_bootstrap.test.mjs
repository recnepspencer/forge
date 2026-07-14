import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form entry presentation can wait for declared layout measurement bootstrap", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title", { row: "main" }),
      }),
      presentation: {
        entry: {
          bootstrap: {
            layoutMeasurement: true,
          },
        },
      },
    });

    const pendingLane = form.presentationLifecycle("entry");
    assert.equal(pendingLane.status, "pending");
    assert.equal(pendingLane.bootstrap?.posture, "pending");
    assert.equal(pendingLane.bootstrap?.layoutMeasurementPending, true);
    assert.deepEqual(
      pendingLane.bootstrap?.dependencies.blocking.map((dependency) => dependency.dependency),
      ["layoutMeasurement"],
    );

    form.recordLayoutMeasurement([
      {
        row: "main",
        labelHeight: 18,
        controlHeight: 32,
        messageHeight: 0,
      },
    ], {
      cause: "animationFrame",
      frameToken: "entry-frame-1",
    });

    const readyLane = form.presentationLifecycle("entry");
    assert.equal(readyLane.status, "ready");
    assert.equal(readyLane.bootstrap?.posture, "ready");
    assert.equal(readyLane.bootstrap?.layoutMeasurementPending, false);
    assert.deepEqual(
      readyLane.bootstrap?.dependencies.satisfied.map((dependency) => dependency.dependency),
      ["layoutMeasurement"],
    );
  } finally {
    await cleanup();
  }
});

test("signals.form entry bootstrap keeps host, adapter capability, focus-target, source-compatibility, and readiness evidence explicit", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const source = signals.input({ title: "" });
    const schemaVersion = signals.input("v1");
    const form = signals.form({
      source: {
        value: source,
        schemaVersion,
      },
      fields: ({ field }) => ({
        title: field("title", {
          adapter: {
            tier: "externalImperative",
            reportsFocus: false,
            reportsRawInput: false,
          },
        }),
      }),
      validation: ({ field }) => ({
        titleRequired: field("title", (value) => (
          value.length > 0
            ? true
            : {
              kind: "invalid",
              message: {
                code: "title.required",
                message: "Title is required",
                target: "title",
                severity: "error",
                audience: "user",
                visibility: "visible",
              },
            }
        )),
      }),
      presentation: {
        entry: {
          bootstrap: {
            sourceCompatibility: true,
            readiness: true,
            hostFacts: true,
            inputCapabilities: true,
            focusTarget: true,
          },
        },
      },
    });

    form.fields.title.set("Client title");
    source.set({ title: "Server title" });
    schemaVersion.set("v2");

    const entryLane = form.presentationLifecycle("entry");
    assert.equal(entryLane.status, "unavailable");
    assert.equal(entryLane.bootstrap?.posture, "unavailable");
    assert.equal(entryLane.bootstrap?.hostUnavailableFacts.includes("focus"), true);
    assert.deepEqual(entryLane.bootstrap?.inputUnavailableFields, ["title"]);
    assert.equal(entryLane.bootstrap?.focusTarget?.posture, "none");
    assert.deepEqual(
      entryLane.bootstrap?.dependencies.required.map((dependency) => dependency.dependency),
      ["sourceCompatibility", "readiness", "hostFacts", "inputCapabilities", "focusTarget"],
    );
    assert.deepEqual(
      entryLane.bootstrap?.dependencies.unavailable.map((dependency) => dependency.dependency).sort(),
      ["hostFacts", "inputCapabilities", "sourceCompatibility"],
    );
    assert.equal(
      entryLane.bootstrap?.dependencies.satisfied.some((dependency) => dependency.dependency === "focusTarget"),
      true,
    );
    assert.equal(
      entryLane.bootstrap?.dependencies.satisfied.some((dependency) => (
        dependency.dependency === "readiness" &&
        dependency.reason.includes("blocker")
      )),
      true,
    );
    assert.equal(typeof entryLane.bootstrap?.digest, "string");
  } finally {
    await cleanup();
  }
});

test("signals.form entry bootstrap can wait on declared initial validation without treating invalid readiness as unavailable", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { slug: "ship-docs" },
      fields: ({ field }) => ({
        slug: field("slug"),
      }),
      validation: ({ asyncField }) => ({
        slugUnique: asyncField("slug", {
          id: "slugUnique",
          triggers: ["explicit"],
        }),
      }),
      presentation: {
        entry: {
          delayedBusyRevealMs: 0,
          minimumBusyMs: 0,
          bootstrap: {
            validation: true,
            readiness: true,
          },
        },
      },
    });

    form.startAsyncValidation("slugUnique");

    const waitingLane = form.presentationLifecycle("entry");
    assert.equal(waitingLane.status, "busy");
    assert.equal(waitingLane.bootstrap?.posture, "pending");
    assert.deepEqual(
      waitingLane.bootstrap?.dependencies.blocking.map((dependency) => dependency.dependency),
      ["validation"],
    );
    assert.equal(
      waitingLane.bootstrap?.dependencies.satisfied.some((dependency) => dependency.dependency === "readiness"),
      true,
    );

    const validationOperation = form.asyncValidationHistory().at(-1);
    form.fulfillAsyncValidation(validationOperation.operationId);

    const readyLane = form.presentationLifecycle("entry");
    assert.equal(readyLane.status, "ready");
    assert.equal(readyLane.bootstrap?.posture, "ready");
    assert.deepEqual(
      readyLane.bootstrap?.dependencies.satisfied.map((dependency) => dependency.dependency).sort(),
      ["readiness", "validation"],
    );
  } finally {
    await cleanup();
  }
});

test("signals.form entry bootstrap can wait on declared source admission and draft restore prerequisites", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const source = signals.input({ title: "Ship docs" });
    const publicInput = signals.publicInput(source, { authority: "readOnly" });
    const sourceAdmission = signals.input({
      status: "pending",
      reason: "source handshake is still in flight",
      token: "source-admission-1",
    });
    const draftRestore = signals.input({
      status: "busy",
      reason: "draft restore is replaying local edits",
      token: "draft-restore-1",
    });
    const form = signals.form({
      source: {
        value: signals.form.source.graphPublicInput(publicInput, { id: "entry-bootstrap-public-input" }),
        sourceAdmission,
        draftRestore,
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        entry: {
          delayedBusyRevealMs: 0,
          minimumBusyMs: 0,
          bootstrap: {
            sourceAdmission: true,
            draftRestore: true,
          },
        },
      },
    });

    assert.equal(form.sourceAuthority().kind, "graphPublicInput");
    assert.equal(form.sourceAdmission()?.status, "pending");
    assert.equal(form.draftRestore()?.status, "busy");

    const waitingLane = form.presentationLifecycle("entry");
    assert.equal(waitingLane.status, "pending");
    assert.equal(waitingLane.bootstrap?.posture, "pending");
    assert.deepEqual(
      waitingLane.bootstrap?.dependencies.required.map((dependency) => dependency.dependency),
      ["sourceAdmission", "draftRestore"],
    );
    assert.deepEqual(
      waitingLane.bootstrap?.dependencies.blocking.map((dependency) => dependency.dependency),
      ["sourceAdmission", "draftRestore"],
    );

    sourceAdmission.set({
      status: "ready",
      reason: "source handshake completed",
      token: "source-admission-2",
    });
    draftRestore.set({
      status: "ready",
      reason: "draft restore completed",
      token: "draft-restore-2",
    });

    const readyLane = form.presentationLifecycle("entry");
    assert.equal(readyLane.status, "ready");
    assert.equal(readyLane.bootstrap?.posture, "ready");
    assert.deepEqual(
      readyLane.bootstrap?.dependencies.satisfied.map((dependency) => dependency.dependency),
      ["sourceAdmission", "draftRestore"],
    );

    const signalWrappedForm = signals.form({
      source: {
        value: signals.form.source.signal(source, { id: "entry-bootstrap-signal" }),
        sourceAdmission: {
          status: "ready",
          reason: "signal source is admitted",
        },
        draftRestore: {
          status: "ready",
          reason: "signal draft restore is settled",
        },
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        entry: {
          bootstrap: {
            sourceAdmission: true,
            draftRestore: true,
          },
        },
      },
    });

    assert.equal(signalWrappedForm.sourceAuthority().kind, "signal");
    assert.equal(signalWrappedForm.presentationLifecycle("entry")?.status, "ready");
  } finally {
    await cleanup();
  }
});

test("signals.form entry bootstrap keeps missing or malformed source-side prerequisites explicit", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        entry: {
          bootstrap: {
            sourceAdmission: true,
            draftRestore: true,
          },
        },
      },
    });

    const unavailableLane = form.presentationLifecycle("entry");
    assert.equal(unavailableLane.status, "unavailable");
    assert.equal(unavailableLane.bootstrap?.posture, "unavailable");
    assert.deepEqual(
      unavailableLane.bootstrap?.dependencies.unavailable.map((dependency) => dependency.dependency),
      ["sourceAdmission", "draftRestore"],
    );

    const malformedForm = signals.form({
      source: {
        value: { title: "Ship docs" },
        sourceAdmission: {
          status: "pending",
          reason: "source handshake is still in flight",
        },
        draftRestore: {
          status: "ready",
          reason: 1,
        },
      },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        entry: {
          bootstrap: {
            sourceAdmission: true,
            draftRestore: true,
          },
        },
      },
    });

    assert.throws(
      () => malformedForm.presentationLifecycle("entry"),
      /form source draftRestore reason must be a non-empty string/,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form denies malformed or misplaced entry bootstrap declarations", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({
          title: field("title"),
        }),
        presentation: {
          action: {
            bootstrap: {
              hostFacts: true,
            },
          },
        },
      }),
      /bootstrap policy is only supported for entry/,
    );

    assert.throws(
      () => signals.form({
        source: { title: "Ship docs" },
        fields: ({ field }) => ({
          title: field("title"),
        }),
        presentation: {
          entry: {
            bootstrap: {
              layoutMeasurement: "yes",
            },
          },
        },
      }),
      /layoutMeasurement must be a boolean/,
    );
  } finally {
    await cleanup();
  }
});
