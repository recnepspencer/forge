import {
  createSignals,
  type ResourceMutationResponseCloseoutMatrix,
  type ResourceMutationResponseCloseoutMatrixRow,
} from "../../../index.js";

const signals = createSignals();

const matrix: ResourceMutationResponseCloseoutMatrix =
  signals.resource.mutationResponses.closeoutMatrix();
const fallbackRow:
  | ResourceMutationResponseCloseoutMatrixRow
  | undefined = matrix.rows.find((row) => row.lane === "deliveryAwaited");
const collectionItemRow:
  | ResourceMutationResponseCloseoutMatrixRow
  | undefined = matrix.rows.find(
    (row) => row.lane === "updateRelatedCollectionItem",
  );
const summaryRow:
  | ResourceMutationResponseCloseoutMatrixRow
  | undefined = matrix.rows.find((row) => row.lane === "updateRelatedSummary");

void matrix.proofLanes.includes("docs");
void matrix.deferredErgonomics.length;
void fallbackRow?.closeoutProof;
void collectionItemRow?.runtimeProof;
void summaryRow?.docsProof;
void fallbackRow?.evidence.runtimeTests.includes(
  "full_mutation_response_reconciliation_convergence.test.mjs",
);
