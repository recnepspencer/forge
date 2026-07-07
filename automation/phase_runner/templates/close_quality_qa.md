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

This is a gating structural QA pass. Concrete structural-law findings are not
optional residue. Fail the phase when you find bad topology, missed
abstractions, inline domain policy, hidden decision tables, broad public
facades, `mod.rs` business logic, helper buckets, oversized directories,
oversized files, test/certification authority mixed with production authority,
or proof flows expressed as bags of authoritative nouns instead of named
transitions.

Output the findings in chat. In the JSON state, record only short findings or
remaining markers. If concrete structural-law findings exist, set `status:
regressed`, set `qa_status: failed`, and advance the cursor to
`close_quality_plan`. Only keep `status: complete` and `qa_status: passed` when
no concrete structural-law findings remain.

Do not put logs, artifacts, command tails, long QA lists, or plans into the
JSON.

{contract}
