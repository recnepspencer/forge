You are closing {project.name}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}
Phase: {phase.id} - {phase.title}

Acceptance evidence (the closeout checklist — run every runnable item now):
{phase.acceptance}

Laws and context to hold as the standard:
{project.context_files}

Close only what is actually closed. The façade is the only surface: confirm the
phase exposes its result through ordinary public APIs and that internal types,
raw rows, and forgeable receipts cannot satisfy the contract from outside. API
presence is not proof — a method that exists is not a method that proves
anything. Where the phase claims a property, confirm the structure enforces it.

Re-run the acceptance checks and record command, exit code, and output tail in
`notes.verification`. Summarize the final proof, the explicit residue or query
gaps that remain — named and owned per the debt law, not silently dropped — and
the exact verification commands. Then set `status` and `qa_status` by the
contract.

Run a structural closeout checkpoint before advancing:

- confirm the implemented directory skeleton matches the phase plan
- list touched code and test files with line counts
- split touched Rust files over 400 lines unless an explicit exemption exists
- reject broad bucket files, vague helper modules, and god functions
- confirm public facades aggregate read-only proof/status and do not implement
  admission authority
- confirm no adapter, shim, bridge, compatibility facade, local report, raw row,
  copied field set, broad scan, or topology/Query substitution path can satisfy
  the phase's ordinary authority contract
- confirm tests include real public-boundary or integration proof, not only
  helper-local synthetic fixtures

If this phase has a deletion ledger, closeout must audit the actual exported
symbols and files, not only the ledger row labels. Do not advance while a target
listed as `delete` or `collapse` is still available through an ordinary public
facade, or while a local ceremony/local guard/proof-obligation surface remains
public without an explicit certification-only boundary or capped residue row.
When in doubt, mark the phase `regressed`, record the public `file:line`, and
return to `repair`.

This is the only turn allowed to advance to the next phase: if a later phase
exists, set the cursor to it at turn `plan`; if this was the last phase, set
`current` to null and set `completed_at`. Do this only when the phase is
`complete` and `qa_status` is `passed`.

{contract}
