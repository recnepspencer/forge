const MUTATION_RESPONSE_CLOSEOUT_PROOF_LANES = Object.freeze([
  "runtime",
  "typeSurface",
  "docs",
  "closeout",
]);

const MUTATION_RESPONSE_CLOSEOUT_ROWS = Object.freeze([
  createMutationResponseCloseoutMatrixRow({
    lane: "saveDetailReplace",
    category: "supportedErgonomicHappyPath",
    summary:
      "Exact whole-detail reconciliation is supported when the route proves the whole canonical detail value.",
    evidence: {
      runtimeTests: ["save_response_detail_line_reconciliation.test.mjs"],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-reconciliation.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "saveDetailGranular",
    category: "supportedErgonomicHappyPath",
    summary:
      "Exact field, JSON path, and region reconciliation is supported when the route declares the matching detail locus.",
    evidence: {
      runtimeTests: ["detail_granular_response.test.mjs"],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-reconciliation.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "updateRelatedCollectionItem",
    category: "supportedErgonomicHappyPath",
    summary:
      "Related collection item reconciliation is supported for declared resident targets.",
    evidence: {
      runtimeTests: [
        "save_response_collection_summary_reconciliation.test.mjs",
        "save_response_multi_family_convergence.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-reconciliation.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "updateRelatedSummary",
    category: "supportedErgonomicHappyPath",
    summary:
      "Related summary reconciliation is supported for declared resident targets.",
    evidence: {
      runtimeTests: [
        "save_response_collection_summary_reconciliation.test.mjs",
        "save_response_multi_family_convergence.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-reconciliation.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "createPlacement",
    category: "supportedErgonomicHappyPath",
    summary:
      "Create placement is supported when the response proves canonical insertion into the declared topology.",
    evidence: {
      runtimeTests: [
        "create_response_collection_placement_reconciliation.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-reconciliation.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "createIdentityMigration",
    category: "supportedErgonomicHappyPath",
    summary:
      "Create identity migration is supported when the response proves canonical identity mapping for the admitted target classes.",
    evidence: {
      runtimeTests: [
        "save_response_identity_migration_foundation.test.mjs",
        "save_response_identity_migration_target_classes.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-reconciliation.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "deleteExactRemoval",
    category: "supportedErgonomicHappyPath",
    summary:
      "Exact deletion is supported when the response proves the removed resident item in the declared topology.",
    evidence: {
      runtimeTests: [
        "remove_response_additional_topology_deletion_reconciliation.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-reconciliation.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "deleteCanonicalTombstone",
    category: "supportedErgonomicHappyPath",
    summary:
      "Canonical tombstone posture is supported when the route declares a retained deleted-item shape and the response proves it.",
    evidence: {
      runtimeTests: [
        "remove_response_tombstone_reconciliation.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-reconciliation.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "multiFamilyReconciliation",
    category: "supportedErgonomicHappyPath",
    summary:
      "One write response may reconcile multiple declared families together in one canonical plan.",
    evidence: {
      runtimeTests: [
        "save_response_multi_family_convergence.test.mjs",
        "save_response_multi_family_target_outcome_classes.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-reconciliation.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "refetchRequired",
    category: "supportedTypedUnavailableFallback",
    summary:
      "The write was admitted, but exact canonical local truth still requires a later refresh.",
    evidence: {
      runtimeTests: [
        "mutation_response_fallback_honesty.test.mjs",
        "save_response_stale_target_denial.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "deliveryAwaited",
    category: "supportedTypedUnavailableFallback",
    summary:
      "The write was admitted, but exact canonical local truth is expected from a later delivery update.",
    evidence: {
      runtimeTests: ["mutation_response_fallback_honesty.test.mjs"],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "partialReconciliation",
    category: "supportedTypedUnavailableFallback",
    summary:
      "Some declared targets reconciled exactly and others stayed explicit in partial fallback posture.",
    evidence: {
      runtimeTests: [
        "save_response_partial_mapping.test.mjs",
        "save_response_multi_family_partial_allowed.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "placementUnavailable",
    category: "supportedTypedUnavailableFallback",
    summary:
      "The create route was admitted, but the response did not prove an exact insertion position.",
    evidence: {
      runtimeTests: ["mutation_response_fallback_honesty.test.mjs"],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "deletionUnavailable",
    category: "supportedTypedUnavailableFallback",
    summary:
      "The remove route was admitted, but the response did not prove an exact topology deletion.",
    evidence: {
      runtimeTests: ["remove_response_detail_invalidation_reconciliation.test.mjs"],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "identityMigrationUnavailable",
    category: "supportedTypedUnavailableFallback",
    summary:
      "The write was admitted, but the declared target could not migrate exactly.",
    evidence: {
      runtimeTests: ["save_response_identity_migration_policy.test.mjs"],
      typeSurface: ["resource_api_response_mutation_contract_usage.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["full_mutation_response_reconciliation_convergence.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "overclaimedDeclarations",
    category: "supportedPreciseDenial",
    summary:
      "Type and runtime boundaries reject detail, placement, deletion, identity, and multi-target declarations that claim stronger proof than the route really has.",
    evidence: {
      runtimeTests: [
        "api_write_semantic_finalizers.test.mjs",
        "save_response_multi_family_target_outcome_classes.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_denials.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["resource_mutation_response_closeout_matrix.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "hiddenBestEffortMutation",
    category: "intentionallyOutOfScope",
    summary:
      "Forge does not claim silent best-effort mutation of undeclared read truth.",
    evidence: {
      runtimeTests: ["mutation_response_fallback_honesty.test.mjs"],
      typeSurface: ["resource_api_response_mutation_contract_denials.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["resource_mutation_response_closeout_matrix.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "fallbackPresentedAsExact",
    category: "intentionallyOutOfScope",
    summary:
      "Fallback posture is not equivalent to exact reconciliation and must not be marketed as such.",
    evidence: {
      runtimeTests: [
        "mutation_response_fallback_honesty.test.mjs",
        "full_mutation_response_reconciliation_convergence.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_denials.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["resource_mutation_response_closeout_matrix.test.mjs"],
    },
  }),
  createMutationResponseCloseoutMatrixRow({
    lane: "advertisingDeniedAsErgonomics",
    category: "intentionallyOutOfScope",
    summary:
      "Denied-only capability rows must not be marketed as normal ergonomic support.",
    evidence: {
      runtimeTests: [
        "api_write_semantic_finalizers.test.mjs",
        "resource_mutation_response_closeout_matrix.test.mjs",
      ],
      typeSurface: ["resource_api_response_mutation_contract_denials.ts"],
      docs: ["mutation-response-closeout-matrix.md"],
      closeout: ["resource_mutation_response_closeout_matrix.test.mjs"],
    },
  }),
]);

const resourceMutationResponses = Object.freeze({
  closeoutMatrix(...args) {
    if (args.length !== 0) {
      throw new TypeError(
        "resource.mutationResponses.closeoutMatrix() does not accept arguments",
      );
    }
    return createMutationResponseCloseoutMatrix();
  },
});

function createMutationResponseCloseoutMatrix() {
  return Object.freeze({
    proofLanes: MUTATION_RESPONSE_CLOSEOUT_PROOF_LANES,
    rows: MUTATION_RESPONSE_CLOSEOUT_ROWS,
    deferredErgonomics: Object.freeze([]),
  });
}

function createMutationResponseCloseoutMatrixRow({
  lane,
  category,
  summary,
  evidence,
}) {
  return Object.freeze({
    lane,
    category,
    summary,
    runtimeProof: true,
    typeSurfaceProof: true,
    docsProof: true,
    closeoutProof: true,
    evidence: Object.freeze({
      runtimeTests: Object.freeze([...evidence.runtimeTests]),
      typeSurface: Object.freeze([...evidence.typeSurface]),
      docs: Object.freeze([...evidence.docs]),
      closeout: Object.freeze([...evidence.closeout]),
    }),
  });
}

export { resourceMutationResponses };
