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

Work this phase's category plan item by item. Either complete the replacement
and delete/collapse the prior production path in the same phase, or record the
exact mechanical blocker that prevents the hard break. Do not leave both the old
production path and the new proof path alive as a convenience bridge. Slow
conversions are failed implementations unless the old path is certification-only,
capped residue, or a named query/runtime gap.

Treat adapter creation as a defect until proven otherwise. Do not add a shim,
wrapper, bridge, compatibility facade, pass-through conversion, or old-to-new
adapter to make callers easier unless you also prove it is mechanically barred
from production authority and record the cap/removal trigger. If the work feels
like it needs an adapter, first try the harder cleanup: change the callers,
delete the old path, and make the new proof type the only ordinary route.

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
