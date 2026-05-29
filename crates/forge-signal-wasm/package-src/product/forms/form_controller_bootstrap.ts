const EMPTY_PATCH_PLAN = Object.freeze({
  semanticDirty: false,
  empty: true,
  operations: Object.freeze([]),
  blocked: Object.freeze([]),
  broadReplacement: false,
  replacement: null,
  equality: Object.freeze({}),
  breadth: Object.freeze({
    declaredFields: 0,
    comparedFields: 0,
    changedFields: 0,
    skippedRawInputFields: 0,
    omittedFields: 0,
    clearedFields: 0,
    sourceSnapshots: 0,
    effectiveSnapshots: 0,
  }),
  equivalenceDigest: "bootstrap:emptyPatchPlan",
});

const EMPTY_READINESS = Object.freeze({
  canSubmit: false,
  blockers: Object.freeze([]),
  patchPlan: EMPTY_PATCH_PLAN,
});

const EMPTY_MESSAGES = Object.freeze([]);

export function createFormControllerBootstrapFacade() {
  return {
    source() {
      return {};
    },
    draft() {
      return {};
    },
    effective() {
      return {};
    },
    dirty() {
      return false;
    },
    patchPlan() {
      return EMPTY_PATCH_PLAN;
    },
    readiness() {
      return EMPTY_READINESS;
    },
    visibleMessages() {
      return EMPTY_MESSAGES;
    },
  };
}
