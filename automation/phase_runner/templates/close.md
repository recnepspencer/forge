Phase {phase.id}: {phase.title} has passed the required done-check loop.

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
   yet. First, find everything weak or synthetic in our tests. Deliver me a
   list. Then tell me which production surfaces are missing to support them
   honestly so that we can make them completely real.
2. Now lets create an in-chat plan to fix those issues. Make sure it is
   principled and follows our arch laws and respects our current APIs.
3. Now go implement that plan.
4. [$code-quality-qa](C:\Users\Esther\.codex\skills\code-quality-qa\SKILL.md)
   now lets double check our directories and file lengths.
5. Now what is left before we can call this aerospace grade?
6. Create another in-chat plan to implement that.
7. Now go implement that plan.

Important boundaries:

- Do not claim aerospace-grade unless the evidence really supports it. It is OK
  to say what remains.
- Do not loop because tests are not perfect or because aerospace-grade remains
  out of reach. Implement the principled, phase-relevant fixes that are
  reasonable now, and record any larger remainder in chat.
- Do not put logs, artifacts, command tails, long QA lists, or plans into the
  JSON. The JSON is only progress tracking.

When this close pass is complete, update only lightweight JSON state:

- keep this phase `status: complete` and `qa_status: passed`
- add at most short `notes.done` / `notes.remaining` markers
- if a later phase exists, advance to that phase at turn `plan`
- if this was the last phase, set `current` to null and set `completed_at`

{contract}
