import assert from "node:assert/strict";
import test from "node:test";

import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("resource mutation response closeout matrix certifies product support categories", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const matrix = runtime.signals.resource.mutationResponses.closeoutMatrix();

    assert.deepEqual(matrix.proofLanes, [
      "runtime",
      "typeSurface",
      "docs",
      "closeout",
    ]);
    assert.ok(Object.isFrozen(matrix));
    assert.ok(Object.isFrozen(matrix.rows));
    assert.ok(Object.isFrozen(matrix.deferredErgonomics));
    assert.ok(matrix.rows.every((row) => Object.isFrozen(row)));
    assert.deepEqual(matrix.deferredErgonomics, []);
    assert.equal(
      matrix.rows.find((row) => row.lane === "saveDetailReplace")?.category,
      "supportedErgonomicHappyPath",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "updateRelatedCollectionItem")?.category,
      "supportedErgonomicHappyPath",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "updateRelatedSummary")?.category,
      "supportedErgonomicHappyPath",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "createPlacement")?.category,
      "supportedErgonomicHappyPath",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "createIdentityMigration")?.category,
      "supportedErgonomicHappyPath",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "deleteExactRemoval")?.category,
      "supportedErgonomicHappyPath",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "deleteCanonicalTombstone")?.category,
      "supportedErgonomicHappyPath",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "deliveryAwaited")?.category,
      "supportedTypedUnavailableFallback",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "advertisingDeniedAsErgonomics")?.category,
      "intentionallyOutOfScope",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "overclaimedDeclarations")?.category,
      "supportedPreciseDenial",
    );
    assert.equal(
      matrix.rows.find((row) => row.lane === "hiddenBestEffortMutation")?.category,
      "intentionallyOutOfScope",
    );
    assert.deepEqual(
      matrix.rows.find((row) => row.lane === "multiFamilyReconciliation")?.evidence.closeout,
      ["full_mutation_response_reconciliation_convergence.test.mjs"],
    );
    assert.throws(
      () => runtime.signals.resource.mutationResponses.closeoutMatrix("extra"),
      /does not accept arguments/,
    );
  } finally {
    await runtime.cleanup();
  }
});
