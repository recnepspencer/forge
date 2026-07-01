Now create an in-chat plan to fix the test QA findings for phase {phase.id}:
{phase.title}.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Open test QA summary from JSON:
{phase.notes.findings}

Use the detailed findings from the previous chat turn as the real repair input.
The JSON summary is only a pointer, not the artifact of record.

Plan only. Do not implement yet.

The plan must distinguish clearly between:

- test changes
- missing production surfaces that must be added so tests can use real code
- proof upgrades needed in verification lanes
- any residue that should remain explicit debt instead of being faked

Aggression rules for this plan:

- Produce one implementation path, not a menu.
- Collapse sibling findings into one root-cause repair sequence wherever
  possible.
- Prefer adding or cutting over to a real production seam over inventing a
  certification-only helper.
- If the correct fix is obvious after reading the findings and touched APIs,
  keep the plan short and executable instead of expanding it into another review.

Make the plan specific enough that the next implementation turn can execute it
literally.

After posting the plan, update the JSON state file directly:

- short `notes.plan`
- cursor turn `test_repair_implement`
- this turn is a planning turn only; do not leave `current` on
  `test_repair_plan` after writing the plan
- if the phase row is already `status: complete` and `qa_status: passed`, the
  only valid same-phase next turn from this prompt is `test_repair_implement`
  because you are explicitly planning remaining hardening work

{contract}
