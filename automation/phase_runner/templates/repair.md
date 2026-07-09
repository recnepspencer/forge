Now lets create an in-chat plan to fix the phase {phase.id}: {phase.title}
done-check issues. Make sure it is principled, follows our arch laws, follows
our perf laws, and respects our current APIs.

Then go implement that plan.

Config file: {config_file}
Projection file: {projection_file}
Event log file: {event_log_file}
Spec file: {spec_file}
Run id: {run_id}
Cursor: phase {current.phase}, turn {current.turn}

Phase scope:
{phase.scope}

Acceptance evidence:
{phase.acceptance}

Open done-check summary from projection:
{phase.notes.findings}

Use the detailed findings from the previous chat turn as the real repair input.
The projection summary is only a pointer, not the artifact of record.

Repair rules:

- Operator correction for this repair:
  - Do not invent or demand a bespoke Query-owned "gap ledger" artifact.
  - Use Query surfaces for what they actually mean:
    - support and admission posture prove whether a Query capability is supported, deferred, or unsupported
    - consumer residue audit proves whether Worth is rebuilding Query proof locally
  - Do not collapse those into one fake unified artifact.
  - If Worth still has a surviving wrapper, alias, adapter, local scanner, local manifest, or local second-ontology lane, classify that as Worth-local explicit residue and fix it here.
  - If a migration is blocked because Query does not support a required ordinary-path capability, name the exact missing capability and treat it as a blocker to fix, not as a standing debt ledger.
- If you cannot name the exact missing Query capability, do not manufacture a Query-gap row. Treat it as Worth-local residue or a bad test assumption and fix the model accordingly.
- Before choosing a fix, classify the problem as one of:
  - `local fix`
  - `structural fix`
  - `phase-scope mismatch`
- Choose `local fix` only if the defect is isolated and fixing it will not
  leave the same authority gap behind.
- Choose `structural fix` if the finding reveals a missing ordinary lane,
  synthetic authority, fake proof path, or boundary collapse.
- Choose `phase-scope mismatch` only if the review is demanding a surface this
  phase does not honestly own.
- State the classification first in chat.
- If the correct classification is `structural fix`, do not propose a narrow
  patch that only silences the current finding.
- Fix the cause, not the symptom.
- Fix the full visible finding family, not just the first file that failed.
- If the previous review identified repeated or shared findings, start by
  naming the root cause category: wrong ownership boundary, WORTHable authority,
  certification-overreach, count/projection pretending to be proof, adoption not
  tied to production API, or missing non-WORTHability proof.
- If this phase has already failed review two or more times, assume the current
  defects may be one unresolved seam unless you can prove they are unrelated.
- Before editing after repeated review failures, answer briefly in chat:
  - what is the one canonical artifact/witness/contract that should own the
    truth
  - what still derives from a proxy, heuristic, mutable field, raw digest, or
    displaced legacy lane instead
  - whether the hostile proof currently exercises the real production derivation
    path or a synthetic shortcut
- When the root cause is architectural, repair the architecture. You have
  permission to create or move a lower Store vocabulary/contract surface, seal
  constructors, replace public data bags with private-field witnesses, move
  authority from certification into the owning runtime/lower crate, replace
  count summaries with typed lower evidence, and add compile-fail/API-misuse
  tests proving the old shortcut is impossible.
- Do not satisfy a review finding by replacing one proxy with a slightly nicer
  proxy if the spec requires a runtime-owned admitted artifact.
- Do not patch hostile proofs with synthetic inputs when the review says the
  production derivation seam is still unsealed.
- When the defect family is WORTHable authority, proxy identity, weak
  equivalence, mutable-boundary leakage, or displaced ordinary authority,
  prefer one end-to-end seam rewrite over multiple local edits that leave the
  same weaker source alive underneath.
- Mint canonical witnesses once, admit them once, preserve them end-to-end, and
  cut downstream identity/equivalence/inspection consumers over to them
  directly.
- Do not add another denial wrapper around a WORTHable path. Remove or seal the
  path that made the impossible state constructible.
- Certification is the courtroom, not the law. It may materialize and prove
  executed Store law; it must not mint runtime authority, lower Store witnesses,
  proof freshness, readmission, or counter strength on its own.
- Do not weaken tests or rename debt to make findings disappear.
- Do not keep old authority alive through adapters, shims, wrappers, bridges, or
  compatibility facades unless they are mechanically barred from ordinary
  production authority.
- If review named multiple independent leaks in the same cutover family, close
  them in this turn unless one of them is genuinely blocked by a deeper seam
  you are already replacing.
- Keep the repair scoped to making this phase actually done.
- Put the repair plan, implementation explanation, and any important evidence in
  chat, not in runner authority files.
- Do not emit `repair_blocked` or any blocked-style runner event. The runner
  does not support it. If the phase still is not done after repair, either finish
  a real repair with `repair_completed` or leave the next review to emit
  `review_failed` with sharper findings.

In the plan or implementation explanation, say briefly:

- what the real root cause is
- why the chosen fix removes that root cause
- what adjacent future finding this fix is intended to prevent

Do not reward issue-by-issue patching if multiple findings are caused by the
same missing authority seam. Prefer one deeper fix over several shallow fixes
when they share a root cause.

After repair, finish with:

`RUNNER_EVENT: {"event_type":"repair_completed","payload":{"notes":{"done":["..."],"verification":["..."]}}}`

Do not stop at architectural analysis. If you can name the real seam, implement
it in this turn and advance the runner honestly.

Phase-specific instructions:
{phase.instructions}

{contract}
