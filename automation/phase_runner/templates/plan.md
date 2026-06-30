Now lets create an implementation plan for phase {phase.id}: {phase.title}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Do the following:

- Make sure to follow our arch and perf laws.
- Review relevant context, including the spec, this phase's scope paths, and the
  relevant APIs.
- Read `_docs/more_guidelines/dx_laws.md` if it exists.
- Plan the directory skeleton explicitly.
- Plan the DX target via DX laws as an actual Rust code block target.
- Make implicit requirements explicit.
- Build the plan inline in the chat, not in an md file.
- Include the phase-relevant plan.
- Be explicit about what each step requires, changes, and proves. Do not give me
  a loose list of adhd bullet points.

The plan should cover:

1. Relevant context read and what it constrains.
2. The adversarial constraint this phase must survive.
3. The DX target as a Rust code block.
4. The directory skeleton and each file/module's responsibility.
5. Implicit requirements made explicit.
6. The implementation sequence, with requirements, edits, proof, and blockers.

Phase-specific instructions:
{phase.instructions}

After posting the plan in chat, update the JSON state file directly: set this
phase to `in_progress`, keep only a short `notes.plan` marker, and advance the
cursor to `implement`.

{contract}
