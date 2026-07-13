## M1B scope adjudication (mandatory)

The milestone specification is authority. A review finding is a claim that must
be admitted against the current phase before it can block work.

For every proposed blocking finding, establish all four fields:

- `spec_basis`: quote or precisely cite the current phase sentence that requires it
- `reachable_harm`: name the admitted public or immediate trusted-owner path that remains wrong
- `minimal_fix`: name the smallest common-cause correction required for current acceptance
- `owner_phase`: explain why this phase owns it instead of a later M1B phase

A finding missing any field is non-blocking follow-up commentary. Do not write
it into the findings artifact and do not fail the phase for it.

Hard scope ceiling:

- honor the phase's Warnings, Engineering decisions, and explicit exclusions
- do not demand full Rust/Cargo/type resolution when the spec declares a
  name-based or snapshot-backed boundary
- do not pull Phase 5 source-law breadth, Phase 6 ratchet closure, Phase 7
  diagnostics, or Phase 8 corpus exhaustiveness into an earlier phase
- do not generalize from one hostile example into a compiler-completeness
  requirement unless current acceptance explicitly requires that completeness

Before editing, classify each received finding as `admitted blocker`,
`phase-scope mismatch`, or `follow-up hardening`. Repair only admitted blockers.
It is valid to make no code change and complete the repair turn when all received
findings are scope mismatches or follow-up hardening; record that adjudication in
chat and emit the normal successful event for the turn.

Stop when the current phase's named acceptance and required hostile proofs pass.
Do not keep the phase open to eliminate hypothetical deeper bypasses owned by
later phases or trusted-internal hardening.
