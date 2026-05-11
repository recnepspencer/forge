import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("detail response contracts own the detail finalizer lane", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.detail()();
    assert.equal(response.kind, "detail");
    assert.equal(response.lensProof.topology, "detail");
    assert.equal(
      response.lensProof.capabilityRows.some(
        (row) => row.locus === "detailResponse" && row.patchScope === "line",
      ),
      true,
    );

    const route = runtime.signals.api({}).url("/users/:userId").response(response);
    const user = route.detail({
      load: ({ userId }) => ({ id: userId, name: "First" }),
    });
    assert.deepEqual(user.line({ userId: "u1" }).value(), {
      id: "u1",
      name: "First",
    });

    assert.throws(
      () => route.list({ load: () => [] }),
      /detail response lane; use detail/,
    );
    assert.throws(
      () => route.paged({ accumulatePage: (_existing, next) => next, load: () => [] }),
      /detail response lane; use detail/,
    );
    assert.throws(
      () => route.update({ load: () => ({ id: "u1" }) }),
      /broad replacement only/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("summary response contracts own the single-response detail finalizer lane", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const response = runtime.signals.resource.response.summary()();
    assert.equal(response.kind, "summary");
    assert.equal(response.lensProof.topology, "summary");
    assert.equal(
      response.lensProof.capabilityRows.some(
        (row) => row.locus === "summaryResponse" && row.patchScope === "line",
      ),
      true,
    );

    const route = runtime.signals.api({}).url("/task-count").response(response);
    const count = route.detail({
      load: () => ({ total: 1 }),
    });
    assert.deepEqual(count.line({}).value(), { total: 1 });

    assert.throws(
      () => route.list({ load: () => [] }),
      /summary response lane; use detail/,
    );
    assert.throws(
      () => route.paged({ accumulatePage: (_existing, next) => next, load: () => [] }),
      /summary response lane; use detail/,
    );
    assert.throws(
      () => route.create({ load: () => ({ total: 1 }) }),
      /summary mutation response lenses/,
    );
  } finally {
    await runtime.cleanup();
  }
});
