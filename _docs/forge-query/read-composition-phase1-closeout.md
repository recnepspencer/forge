# Read Composition Phase 1 Closeout

This document freezes the `forge-query` graph-composition-for-reads Phase 1
kernel boundary and the public answer to “is Worth allowed to begin Phase 2
adoption yet?”

## Stable Surface

Public read-composition entry points:

- `compose_read`
- `compose_read_with_invariant_pack`
- `define_read_family`
- `define_read_family_with_invariant_pack`
- `execute_read_family`

Canonical graph artifacts:

- `read_graph`
- `read_result`
- `read_receipt`
- `typed_read_denial`

Scope classes:

- `local_neighborhood`
- `anchored_expansion`
- `explicit_broad_search`

Graph families:

- `detail`
- `collection`

Execution engines:

- `query_runtime_current`

Fallback classes:

- `none`
- `snapshot_indexed_debt`
- `whole_view_debt`

Built-in operators:

- `direct_edge`
- `successor_walk`
- `shared_endpoint`
- `shared_attachment`
- `bounded_ancestor`
- `bounded_descendant`
- `anchored_frontier`
- `frontier_search`

Relationship-proof postures:

- `not_required`
- `descriptor_admitted_synthetic_runtime`

Reusable family admission modes:

- `kernel_only`
- `domain_invariant_admitted`

Extension hooks:

- `domain_read_family_lowering`
- `domain_invariant_pack`
- `domain_decoder`
- `domain_result_certification`

Boundary guards:

- `operator_owned_builders_hide_traverse`
- `scope_class_relabeling_denies_typed`
- `built_in_operator_shape_denies_typed`
- `relationship_proof_admission_denies_typed`
- `domain_invariant_pack_denies_before_execution`

Denial lanes:

- `invalid_root`
- `built_in_operator_denied`
- `relationship_proof_admission_denied`
- `scope_shape_denied`
- `authoring_denied`
- `canonicalization_denied`
- `validation_denied`
- `planning_denied`
- `basis_resolution_denied`
- `basis_preflight_denied`
- `execution_denied`
- `domain_invariant_denied`

## Safe To Build Now

- compose_read, compose_read_with_invariant_pack, define_read_family, define_read_family_with_invariant_pack, and execute_read_family form one public read-composition product instead of separate helper stories
- the canonical read artifact is ReadGraph and every admitted execution returns a ReadReceipt with scope class, graph family, breadth, fallback posture, and relationship-proof admission identity
- the Phase 1 runtime taxonomy now freezes query_runtime_current together with none, snapshot_indexed_debt, and whole_view_debt fallback classes as public read-kernel posture
- scope classes are kernel-owned and freeze local_neighborhood, anchored_expansion, and explicit_broad_search at the shared boundary instead of letting callers relabel the same lowered read
- operator-owned graph lanes now cover direct_edge, successor_walk, shared_endpoint, shared_attachment, bounded_ancestor, bounded_descendant, anchored_frontier, and frontier_search
- traversal-bearing reads now admit descriptor-backed synthetic runtime relationship proof before execution instead of reporting only a receipt heuristic
- invariant packs can narrow an admitted read graph before execution and deny through a typed domain-invariant lane with an attached rejected-graph summary
- reusable ReadFamily artifacts are part of kernel completeness and keep admission history in their digest so invariant-admitted families do not collapse into plain reusable reads
- operator-owned builders keep traversal ownership mechanical because the exported operator-builder boundary hides traverse and is compile-fail certified
- later domain adoption must extend through the frozen read-composition hooks for domain_read_family_lowering, domain_invariant_pack, domain_decoder, and domain_result_certification instead of rebuilding a second local read stack

## Must Not Assume Yet

- do not assume Phase 2 Worth topology migration is complete; this closeout only certifies that the generic read kernel is ready for honest adoption
- do not assume the side quest is fully closed; Phase 3 still owes aggregate query-native-versus-fallback breadth and debt proof after Worth migration
- do not assume all future domain families already exist; later topology, trim, carrier, NURBS, fillet, and branch-history vocabularies still need domain-owned adoption on top of this kernel

## Migration Guidance

- start Phase 2 by moving one bounded Worth topology family such as LoopCycleNeighborhood onto compose_read plus a domain-owned decoded view
- bind the first Worth topology family through the frozen lowering, invariant-pack, decoder, and certification hook boundaries instead of inventing topology-local extension seams
- prefer an operator-owned read lane whenever the domain shape matches one of the admitted built-in operators instead of open-coding traverse in the Worth facade
- keep snapshot_index behind the Worth read facade only as explicit fallback debt and surface that fallback exactly on the returned receipt/view boundary
- do not resume Milestone 3 hostility widening until Worth migration lands and the side-quest aggregate closeout proof names any remaining fallback consumers as debt rows

## Phase Gate

- `phase_one_kernel_complete`
  - `satisfied`
  - the generic read kernel is frozen with stable operators, typed denials, reusable read families, typed extension hooks, and descriptor-backed relationship-proof admission
- `phase_two_worth_adoption_ready`
  - `satisfied`
  - Worth may begin domain adoption through the frozen lowering, invariant-pack, decoder, and certification hooks, starting with loop_cycle_neighborhood
- `phase_three_aggregate_proof_complete`
  - `blocked`
  - aggregate query-native-versus-fallback breadth and debt proof cannot complete until Worth migrates active topology families onto the frozen kernel
- `milestone_three_resume_ready`
  - `blocked`
  - Milestone 3 remains blocked until the side quest reaches Phase 3 aggregate closeout proof and names any remaining fallback consumers as debt rows

## Required Verification Commands

- `cargo fmt --package forge-query`
- `cargo test -p forge-query runtime::tests::read_composition --quiet`
- `cargo test -p forge-query --test phase_boundaries_compile_fail --quiet`
- `cargo test -p forge-query --quiet`
- `git diff --check`
