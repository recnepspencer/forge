You are implementing {project.name}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}
Phase: {phase.id} - {phase.title}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Work this phase's plan in `notes.plan`. If no usable plan exists, write a concise
one before editing. Continue from the nearest unfinished item.

Enforce mechanically, not by convention: make invalid states unrepresentable,
push violations to compile errors or tests, and prefer `pub(crate)` over a
comment asking callers to behave. Scope expansion is the norm, not the
exception — if the real fix lives behind a blocker in another module or crate,
expand into it and build the blocker. Do not widen an API, expose a raw seam, or
leave an escape hatch to dodge hard work, and do not mark debt unless the
blocker fix is genuinely major and mechanically contained, exactly as the debt
law requires.

Verify what you changed and record what proves it in `notes.verification` before
you claim any status.

Phase-specific instructions:
{phase.instructions}

Record what changed, what proves it, and what remains in this phase's notes; set
status by the contract; advance the cursor.

{contract}
