---
name: qa-tests
description: Review and correct WORTH tests and test harnesses as production evidence. Use when auditing fixture realism, integration or end-to-end claims, oracle independence, adversarial strength, redundant tests, compile-test inflation, or test cost.
---

# QA Tests

Attempt to falsify the suite's proof claims. A green test is evidence only for
the production defect it can independently expose.

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

## Correct and repeat

Fix production weakness in production rather than padding the test. Repair
fixture provenance and reuse architecture before counterfeiting inputs.
Consolidate redundant scenarios and compiler sessions instead of accumulating
more green artifacts.

After correction, rerun the relevant evidence, verify that the test fails under
the named fault where practical, and reassess the affected proof family.
Continue until no meaningful test or harness findings remain.

Completion requires causally valid worlds, honest boundary labels, independent
observations, intended-cause failures, adversarial evidence proportionate to
risk, and justified compile plus execution cost.
