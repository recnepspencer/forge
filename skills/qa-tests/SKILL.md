---
name: qa-tests
description: Run a WORTH-quality hostile review of tests and test harnesses. Use when deciding whether tests are as clean as production code, are real end-to-end or integration proof, and are strong enough to expose weaknesses in production code rather than just passing.
---

# QA Tests

Use this skill when implementation exists but the tests themselves need hostile
review.

Be skeptical and anal.

Assume a test can look clean, modern, exact, and well-factored while still
proving something weak. Do not grade on presentation. Do not relax because a
test looks professional. Stay suspicious until the proof pressure is clearly
real.

Keep the bar simple and hard:

- test code must be as clean as production code
- shared test abstractions must be as clean as production abstractions
- tests must prove real system behavior through end-to-end or integration
  pressure
- synthetic-only, theater, and happy-path-only tests are banned
- the point is to flush out production weakness, not to make the test pass with
  glue, strings, or harness tricks

## Read first

Read these before reviewing tests:

1. `C:\Users\Esther\Documents\Programming\WORTH_workspace\WORTH\_docs\coding_guidelines\MENTALITY.md`
2. `C:\Users\Esther\Documents\Programming\WORTH_workspace\WORTH\_docs\coding_guidelines\arch_laws.md`
3. `C:\Users\Esther\Documents\Programming\WORTH_workspace\WORTH\_docs\coding_guidelines\composition_laws.md` if it is populated
4. `C:\Users\Esther\Documents\Programming\WORTH_workspace\WORTH\_docs\coding_guidelines\domain_structure_laws.md`
5. `C:\Users\Esther\Documents\Programming\WORTH_workspace\WORTH\_docs\coding_guidelines\perf_laws.md`
6. the governing spec or milestone
7. the milestone test requirements, if they exist
8. the relevant test files and test-support files

## Standard

Review tests like a hostile engineer.

Do not ask:

- do they pass?
- is there coverage?
- is there at least one denial test?

Ask:

- do these tests exercise the real system boundary?
- do they try to break the code?
- do they prove hard behavior, not just success?
- is the harness honest?
- is the test code clean enough to belong in production?

## Hard rules

### 1. Tests are production code

Messy tests are banned.

That includes:

- test files
- fixtures
- support modules
- setup helpers
- assertion helpers
- shared harness layers

If the code would be unacceptable in `src/`, it is unacceptable in tests.

### 2. Prefer end-to-end and integration proof

The default target is end-to-end or integration coverage of real behavior.

Unit-style tests are acceptable only when they prove a narrow contract that
cannot be covered honestly through a stronger surface.

If a test can be moved up to a more real boundary, prefer that.

Do not treat "the production extractor path is real" as equivalent to "the test
is truly end-to-end."

End-to-end means the whole test construction path is real:

- setup
- request or input construction
- ownership and authority path
- execution path
- output and failure verification path

If only the thing being tested is real, but the surrounding construction is
synthetic, pre-solved, or helper-softened, the test is not truly end-to-end.

This must be explicitly verified. Do not infer it from presentation or from one
real seam.

Manual verification is required before certifying that a test is genuinely
end-to-end.

### 3. Fake proof is banned

These are bad by default:

- synthetic-only tests
- theater tests
- happy-path-only tests
- tests that mostly prove the harness
- tests that pass by adding strings, glue, mocks, or convenience shims around
  the real weakness

If the production code is weak, fix production code or prove the weakness
explicitly. Do not pad the test until it turns green.

### 4. Shared abstractions must be honest

Shared test helpers must clarify proof, not hide it.

Do not accept helpers that:

- merge distinct semantics
- hide edge conditions
- inject behavior the real system does not have
- make failures harder to diagnose
- exist only to make brittle tests easier to write

### 5. Edge pressure is mandatory

Prioritize:

- boundary cases
- denial paths
- replay and retry behavior
- stale state and precondition drift
- malformed inputs
- lifecycle interruptions
- integrity mismatches
- parity and exactness checks

If the suite mostly proves the obvious path, it is weak.

### 6. No cleanup theater

Do not "improve the tests" by:

- weakening assertions
- snapshotting noise instead of meaning
- adding brittle helper layers
- hard-coding fixture residue
- asserting on incidental strings instead of real structure

The goal is stronger proof and cleaner code, not easier passing.

## Synthetic-test heuristics

These are not always conclusive, but they are strong warning signs that a test
has been faked to look real.

### 1. The test hard-codes its own success

Signs:

- it invents the exact strings, ids, ordering, digests, or summaries it later
  "verifies"
- it asserts fixture residue instead of contract meaning
- it uses vague checks like `contains(...)` where structured truth should be
  asserted

### 2. The test dodges the real boundary

Signs:

- it calls lower-level helpers instead of the real integration seam
- it stubs, disables, or shortcuts the subsystem most likely to fail
- it proves a miniature model of production instead of production behavior
- it contains one real production path but the rest of the test construction is
  still synthetic

### 3. The harness is doing the work the product should have to do

Signs:

- helpers inject support, state, auth, identity, ordering, or cleanup that real
  callers must earn
- shared abstractions smooth over lifecycle edges, denial paths, or integrity
  checks
- the suite passes because the harness made the world nicer than production

### 4. The pressure is curated, not hostile

Signs:

- inputs are unnaturally clean, complete, ordered, and cooperative
- the suite proves happy path plus toy invalid inputs, but skips stale state,
  retries, interruption, near-miss preconditions, malformed-but-plausible
  input, and partial progress
- it never tests the cases most likely to expose an expensive lie

### 5. The proof is circular

Signs:

- the test mostly checks that fake path A agrees with fake path B
- setup, execution, and assertions all depend on the same local assumptions
- a subtle dishonest implementation could still pass because the test only
  verifies a green outcome instead of the hard invariant

## Reasons to be skeptical

These are warning signs that a single test may be bench-maxed to look stronger
than it is.

### 1. The test looks cleaner than its proof actually is

Signs:

- beautiful naming, tidy decomposition, and polished structure
- but underneath the presentation, the test still only proves an easy outcome

### 2. The helper boundaries are disguising the real pressure surface

Signs:

- important setup, normalization, or assertions are pushed into tasteful shared
  helpers
- the test reads elegantly, but you can no longer see whether the hard part is
  actually under pressure

### 3. The test asserts something exact, but not something important

Signs:

- it checks a digest, receipt, snapshot, or structured output very precisely
- but that exactness is attached to a low-risk artifact instead of the
  dangerous contract edge

### 4. The test reaches the real seam, but only after pre-solving the problem

Signs:

- it uses the real entry point
- but the fixture, helper, or setup path has already satisfied the hard
  conditions that should have been under pressure

### 5. The test is sharp about output and blurry about cause

Signs:

- it can tell you that the final artifact is wrong
- but not whether the problem came from authority, lifecycle, replay,
  integrity, ordering, or precondition handling

### 6. The abstractions are tasteful enough that you stop questioning them

Signs:

- nothing looks obviously magical
- but a helper may be encoding assumptions, hiding branches, or collapsing
  distinctions that should stay visible in the test body

### 7. The test would survive the wrong kind of dishonesty

Signs:

- a subtly fake implementation could still pass
- because the test proves a polished route through the system instead of the
  invariant that would expose cheating

## Review questions

Use these aggressively:

1. What real production weakness would this test catch?
2. Would this still fail if the implementation became subtly dishonest?
3. Is this proving the real boundary or a synthetic miniature of it?
4. Is the harness exposing truth or hiding it?
5. Is any helper living at the wrong abstraction layer?
6. Is any test only succeeding because the harness made life easier than
   production?
7. Is the code clean, decomposed, and readable enough to count as production
   quality?

## Required workflow

1. Read the governing docs and spec.
2. Read the tests and support code.
3. Report findings first.
4. Fix weak tests, weak harness code, or weak production code exposed by the
   tests.
5. Reassess.
6. Continue until no meaningful findings remain.

Do not stop at "tests pass now."

## Output discipline

Report findings first.

For each finding, state:

1. what is weak or dishonest
2. why it matters
3. whether the fix belongs in the test, the harness, or production code
4. what stronger proof is required

If no findings remain, say so only after genuinely hostile review.

## Canonical QA prompt

```text
Perform a brutal QA of these tests and their harness.

Evaluate them against:
- the governing implementation spec
- the milestone test requirements
- `arch_laws.md`
- `composition_laws.md`, if it is populated
- `domain_structure_laws.md`
- `perf_laws.md`
- the standard that test code and shared test abstractions must be as clean as production code

Assume the bar is hostile and practical. Prefer end-to-end and integration proof. Synthetic-only, theater, and happy-path-only tests are banned. Look for tests that pass without stressing real behavior, harness code that hides semantics, helpers that patch around production weakness, and any place where the suite is trying to get green instead of trying to break the code.

Report findings first.

For each finding, state:
1. what is weak or dishonest
2. why it matters
3. whether the fix belongs in the test, the harness, or production code
4. what stronger proof is required
```

## Canonical correction prompt

```text
Address these test QA findings completely.

Strengthen the tests. Clean the harness. Remove theater. If the tests expose a production weakness, fix production code instead of padding the test.

After each round, reassess whether:
- the tests now exercise a real boundary
- the edge cases are hostile enough
- any helper still hides semantics
- any abstraction is still sloppier than production code
- any test is still mainly happy-path proof

Continue until no meaningful findings remain.
```
