You are planning {project.name}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}
Phase: {phase.id} - {phase.title}

Phase owner:
{phase.owner}

Phase scope:
{phase.scope}

Acceptance evidence (the checklist this plan must drive toward):
{phase.acceptance}

Before anything else, state the adversarial constraint: the single condition
that would break a naive implementation of this phase at production scale. State
it precisely, and quantitatively where you can. The plan exists to survive that
constraint; types, modules, and tests are how it survives. If you cannot state
the constraint, you do not yet understand the phase well enough to plan it —
read the spec and code until you can.

Then write or update the implementation plan in this phase's `notes.plan`. Solve
the hard problem first: sequence the work so the load-bearing infrastructure is
built before the features that depend on it. Keep the plan concrete enough that
another agent continues it cold, with no chat history.

Phase-specific instructions:
{phase.instructions}

Set this phase `status` to `in_progress` unless it is genuinely blocked, record
the plan, then advance the cursor.

{contract}
