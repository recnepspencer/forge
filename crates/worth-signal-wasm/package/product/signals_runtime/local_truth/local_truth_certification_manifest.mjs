export const localTruthCertificationManifest = Object.freeze({
  authority: ["schema-and-commit", "sealed-basis", "atomic-fault-injection"],
  mergeSemantics: ["sibling", "parent-child", "repeated-partial", "manual-resolution"],
  derivation: ["exact-aspects", "post-commit-failure", "destroy-and-rebuild"],
  deployment: ["worker-first", "main-thread-compatibility", "semantic-parity"],
  boundedness: { branches: 32, entities: 128, aspects: 64 },
  recovery: [
    "checkpoint-compaction",
    "derived-index-reconstruction",
    "checkpoint-corruption-denial",
    "stale-review-denial",
    "projection-rebuild",
    "protocol-replay-admission",
  ],
  uiNoShortcut: ["runtime-values", "runtime-alternative-ids", "no-object-compositor"],
});
