# Graph Obligation Consumer Kit

The graph obligation Consumer Kit is the ordinary downstream adoption path for
crates that consume Query's graph touch obligation authority.

It exists so downstream crates do not need their own local ceremony for
registration, selector coverage, support pinning, in-memory proof, bypass
audit, adoption manifests, or residue manifests.

## What This Feature Is

The kit is the proof lane for consumers, not the authority itself. Query owns
graph touch descriptors, operating world descriptors, obligation selection,
dispatch envelopes, executor verdicts, support rows, and diagnostics. The
Consumer Kit gives downstream crates a stable way to prove they are using those
surfaces honestly.

For graph obligations, honest adoption is execution-backed. A consumer should
prove that Query selected obligations from a real touch descriptor and
operating world, executed the selected obligations in the in-memory proof
workspace, and attached the execution proof digest to the adoption manifest.

The mental model is:

```text
consumer owns domain facts and source files
Query owns graph obligation consumption proof
```

If the consumer has to invent a report struct, grep its own code for forbidden
graph traversals, pin support by string lists, or fake a receipt because a real
workspace is awkward to create, the kit is missing from the path.

## Required Proof Jobs

The complete consumer path must cover these jobs:

- registration: declare graph obligations through Query's public facade
- selector coverage: prove relevant graph touches can select the expected
  obligations
- support pinning: bind required support posture by obligation kind, support
  lane, expected status, and budget digest where the consumer depends on a
  specific execution budget
- in-memory proof: run adoption tests against a real Query workspace
- execution-backed adoption: connect selected obligations to real executor rows
  with `prove_execution_with(...)` and `prove_adoption_with_execution()`,
  producing `ForgeQueryGraphObligationExecutionBackedAdoptionProof`
- bypass audit: detect local validator tables, local graph walks, private
  legality graphs, and other local ceremony
- adoption manifests: record what moved from consumer folklore into Query
- residue manifests: record what remains and whether it is a compatibility
  bridge, temporary residue, or product gap

These are not optional extras. They are the public proof shape required to make
graph obligation adoption repeatable across downstream crates.

## Obligation And Support Vocabulary

Consumer-kit reports must use the same obligation kinds as graph touch
authority and the support matrix:

- `BlockingInvariant`
- `SchemaContractValidator`
- `AdvisoryObligation`
- `PreflightSequencingObligation`
- `CapabilityGapScreen`
- `OperatingContextGate`

They must also use the same support statuses:

- `Supported`
- `Unsupported`
- `NotApplicable`
- `DiagnosticOnly`
- `DeferredToBackstop`

Different names in consumer reports are evidence drift. They make support
pinning and certification weaker because the consumer cannot prove it is
talking about the same lane as Query.

Canonical kind labels are `blocking-invariant`,
`schema-contract-validator`, `advisory-obligation`,
`preflight-sequencing-obligation`, `capability-gap-screen`, and
`operating-context-gate`. Canonical support status labels are `supported`,
`unsupported`, `not-applicable`, `diagnostic-only`, and
`deferred-to-backstop`.

Consumer-kit support pins and manifests must also preserve the covered lane
vocabulary used by the `Milestone 9.9 Graph Touch Obligation Authority Hostile Certification Matrix`:

- graph composition
- authoritative command batch
- scalar mutation
- effect-triggered write intent
- declaration entry
- contribution orchestration
- read family
- live read
- preview mutation
- preview intent
- branch intent
- policy-aware graph mutation
- primitive construction birth
- worth-topo operator catalog
- worth-kernel phase chain

Canonical covered lane labels are `graph-composition`,
`authoritative-command-batch`, `scalar-mutation`,
`effect-triggered-write-intent`, `declaration-entry`,
`contribution-orchestration`, `read-family`, `live-read`,
`preview-mutation`, `preview-intent`, `branch-intent`,
`policy-aware-graph-mutation`, `primitive-construction-birth`,
`worth-topo-operator-catalog`, and `worth-kernel-phase-chain`.

The kit may report a lane as `Unsupported`, `NotApplicable`,
`DiagnosticOnly`, or `DeferredToBackstop`, but it must keep the lane identity
visible so downstream proof does not collapse into local ceremony.

## Budget And Diagnostic Requirements

Consumer proof must be budget-honest. Large graph and boolean-like operations
are allowed to deny with `BudgetExceeded`. Reports should preserve
`budget-exceeded`, state-load counters, cost classes such as
`sparse-topology`, and artifact-policy-gated diagnostics when those artifacts
are available.

The kit must not turn a budget denial into silent success by shrinking,
sampling, or locally completing a graph walk. The consumer can ask Query for a
different admitted proof posture, but the final evidence must still say what
Query actually proved.

## Adoption Workflow

```text
1. register graph obligations through Query
2. run selector coverage against real touch descriptors
3. project and pin support rows by typed identity and digest
4. execute in-memory proof against a real Query workspace
5. run bypass audit for local ceremony
6. publish adoption manifests and residue manifests
7. fail certification on unsupported drift, fake proof, or unowned residue
```

The workflow is intentionally boring. Consumers should not need to design a
local proof system just to use Query's graph obligation authority.

Execution-backed adoption should use the public kit path:

```rust
let proof = graph_obligation_consumer_kit("worth-topo")
    .register_obligations(registration_declaration)
    .declare_selector_coverage(selector_coverage)
    .pin_support(support_pin)
    .audit_local_ceremony(local_ceremony_audit)
    .account_for_residue(residue_manifest)
    .prove_execution_with(&touch_descriptor, &operating_world)
    .unwrap()
    .prove_adoption_with_execution()
    .unwrap();

assert!(proof.execution_proof().has_real_executor_rows());
assert!(proof.manifest().execution_proof_digest().is_some());
```

Selection-only proof can still be useful for narrow inspection, but closeout
adoption of covered graph obligation authority should prefer execution-backed
proof.

## Anti-Patterns

- checked-in string lists pretending to be support pinning
- local AST greps that are not tied to Query's hard-prohibition registry
- fake receipts for graph obligation proof
- selection-only adoption used as final closeout proof for covered execution
  lanes
- tests that only assert a local fixture report can be formatted
- presenting manual invariant packs as the primary covered graph obligation
  path
- "temporarily" bypassed validator maps with no residue manifest

## Related Docs

- [Graph Touch Obligation Authority](graph-touch-obligation-authority.md)
- [Consumer Kit](../foundations/consumer-kit.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
- [Graph Composition Authoring](graph-composition-authoring.md)
