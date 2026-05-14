import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../../runtime_fixture/real_request_runtime.mjs";

test("create responses can materialize a declared detail line through canonical replacement", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    let loadCount = 0;
    const userDetail = runtime.signals.api({}).url("/users/:userId").detail({
      load: ({ userId }) => {
        loadCount += 1;
        return { id: userId, name: "Loaded" };
      },
    });
    const createUser = runtime.signals.api({}).url("/users")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [
          {
            family: userDetail,
            params: ({ body }) => ({ userId: body.id }),
            fallback: "refetchRequired",
            detail: { kind: "replace" },
          },
        ],
        load: ({ body }) => ({ id: body.id, name: body.name }),
      });

    const plan = createUser.line({
      body: { id: "u2", name: "Created" },
    }).mutationResponse();
    const createdLine = userDetail.line({ userId: "u2" });

    assert.equal(loadCount, 0);
    assert.deepEqual(createdLine.value(), { id: "u2", name: "Created" });
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.executionArtifacts[0].scope, "line");
    assert.equal(plan.executionArtifacts[0].residency, "resident");
    assert.equal(plan.executionArtifacts[0].outcomeKind, "applied");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "replace");
    assert.equal(plan.executionArtifacts[0].targetVisibleValueVersion, 1);
    assert.equal(createdLine.diagnostics().lastDeliveryKind, "replace");
    assert.equal(createdLine.diagnostics().lastDeliveryScope, "line");
    assert.equal(createdLine.diagnostics().lastEffect.provenance, "deliveredReplace");
    assert.deepEqual(
      createdLine.history().lifecycle.map((entry) => entry.event),
      ["materialized", "delivered"],
    );
    assert.equal(plan.confirmation.kind, "consumedCanonicalTruth");
  } finally {
    await runtime.cleanup();
  }
});

test("create responses can replace a resident detail line through canonical mutation reconciliation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const userDetail = runtime.signals.api({}).url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId, name: "First" }),
    });
    const residentLine = userDetail.line({ userId: "u1" });
    const createUser = runtime.signals.api({}).url("/users")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [
          {
            family: userDetail,
            params: ({ body }) => ({ userId: body.id }),
            fallback: "refetchRequired",
            detail: { kind: "replace" },
          },
        ],
        load: ({ body }) => ({ id: body.id, name: body.name }),
      });

    const plan = createUser.line({
      body: { id: "u1", name: "Created Name" },
    }).mutationResponse();

    assert.deepEqual(residentLine.value(), { id: "u1", name: "Created Name" });
    assert.equal(plan.executionArtifacts[0].kind, "exactDetail");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "replace");
    assert.equal(residentLine.diagnostics().lastDeliveryScope, "line");
    assert.equal(residentLine.history().lifecycle.at(-1)?.event, "delivered");
  } finally {
    await runtime.cleanup();
  }
});

test("create responses can patch a resident detail field through canonical mutation reconciliation", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detailFields = runtime.signals.resource.detailFields({
      name: {
        read: (value) => value.name,
        write: (value, name) => ({ ...value, name }),
      },
    });
    const profileDetail = runtime.signals.api({}).url("/profiles/:profileId").detail({
      reconcile: detailFields,
      load: ({ profileId }) => ({ id: profileId, name: "First" }),
    });
    const residentLine = profileDetail.line({ profileId: "p1" });
    const createProfile = runtime.signals.api({}).url("/profiles")
      .response(runtime.signals.resource.response.detail()({ name: "name" }))
      .create({
        reconciles: [
          {
            family: profileDetail,
            params: ({ body }) => ({ profileId: body.id }),
            fallback: "refetchRequired",
            detail: { kind: "field", field: "name" },
          },
        ],
        load: ({ body }) => ({ id: body.id, name: body.name }),
      });

    const plan = createProfile.line({
      body: { id: "p1", name: "Renamed" },
    }).mutationResponse();

    assert.equal(residentLine.value().name, "Renamed");
    assert.equal(plan.executionArtifacts[0].scope, "field");
    assert.equal(plan.executionArtifacts[0].deliveryKind, "patch");
    assert.equal(residentLine.diagnostics().lastDeliveryScope, "field");
    assert.equal(residentLine.diagnostics().lastPatchedField, "name");
  } finally {
    await runtime.cleanup();
  }
});

test("create responses keep nonresident narrow detail targets in fallback posture", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const detailFields = runtime.signals.resource.detailFields({
      name: {
        read: (value) => value.name,
        write: (value, name) => ({ ...value, name }),
      },
    });
    const profileDetail = runtime.signals.api({}).url("/profiles/:profileId").detail({
      reconcile: detailFields,
      load: ({ profileId }) => ({ id: profileId, name: "Loaded" }),
    });
    const plan = runtime.signals.api({}).url("/profiles")
      .response(runtime.signals.resource.response.detail()({ name: "name" }))
      .create({
        reconciles: [
          {
            family: profileDetail,
            params: ({ body }) => ({ profileId: body.id }),
            fallback: "refetchRequired",
            detail: { kind: "field", field: "name" },
          },
        ],
        load: ({ body }) => ({ id: body.id, name: body.name }),
      })
      .line({
        body: { id: "p2", name: "Created" },
      })
      .mutationResponse();

    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].fallback, "refetchRequired");
    assert.equal(plan.confirmation.kind, "refetchRequired");
  } finally {
    await runtime.cleanup();
  }
});

test("create responses deny exact detail replacement when a declared target became resident before response settlement", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    let resolveCreate;
    const userDetail = runtime.signals.api({}).url("/users/:userId").detail({
      load: ({ userId }) => ({ id: userId, name: "Resident Load" }),
    });
    const createUser = runtime.signals.api({}).url("/users")
      .response(runtime.signals.resource.response.detail()())
      .create({
        reconciles: [
          {
            family: userDetail,
            params: ({ body }) => ({ userId: body.id }),
            fallback: "refetchRequired",
            detail: { kind: "replace" },
          },
        ],
        load: ({ body }) =>
          new Promise((resolve) => {
            resolveCreate = () => resolve({ id: body.id, name: body.name });
          }),
      });

    const writeLine = createUser.line({
      body: { id: "u3", name: "Created" },
    });
    const residentLine = userDetail.line({ userId: "u3" });
    resolveCreate();
    await new Promise((resolve) => setTimeout(resolve, 0));
    const plan = writeLine.mutationResponse();

    assert.deepEqual(residentLine.value(), { id: "u3", name: "Resident Load" });
    assert.equal(plan.executionArtifacts[0].kind, "fallback");
    assert.equal(plan.executionArtifacts[0].fallback, "refetchRequired");
    assert.equal(plan.executionArtifacts[0].staleness?.reason, "runtimeLineIdChanged");
    assert.equal(plan.confirmation.kind, "refetchRequired");
  } finally {
    await runtime.cleanup();
  }
});
