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

Before planning, read the milestone spec, this phase's scope paths, the relevant
public APIs the phase will touch, arch_laws.md, perf_laws.md, and
_docs/more_guidelines/dx_laws.md if it exists. Record the files and APIs you
actually inspected in this phase's `notes.plan`; do not claim context you did
not read.

Before anything else, state the adversarial constraint: the single condition
that would break a naive implementation of this phase at production scale. State
it precisely, and quantitatively where you can. The plan exists to survive that
constraint; types, modules, and tests are how it survives. If you cannot state
the constraint, you do not yet understand the phase well enough to plan it —
read the spec and code until you can.

Then write or update the implementation plan in this phase's `notes.plan`. The
spec already owns the high-level phase order. Do not reorder phases, split this
phase into new runner phases, or redesign the milestone sequence here. Inside
this single phase, build a detailed category plan so the next implement turn can
work linearly without guessing.

The plan must have these checkpoints, in this order:

1. Adversarial constraint.
   State the production-scale failure mode this phase must survive.
2. Relevant context read.
   Name the spec sections, source files, public APIs, and laws inspected, and
   say what each one constrains.
3. DX target.
   Write the intended caller experience as a Rust code block. This code block is
   a contract target, not decoration. It should show the ordinary route a caller
   should use after this phase and the forbidden route the phase blocks when
   that is clarifying.
4. Directory skeleton.
   Name the exact modules/files that should exist or change after this phase,
   and give each one a single responsibility. Avoid broad bucket modules.
5. Implicit requirements made explicit.
   Spell out authority boundaries, denial cases, deletion/collapse obligations,
   performance locality, public facade behavior, replay/determinism needs, and
   test honesty requirements that the spec implies.
6. Phase-local implementation sequence.
   Give a linear plan with concrete source surfaces, proof boundary, first hard
   break or deletion attempt, tests, structural evidence, and exact blockers.

Do not write a loose bullet cloud. Every plan item must say what it requires,
what it changes, and what proves it.

Use these planning categories as the default vocabulary, omitting only those
that are genuinely out of scope for this phase:

- authority/proof-bearing types and sealed constructors
- operator/request intent lowering
- Query or runtime selection/admission
- validators, invariants, and denial posture
- derived invalidation, dirty propagation, replay, undo, and transaction scope
- evidence lookup, indexing, receipts, and ledger identity
- conflict, independence, cache, equivalence, and reuse
- public facade, diagnostics, counters, and closeout proof
- hard deletion/collapse of replaced surfaces
- compile-fail, hostile tests, line-cap, and composition QA

For every relevant category, give it its own mini-plan with the concrete source
surfaces to inspect or edit, the authority/proof boundary it must establish, the
first hard break, deletion, or collapse it should attempt, the tests or
structural evidence that will prove the category is done, and any exact blocker
that would make the category capped residue or a Query gap.

Add an explicit adapter purge note to every relevant category. Name any existing
adapter, shim, bridge, wrapper, compatibility facade, conversion helper, or old
path kept alive beside the new proof path. The default plan is to delete it and
clean up callers. A plan that keeps it must classify it as certification-only,
capped residue, or Query/runtime gap with owner, cap, removal trigger, and the
test that proves it cannot satisfy ordinary authority APIs. Do not use
"temporary" or "compatibility" as justification; those words are warning signs,
not reasons.

Sequence the phase-local category plan so the hard proof is built before
dependent cleanup or facade work. Keep it concrete enough that another agent can
continue it cold with no chat history. A slow conversion plan that leaves the
old production path alive after its replacement exists is not acceptable.

Phase-specific instructions:
{phase.instructions}

Set this phase `status` to `in_progress` unless it is genuinely blocked, record
the plan, then advance the cursor.

{contract}
