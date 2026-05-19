import assert from "node:assert/strict";
import test from "node:test";

import { withSignals } from "../../action_execution_test_helpers.mjs";
import {
  createMutationResponsePlanFixture,
  createReadOnlyResourceLineFixture,
} from "../fixtures/resource_line_fixture.mjs";

test("signals.form exposes resource line settlement confirmation and verification parity", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: Object.freeze({ kind: "fulfilled", operation: "delivery" }),
          freshness: Object.freeze({ kind: "fresh" }),
          visibleSelection: Object.freeze({
            kind: "merged",
            source: "branchMerge",
            effectId: "effect-7",
            branchId: 7,
            snapshotId: 14,
            basisId: "basis-7",
            detail: "merged branch truth is visible after confirmation",
          }),
          mutationResponse: createMutationResponsePlanFixture({
            confirmationKind: "partialCanonicalTruth",
            fallbackKind: "partialReconciliation",
          }),
        }),
        { id: "resource-settlement-confirmed" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });

    const settlement = form.resourceSource().settlement;
    assert.equal(settlement.kind, "confirmed");
    assert.equal(settlement.operation, "delivery");
    assert.equal(settlement.confirmationKind, "partialCanonicalTruth");
    assert.equal(settlement.visibleSelectionKind, "merged");
    assert.equal(settlement.branchProof.admitted, true);
    assert.equal(settlement.rebaseProof.admitted, true);
    assert.equal(form.verification().digests.resourceSettlementDigest, settlement.digest);
    assert.equal(form.diagnostics().resourceSource.settlement.digest, settlement.digest);
  });
});

test("signals.form exposes resource line failed settlement without laundering it into confirmation truth", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: Object.freeze({
            kind: "rejected",
            operation: "refresh",
            message: "network down",
            continuity: "preservedVisibleValue",
          }),
          freshness: Object.freeze({ kind: "stale", reason: "refreshRejected" }),
        }),
        { id: "resource-settlement-failed" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });

    const settlement = form.resourceSource().settlement;
    assert.equal(settlement.kind, "failed");
    assert.equal(settlement.failureKind, "rejected");
    assert.equal(settlement.operation, "refresh");
    assert.equal(settlement.continuity, "preservedVisibleValue");
    assert.equal(settlement.message, "network down");
    assert.equal(settlement.retryRecommended, true);
    assert.equal(settlement.retryOperation, "refresh");
    assert.equal(settlement.confirmationKind, null);
    assert.equal(form.verification().digests.resourceSettlementDigest, settlement.digest);
  });
});

test("signals.form keeps missing settlement proof explicit when the resource source carries neither failure nor confirmation", async () => {
  await withSignals((signals) => {
    const form = signals.form({
      source: signals.form.source.resourceLine(
        createReadOnlyResourceLineFixture({
          status: Object.freeze({ kind: "fulfilled", operation: "initialLoad" }),
          freshness: Object.freeze({ kind: "fresh" }),
        }),
        { id: "resource-settlement-none" },
      ),
      fields: ({ field }) => ({ title: field("title") }),
    });

    const settlement = form.resourceSource().settlement;
    assert.equal(settlement.kind, "none");
    assert.equal(
      settlement.detail,
      "resource line source does not carry confirmation or failure settlement proof",
    );
    assert.equal(form.verification().digests.resourceSettlementDigest, settlement.digest);
  });
});
