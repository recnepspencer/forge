import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../../runtime_fixture/real_request_runtime.mjs";

test("detail field response contracts expose detail family helpers and narrow reconciliation truth", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.detail()({
      name: "name",
    });
    assert.equal(response.kind, "detail");
    assert.deepEqual(response.lensProof.fieldNames, ["name"]);
    assert.equal(
      response.lensProof.capabilityRows.some(
        (row) => row.locus === "detailField" && row.patchScope === "field",
      ),
      true,
    );

    const user = runtime.signals.api({}).url("/users/:userId").response(response).detail({
      load: ({ userId }) => ({ id: userId, name: "First" }),
    });
    const fieldPatch = user.patch.field({
      field: "name",
      value: "Updated",
    });
    const fieldDelivery = user.delivery.field({
      packetId: "pkt-user-name",
      basisId: null,
      nextBasisId: "basis-1",
      field: "name",
      value: "Delivered",
    });
    const line = user.line({ userId: "u1" });

    assert.equal(fieldPatch.kind, "field");
    assert.equal(fieldDelivery.patch.kind, "field");
    assert.equal(line.reconciliation().narrowField, true);
    assert.deepEqual(line.reconciliation().fieldNames, ["name"]);

    line.patch(fieldPatch);
    assert.deepEqual(line.value(), { id: "u1", name: "Updated" });
    assert.equal(line.diagnostics().lastEffect.locus.kind, "detailField");
    assert.equal(line.diagnostics().lastEffect.patch.fieldProof.fieldName, "name");
    assert.equal(
      line.diagnostics().lastEffect.patch.fieldProof.version,
      "resource-detail-field-proof-v1",
    );
    assert.equal(line.diagnostics().lastEffect.counters.detailFieldTraversalBreadth, 1);
    assert.equal(
      line.diagnostics().lastEffect.counters.detailFieldReconstructionBreadth,
      1,
    );
    assert.equal(line.diagnostics().lastEffect.counters.detailRegionTraversalBreadth, 0);

    line.deliver(fieldDelivery);
    assert.deepEqual(line.value(), { id: "u1", name: "Delivered" });
    assert.equal(line.diagnostics().lastEffect.patch.fieldProof.fieldName, "name");
    assert.equal(line.diagnostics().lastEffect.delivery.scope, "field");
  } finally {
    await runtime.cleanup();
  }
});

test("resource detail field declarations deny accessor-backed maps without invoking them", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    let getterReadCount = 0;
    const declaredFields = {};
    Object.defineProperty(declaredFields, "name", {
      enumerable: true,
      get() {
        getterReadCount += 1;
        return {
          read: (value) => value.name,
          write: (value, name) => ({ ...value, name }),
        };
      },
    });

    assert.throws(
      () => runtime.signals.resource.detailFields(declaredFields),
      /rejects accessor detail field declaration "name"/,
    );
    assert.equal(getterReadCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("resource detail field declarations reject unsafe prototype field names", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const declaredFields = {};
    Object.defineProperty(declaredFields, "__proto__", {
      enumerable: true,
      configurable: true,
      value: {
        read: (value) => value.polluted,
        write: (value, polluted) => ({ ...value, polluted }),
      },
    });

    assert.throws(
      () => runtime.signals.resource.detailFields(declaredFields),
      /rejects unsafe detail field "__proto__"/,
    );
    assert.throws(
      () => runtime.signals.resource.detailFields({
        constructor: {
          read: (value) => value.constructor,
          write: (value, constructor) => ({ ...value, constructor }),
        },
      }),
      /rejects unsafe detail field "constructor"/,
    );
    assert.throws(
      () => runtime.signals.resource.detailFields({
        prototype: {
          read: (value) => value.prototype,
          write: (value, prototype) => ({ ...value, prototype }),
        },
      }),
      /rejects unsafe detail field "prototype"/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("detail field response declarations deny accessor-backed field maps without invoking them", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    let getterReadCount = 0;
    const declaredFields = {};
    Object.defineProperty(declaredFields, "name", {
      enumerable: true,
      get() {
        getterReadCount += 1;
        return "name";
      },
    });

    assert.throws(
      () => runtime.signals.resource.response.detail()(declaredFields),
      /rejects accessor detail field declaration "name"/,
    );
    assert.equal(getterReadCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("detail field response declarations reject unsafe field and backing names", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const declaredFields = {};
    Object.defineProperty(declaredFields, "__proto__", {
      enumerable: true,
      configurable: true,
      value: "name",
    });

    assert.throws(
      () => runtime.signals.resource.response.detail()(declaredFields),
      /rejects unsafe detail field "__proto__"/,
    );
    assert.throws(
      () => runtime.signals.resource.response.detail()({
        polluted: "__proto__",
      }),
      /field "polluted" rejects unsafe object field "__proto__"/,
    );
    assert.throws(
      () => runtime.signals.resource.response.detail()({
        polluted: "prototype",
      }),
      /field "polluted" rejects unsafe object field "prototype"/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("detail field response contracts deny accessor-backed detail values without invoking them", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.detail()({
      name: "name",
    });
    let getterReadCount = 0;
    const accessorValue = { id: "u1" };
    Object.defineProperty(accessorValue, "name", {
      enumerable: true,
      get() {
        getterReadCount += 1;
        return "Hidden";
      },
    });

    assert.throws(
      () => response.fields.definitions.name.read(accessorValue),
      /rejects accessor detail value field "name"/,
    );
    assert.throws(
      () => response.fields.definitions.name.write(accessorValue, "Updated"),
      /rejects accessor detail value property "name"/,
    );
    assert.equal(getterReadCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("detail field response writes deny unrelated accessor properties before cloning", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.detail()({
      name: "name",
    });
    let getterReadCount = 0;
    const accessorValue = { id: "u1", name: "First" };
    Object.defineProperty(accessorValue, "sideEffect", {
      enumerable: true,
      get() {
        getterReadCount += 1;
        return "boom";
      },
    });

    assert.throws(
      () => response.fields.definitions.name.write(accessorValue, "Updated"),
      /rejects accessor detail value property "sideEffect"/,
    );
    assert.equal(getterReadCount, 0);
  } finally {
    await runtime.cleanup();
  }
});
