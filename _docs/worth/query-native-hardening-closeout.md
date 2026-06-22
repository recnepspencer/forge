# Worth Query-Native Hardening Closeout

## Gate Result

The Worth Query-Native Hardening Gate is closed for phase 9 implementation
evidence. `worth-kernel`, `worth-spatial`, and `worth-topo` now publish
Query-native adoption, synthetic-proof disposition, authority-boundary,
composition-honesty, and performance-counter reports that are machine-checked
from production surfaces.

This closeout is not authority by itself. The authority is the machine-readable
report exposed by `worth-kernel::query_adoption` and the crate-local reports it
composes.

## Machine-Checked Counts

```text
audited_source_sets: 17
admitted_source_sets: 9
denied_source_sets: 5
explicit_residue_source_sets: 3
support_requirements: 7
support_observed_rows: 8
support_matched_required_rows: 7
support_snapshot_rows_evaluated: 68
boundary_audit_sources: 5
synthetic_denial_localization_rows: 5
kernel_receipt_families: 8
lower_crate_receipt_families: 2
topology_read_touched_scope: 4
spatial_witness_resolution_requests: 8
spatial_witness_denials: 4
spatial_witness_catalog_lookups: 2
```

## Admitted Surfaces

Admitted surfaces are the production source sets in the cross-crate inventory.
They include Query-owned support and evidence facilities, topology runtime
boundary and workload rails, spatial witness and workload evidence rails, and
kernel workload composition over lower-crate receipts.

The admitted count is not hand-maintained here. It must agree with
`WorthQueryAdoptionInventoryReport` and
`WorthQueryNativeHardeningCloseoutReport`.

## Denied Surfaces

Denied surfaces are certification-only or test-support-only synthetic proof
families. They may remain as hostile tests or certification fixtures, but they
cannot contribute support, evidence, runtime, or workload closeout proof.

The denied count is derived from
`WorthQuerySyntheticProofDisposition::DeniedByBoundary`.

## Explicit Residue

Remaining residue is explicit and diagnostic:

- `crates/worth-kernel/src/query_adoption/residue.rs`
- `crates/worth-spatial/src/query_adoption/residue.rs`
- `crates/worth-topo/src/query_adoption/residue.rs`

No unnamed debt is accepted by this gate. These residue surfaces are not
production closeout proof.

```text
kernel_residue_owner: worth-kernel/query_adoption
kernel_residue_blocker: binding tests still need Milestone 6.5 workload-platform binding admission lanes before they can retire diagnostic residue
kernel_residue_follow_on_milestone: Milestone 6.5
spatial_residue_owner: worth-spatial/query_adoption
spatial_residue_blocker: workload-platform vocabulary rows still need Milestone 6.5 full Query report pinning before they can retire diagnostic residue
spatial_residue_follow_on_milestone: Milestone 6.5
topology_residue_owner: worth-topo/query_adoption
topology_residue_blocker: runtime support reporting still needs Milestone 6.5 workload-platform closeout alignment before it can retire diagnostic residue
topology_residue_follow_on_milestone: Milestone 6.5
```

## Verification Commands

```text
cargo fmt -p forge-query -p worth-kernel -p worth-spatial -p worth-topo
cargo test -p forge-query --lib consumer_kit -- --nocapture
cargo test -p forge-query --test evidence_report_compile_fail -- --nocapture
cargo test -p forge-query --test prohibition_registry_compile_fail -- --nocapture
cargo test -p forge-query --test support_pinning_facade -- --nocapture
cargo test -p forge-query --test support_snapshot_facade -- --nocapture
cargo test -p forge-query --test in_memory_test_backend_facade -- --nocapture
cargo test -p worth-kernel query_adoption -- --nocapture
cargo test -p worth-spatial query_adoption -- --nocapture
cargo test -p worth-topo query_adoption -- --nocapture
cargo test -p worth-topo runtime_boundary -- --nocapture
cargo test -p worth-spatial boolean_readiness_workload -- --nocapture
cargo test -p worth-kernel query_native_hardening_closeout -- --nocapture
```

## AI-Facing Agreement

`crates/forge-query/docs/AI_README.md` is part of the closeout proof. It must
name the Query `Consumer Kit` as the downstream proof surface and keep the
covered families explicit: evidence reports, hard-prohibition audits, support
snapshots, support pins, in-memory test workspaces, and adoption/residue proof.

`WorthQueryNativeHardeningCloseoutReport` includes AI_README agreement in
`gate_closed()`, so this closeout cannot pass if the AI-facing mental model
for the Consumer Kit drifts away from the machine-checked Worth gate.

## Roadmap Dependency

`Milestone 6.5` and `Milestone 7` depend on this gate. They may consume the
Query-native Worth rails, but they may not reopen pre-Query synthetic fixtures,
hand-built evidence rows, kernel-forged lower-crate proof, or unmeasured broad
runtime paths as closeout evidence.
