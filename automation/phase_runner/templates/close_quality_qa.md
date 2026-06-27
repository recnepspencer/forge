Run only the structural code-quality QA discovery pass for phase {phase.id}:
{phase.title}.

Do not create a fix plan. Do not edit code. Do not repair issues in this turn.

State file: {state_file}
Spec file: {spec_file}
Cursor: phase {current.phase}, turn {current.turn}

Use this skill:

[$code-quality-qa](C:\Users\Esther\.codex\skills\code-quality-qa\SKILL.md)

Double check directories, file lengths, responsibility boundaries, names, helper
placement, module topology, and public facade shape for this phase. Then answer
what remains before this phase can honestly be called aerospace-grade, without
claiming that it is aerospace-grade unless the evidence supports it.

Output the findings in chat. In the JSON state, record only short findings or
remaining markers, keep `status: complete` and `qa_status: passed`, and advance
the cursor to `close_quality_plan`.

Do not put logs, artifacts, command tails, long QA lists, or plans into the
JSON.

{contract}
