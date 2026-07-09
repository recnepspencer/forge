Phase {phase.id}: {phase.title} has passed the required phase-done loop for the
WORTH Store Aspect-Native Workspace Gate.

Now run the non-looping hardening sequence below. These passes can produce work,
and you should implement the reasonable fixes they uncover, but do not force the
runner to loop on test purity, directory polish, or aerospace-grade status. The
only mandatory loop was whether the phase was actually done.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Acceptance evidence:
{phase.acceptance}

Run this sequence in chat:

1. [$qa-tests](C:\Users\Esther\.codex\skills\qa-tests\SKILL.md) Do not code
   yet. First, find everything weak, synthetic, or too self-referential in this
   phase's tests. Focus on whether the tests genuinely prove the production
   surface instead of fixture theater.
2. Tell me which production surfaces are missing to support honest tests.
3. Create an in-chat plan to fix those issues. Make sure it is principled,
   follows our arch laws, and respects current APIs.
4. Go implement that plan.
5. [$code-quality-qa](C:\Users\Esther\.codex\skills\code-quality-qa\SKILL.md)
   double check directories, file lengths, responsibility boundaries, names,
   and module topology for this phase.
6. Ask what is left before this phase can honestly be called gate-closed.
7. Create another in-chat plan to implement the reasonable phase-relevant
   remainder.
8. Go implement that plan.

Important boundaries:

- Close only what is actually closed.
- Do not claim whole-gate closure unless this is the final phase and all prior
  phases are complete.
- Do not claim aerospace-grade unless the evidence really supports it. It is OK
  to say what remains.
- Do not loop because tests could be stronger or because broader gate closure
  remains out of reach. Implement principled, phase-relevant fixes that are
  reasonable now, and record any larger remainder in chat.
- Do not put logs, artifacts, command tails, long QA lists, or plans into the
  JSON. The JSON is only progress tracking.

When this close pass is complete, update only lightweight JSON state:

- keep this phase `status: complete` and `qa_status: passed`
- add at most short `notes.done` / `notes.remaining` markers
- if a later phase exists, advance to that phase at turn `plan`
- if this was the last phase, set `current` to null and set `completed_at`

{contract}
