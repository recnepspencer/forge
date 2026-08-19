---
name: qa-tests
description: Review and correct WORTH tests and test harnesses as production evidence with a fresh independent agent or CLI critic. Use when auditing fixture realism, integration or end-to-end claims, oracle independence, adversarial strength, redundant tests, compile-test inflation, test cost, or when outside-eyes review of a suite's proof claims is required.
---

# QA Tests

Attempt to falsify the suite's proof claims. A green test is evidence only for
the production defect it can independently expose.

## Independent review

Launch a fresh, read-only independent test critic at the start. Do not wait
for a self-exhausted primary pass. The critic is an independent evidence source
for missed proof defects, not a substitute for the primary agent's repository
work.

If the user names a reviewer, model, or CLI, use it. Valid choices may include
a fresh Codex agent, Claude CLI, Cursor Agent, Grok, or another user-supplied
non-interactive review command. Do not invent commands, install tools,
authenticate accounts, or grant the critic write authority.

If the user does not specify a reviewer, default to Luna Max and proceed
without blocking for a choice. If neither Luna Max nor a user-selected reviewer
can be launched, report the blocker; the primary agent cannot serve as its own
independent critic.

Give the critic a compact, source-bound packet of raw evidence only:

- repository instructions, governing specification, and test requirements
- the frozen revision or deterministic source fingerprint
- the scoped production and test diff
- relevant fixtures, harnesses, assertions, and real production boundaries
- any search-coverage already recorded; do not delay launch to finish a
  complete manifest
- test commands, output, timings, retained artifacts, and known environment
  constraints
- a neutral request to identify proof claims that can remain green for the
  wrong reason

Do not disclose the primary agent's findings, planned corrections, or expected
answer before the critic's first pass. Record the critic identity, model or
command, source revision, scope, prompt, and complete findings. Independently
verify each finding in source or execution; outside review supplies hypotheses
and adversarial perspective, not authority.

## Establish the proof surface

Read the repository instructions, governing specification and test
requirements, `testing_laws.md`, the relevant production paths, and all fixture,
harness, and assertion code involved.

For each material production guarantee, identify:

- the plausible defect the suite must expose
- the real authority or integration boundary implicated by that defect
- the world and causal preconditions required to reach it
- the observation that distinguishes correct behavior independently

## Review

Trace setup, action, observation, and teardown rather than trusting test names.
Apply these lenses where relevant:

- **World honesty:** Valid identities, authority, relationships, revisions, and
  persisted facts have causal provenance. Canonical worlds and isolated deltas
  reuse expensive setup without shared mutable fate.
- **Boundary honesty:** Integration and end-to-end labels match the production
  composition, authority, persistence, protocol, and observation paths actually
  exercised.
- **Oracle independence:** Expected results do not reuse the semantics under
  test, and failures are shown to occur for the intended cause.
- **Fault pressure:** Tests cover the dangerous semantic classes and relevant
  denial, cancellation, concurrency, exhaustion, recovery, migration, and
  partial-effect paths rather than multiplying cooperative examples.
- **Proof economy:** Every test owns distinct evidence. Local tests, mocks,
  snapshots, and compile-fail cases exist only where they are the cheapest
  honest proof of a real contract.
- **Harness integrity:** Test support preserves production invariants, exposes
  phase failures, and obeys production composition and domain structure.
- **Cost honesty:** Cargo targets, compiler sessions, fixture construction,
  external startup, retries, and retained artifacts are intentional and
  proportionate to the evidence produced.

Do not reject a test merely because it is local or synthetic. Reject it when
its boundary, fixture, oracle, or cost cannot support the claim made for it.

## Findings

Report findings before summaries. For each finding state:

1. affected production claim
2. concrete weakness and evidence
3. whether the defect belongs to production, fixture, harness, or test
4. required correction
5. stronger proof that would close the finding

Do not report coverage percentage, assertion count, or stylistic preference as
proof weakness by itself.

Reconcile the primary review and the critic against repository evidence. Add
critic findings only when that evidence supports them. Record rejected findings
with the evidence that disproves them so consensus is never mistaken for proof.

## Correct and repeat

Fix production weakness in production rather than padding the test. Repair
fixture provenance and reuse architecture before counterfeiting inputs.
Consolidate redundant scenarios and compiler sessions instead of accumulating
more green artifacts.

Do not keep expanding the tests into a general Rust compiler, public-API analyzer, macro expander, or complete name-resolution engine merely to guard against hypothetical future code.

After correction, rerun the relevant evidence, verify that the test fails under
the named fault where practical, and reassess the affected proof family.
Continue until no meaningful test or harness findings remain.

When corrections materially change the production boundary, fixture
provenance, oracle, harness topology, or claimed test form, obtain a closure
pass from a new critic instance against the final revision. Do not reuse the
initial critic's context or conclusions. The replacement critic defaults to
Luna Max unless the user specified another reviewer. Never use the critic as an
ongoing repository search assistant.

Completion requires causally valid worlds, honest boundary labels, independent
observations, intended-cause failures, adversarial evidence proportionate to
risk, justified compile plus execution cost, and a recorded outside-eyes pass
with no unresolved supported finding. Report which reviewer was used and which
critic findings were accepted or rejected and why.
