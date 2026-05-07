---
name: qa-tests
description: Run a Forge-quality hostile review of tests and test harnesses. Use when reviewing whether tests are genuinely adversarial, architecturally honest, code-clean, properly abstracted, and supported by the right domain-agnostic or domain-specific harness layers.
---

# QA Tests

Use this skill when implementation exists but the tests themselves need hostile review.

This skill is not about whether tests pass.
It is about whether the tests and harness are worthy of the codebase.

The standard is:

- tests are production-quality code
- tests certify real system properties
- harness architecture is real architecture
- proof logic is abstracted at the correct layer
- fixtures, adapters, and assertion helpers are decomposed by true responsibility
- file and directory discipline are enforced as hard constraints
- no one gets to excuse test mess as "just test code"

## Mandatory reading order

Read these in this order before running the test QA pass:

1. `_docs/coding_guidelines/MENTALITY.md`
2. `_docs/coding_guidelines/arch_laws.md`
3. `_docs/coding_guidelines/composition_laws.md` if it is populated
4. `_docs/coding_guidelines/domain_structure_laws.md`
5. `_docs/coding_guidelines/perf_laws.md`

Then reread: 5. the governing milestone/spec 6. the milestone test requirements, if one exists 7. the relevant test files 8. the relevant test support / harness files

## Standard

Review as a hostile engineer who assumes tests may pass while still being weak,
noisy, duplicated, brittle, under-abstracted, over-abstracted,
architecturally dishonest, or simply badly written.

The bar is not:

- "the tests pass"
- "coverage exists"
- "the assertions are thorough"
- "the harness is convenient"

The bar is:

- the tests prove the correct thing
- the tests are adversarial enough
- the test code itself is clean, decomposed, and maintainable
- the harness makes meaning clearer rather than more magical
- abstractions live in the correct layer
- file and directory discipline are respected with zero sloppiness

## Non-negotiable code quality rules

These rules are mandatory for test code and harness code.

### 1. Test code is production code

Test files, fixtures, support modules, harness adapters, and assertion helpers
must meet the same code quality bar as production runtime code.

Bad test code is not acceptable because it "only" verifies behavior.
If the test suite is messy, the proof surface is messy.

### 2. Harness architecture is real architecture

A test harness is not a bag of helpers.
It is a subsystem that models:

- setup
- proof
- adapters
- fixtures
- domain pressure
- runtime substrate

If the harness is flat, blurry, or convenience-driven, it will rot the proof
surface exactly the way a bad runtime facade rots production semantics.

### 3. Files over 400 lines are banned

Code files and test files over **400 lines** are forbidden unless an explicit
written exemption exists in the governing spec, roadmap, workspace rule, or
allowlist.

This applies to:

- test files
- support files
- fixture files
- harness helpers
- adapters
- proof helpers

If a touched file exceeds the cap, splitting it is part of the work.
Do not defer it as future cleanup.

### 4. Directories over 10 files are banned by default

Any test or support directory with more than **10 files** is forbidden unless a
written exemption exists.

This rule exists because flat file piles destroy domain clarity and encourage
category-bucket architecture.

If a directory is nearing or above 10 files:

- split it by responsibility
- create domain folders or proof-surface folders
- make the structure teach the shape of the domain

Do not tolerate "we’ll reorganize later."

### 5. Category is not responsibility

A file or folder is not well designed just because its label sounds tidy.

These are suspect by default unless narrowly scoped:

- `helpers`
- `common`
- `assertions`
- `builders`
- `utils`
- `support`

A category label is not a responsibility.
A responsibility is something that changes for one reason, fails independently,
and teaches one piece of the system.

### 6. Tests must read like proof, not ceremony

The goal is not terse cleverness.
The goal is:

- explicit meaning
- low noise
- strong signal
- easy diagnosis on failure
- clean decomposition

A giant test with endless inline tuple-building, repeated fixture assembly, and
ad hoc parity checks is not "thorough." It is under-designed.

## Scope of this skill

This skill is for test and harness QA.

It should focus on:

- adversarial strength of tests
- architectural honesty of assertions
- code quality of tests
- code quality of test support
- abstraction placement
- harness integrity
- domain-specific vs generic support boundaries
- brittleness, sprawl, and noise
- hidden dependence on fixture quirks
- exactness of proof surfaces
- file and directory decomposition

If the tests expose a substrate lie, fixing that lie is in scope.
But the main target is the quality of the tests and harness themselves.

## Harness architecture rules

### 1. Organize harness code by responsibility, not convenience

Fixture construction, assertion helpers, bridge adapters, proof helpers,
domain builders, and support profiles are different responsibilities.

If they change for different reasons, they belong in different files and
probably different folders.

A support tree that mixes:

- fixture construction
- bridge/runtime adapters
- assertion helpers
- domain setup
- proof snapshots

inside one bucket is a domain-model failure.

### 2. Separate proof helpers from fixture assembly

These are usually different subdomains.

Examples of healthy separation:

- `support/proof/...`
- `support/runtime/...`
- `support/domains/geometry/...`
- `support/fixtures/...`

Do not mix:

- how a system is constructed for the test
  with
- how its contracts are asserted

### 3. Domain pressure belongs in domain support

When repeated setup or naming is genuinely about geometry, topology, policy, or
some other real domain, give it a domain home.

Do not pretend domain setup is generic if it is not.

### 4. Generic proof mechanics belong in generic support

When multiple tests certify the same runtime contract, and the logic is
genuinely domain-agnostic, lift it into generic support.

Examples:

- resolution-map snapshot helpers
- receipt/inspection parity helpers
- exact digest comparison helpers
- lifecycle snapshot assertion helpers

Do not leave obviously generic proof mechanics buried in a single domain test.

### 5. Harness honesty is mandatory

A harness must not:

- silently inject support the runtime did not declare
- collapse unsupported and domain-invalid paths
- normalize away distinctions the public contract preserves
- hide ordering, identity, lineage, or verification assumptions that the real
  runtime requires
- make a weak proof look stronger than it is

A convenient dishonest harness is worse than repetitive honest setup.

### 6. Assume the harness will grow

Assume the test family you are touching will continue to grow.

Organize the harness for the future size of the proof surface, not for the
current number of tests.
If a structure will obviously become crowded, split it now.

## Harness abstraction rules

These are mandatory architecture rules for test support and harness design.

### 1. Generic proof mechanics must be lifted when repetition proves a shared contract

If multiple tests certify the same runtime contract and repeat the same proof
mechanics, that logic belongs in generic test support.

Examples:

- resolution-map snapshot helpers
- receipt/inspection parity helpers
- lifecycle snapshot assertion helpers
- digest comparison helpers
- lineage / assumption / evidence parity helpers

Leaving clearly generic proof mechanics duplicated across domain tests is a
harness design failure.

### 2. Domain pressure must not be hidden inside fake generic helpers

If a helper encodes geometry, topology, policy, subscription, or other
domain-specific vocabulary or setup, it belongs in domain support rather than
generic support.

Do not label a helper generic when it is really expressing one domain's
pressure shape.

### 3. Setup helpers and proof helpers are different responsibilities

Helpers that construct test state and helpers that assert runtime contracts are
usually separate responsibilities and must not be merged casually.

Examples of healthy separation:

- domain setup helpers
- runtime substrate helpers
- proof snapshot helpers
- parity assertion helpers

A file that mixes all of these because they are "all test support" is a
category bucket, not a responsible design.

### 4. Do not abstract away semantic edges

A helper is invalid if it hides distinctions the runtime contract preserves.

Do not let helpers blur:

- lifecycle differences
- identity vs replacement semantics
- runtime denial vs domain denial
- verification assumptions
- ordering requirements
- presence vs absence meaning

Convenience is not allowed to erase auditability.

### 5. Shared helpers must clarify the proof, not make it magical

Good helpers reduce noise while keeping the proof legible.

Bad helpers:

- hide too much of the assertion surface
- merge distinct semantics into one API
- make failures harder to diagnose
- encode fixture quirks as if they were contract truth

If the helper makes the test feel more "elegant" but less explicit, it is
probably the wrong abstraction.

### 6. Repetition alone does not justify abstraction

Before abstracting, ask:

- is this actually the same proof surface?
- is this shared runtime contract or only shared syntax?
- would lifting this helper merge meanings that should stay distinct?

The wrong abstraction is worse than some honest repetition.

### 7. Harness abstractions must live at the narrowest honest layer

When introducing a helper, decide explicitly whether it belongs in:

- the test file
- generic test support
- domain-specific test support
- runtime-substrate support
- fixture support

Do not leave this implicit.
Abstraction placement is part of the proof architecture.

## Core review questions

Use these questions aggressively.

### 1. Is the test actually adversarial?

Does it pressure:

- boundary cases
- near-miss semantics
- denial paths
- absence-is-meaningful behavior
- replay / parity / round-trip invariants
- exact proof-bearing artifacts

If it only proves the obvious path, it is weak.

### 2. Is the test proving behavior or architecture?

Look for whether the test certifies:

- exact lifecycle classification
- exact resolution-map meaning
- exact digest / counter / summary output
- receipt / inspection parity
- fail-closed boundaries
- distinction between nearby semantic classes

If the test only proves "operation succeeded," it is weak by default.

### 3. Are the assertions honest?

Tests must not freeze:

- fixture accidents
- bridge quirks unless the contract is explicitly bridge-specific
- incidental ordering unless ordering is part of the contract
- implementation residue that is not actually public meaning

If an assertion passes today for the wrong reason, that is a test bug.

### 4. Is the test code itself clean?

Review the code, not just the proof:

- Is the file doing too much?
- Is setup mixed with proof logic?
- Are helper boundaries obvious?
- Are there repeated inline tuple encodings?
- Are there giant assertion blocks that should be named helpers?
- Are there long tests that should be decomposed into proof helpers?
- Is the file over 400 lines?
- Is the directory over 10 files without a structure split or exemption?

A test can prove the right thing and still be badly written.
That is still a finding.

### 5. Is the harness helping or hiding?

A good harness:

- reduces noise
- centralizes repeated proof mechanics
- preserves semantic visibility
- clarifies what is being certified

A bad harness:

- hides meaning behind convenience
- merges distinct semantics into one helper
- bakes in fragile fixture assumptions
- makes tests look cleaner while weakening what they prove

### 6. Are abstractions at the correct layer?

Lift to **domain-agnostic test support** only when:

- multiple tests prove the same runtime contract
- the helper is about proof mechanics, not domain vocabulary
- the helper does not collapse distinct semantics

Lift to **domain-specific test support** when:

- the shape is genuine domain pressure
- the helper captures repeated domain fixture construction
- the semantics are not honestly generic

Do not create fake generic helpers that are secretly geometry, topology,
policy, or subscription-specific.

### 7. Does the directory structure teach the domain?

A directory with many files must be organized by:

- proof surface
- domain pressure family
- harness concern
- runtime substrate
- fixture authority type
- adapter kind

Bad split axes:

- generic convenience labels
- undifferentiated support buckets
- file type alone
- "stuff we didn’t know where else to put"

If the structure does not teach what kinds of tests exist and why, it is
failing as architecture.

### 8. Is the harness acting like a DSL where proof should stay visible?

A test DSL or fluent helper is acceptable only if it preserves semantic
visibility.

Do not let a setup DSL:

- hide lifecycle distinctions
- hide verification assumptions
- hide denial classes
- hide ordering or identity contracts

Convenience must never outrank auditability.

### 9. Does the test fail loudly and usefully?

When it breaks, can we tell:

- what contract regressed?
- whether the problem is test drift, harness drift, or substrate drift?
- whether the failure is about lifecycle, identity, lineage, resolution, or
  parity?

Broad tests are fine.
Broad tests with muddy failure shape are not.

## Test quality anti-patterns

Treat these as findings unless clearly justified:

- happy-path-only proof
- giant test files
- giant support files
- flat directories with too many files
- inline repetition of the same proof mechanics across multiple tests
- domain-specific fixture construction repeated without a domain helper
- generic proof mechanics trapped inside one domain test
- helpers that mix multiple semantic concerns
- assertion helpers and setup builders living in the same module without a
  narrow reason
- assertions on incidental fixture values
- asserting presence when exact value is required
- asserting count when exact map or digest is the real contract
- brittle index-based assertions where ordering is not contractually required
- vague string-contains checks where exact structured assertions are possible
- giant tuple blocks repeated in several files
- domain-agnostic helper candidates left duplicated
- fake generic helpers that smuggle domain vocabulary
- overgrown mutation test directories with no sub-organization
- tests that prove success but not fail-closed absence
- tests that verify receipt but not inspection, or vice versa
- tests whose setup, execution, and proof are all tangled together

## Required workflow

1. Read the mandatory docs in order.
2. Reread the governing spec and test requirements.
3. Perform a hostile review of the tests and harness.
4. Report findings first.
5. Fix the findings.
6. Reassess whether the fixes exposed a better abstraction boundary.
7. Reassess whether the file or directory topology is still honest.
8. Continue until no meaningful findings remain.

Do not stop after strengthening assertions if the file is still overgrown.
Do not stop after splitting a file if the proof logic is still duplicated.
Do not stop after creating a helper if it lives at the wrong abstraction layer.

## Required output discipline

When reporting findings:

- findings first
- concrete, not vague
- no celebration
- no "overall looks good" opener

For each finding, state:

1. what is wrong
2. why it matters
3. what authority it violates
4. what correction is required

If the finding is about abstraction placement, say explicitly whether the
correction belongs in:

- the test file
- generic test support
- domain-specific test support
- directory structure
- production substrate

## Canonical QA prompt

Preserve this wording exactly when you use it internally as your review frame.

```text
Perform a brutal QA of these tests and their harness.

Evaluate them against:
- the governing implementation spec
- the milestone test requirements
- `arch_laws.md`
- `composition_laws.md`, if it is populated
- `domain_structure_laws.md`
- `perf_laws.md`
- the standard that test code and harness code must be as clean as production code

Assume the bar is aerospace-grade. Look for shallow assertions, non-adversarial tests, brittle fixture coupling, abstraction drift, hidden harness dishonesty, repeated proof logic that belongs in support, fake generic helpers that are secretly domain-specific, oversized files, overgrown directories, and any test that technically passes while proving the wrong thing or being written badly.

Report findings first.

For each finding, state:
1. what is wrong
2. why it matters
3. what authority it violates
4. what correction is required

If there are no findings, say so explicitly only after genuinely hostile review.
```

## Canonical correction-loop prompt

Preserve this wording exactly when you use it internally as your correction
frame.

```text
Address these test QA findings completely.

Do not negotiate with them, minimize them, or patch around them. Strengthen the tests, clean the code, fix the harness, and move abstractions to the correct layer. If the tests expose a substrate lie, correct the substrate rather than preserving a misleading assertion.

After corrections, reassess whether:
- any test still proves the wrong thing
- any helper lives at the wrong abstraction layer
- any repeated proof logic should be lifted
- any domain-specific shape is pretending to be generic
- any generic helper is still trapped inside a domain test
- any file is still too large
- any directory is still too flat or too crowded

Then continue until no meaningful findings remain.
```

When introducing helpers:

### Lift into generic support only if all are true:

- the helper proves a contract shared by multiple tests
- the contract is runtime-generic rather than domain-vocabulary-specific
- the helper does not merge distinct semantic meanings
- the helper makes tests clearer, not more magical

### Lift into domain support when:

- the repeated shape is genuine domain pressure
- the helper captures repeated domain setup or naming
- the helper would be dishonest if presented as generic

### Do not abstract if:

- the repetition is small and the helper would hide meaning
- the tests are similar in syntax but different in semantic contract
- the abstraction would merge proof surfaces that should stay distinct

## File and directory enforcement

These are hard gates, not nice-to-haves.

### File cap

- Default maximum: **400 lines**
- Applies to:
  - test files
  - support files
  - fixture files
  - harness helpers
  - adapters
  - proof helpers

If a touched file is over the cap and has no written exemption, splitting it is
mandatory.

### Directory cap

- Default maximum: **10 files per directory**
- Applies to:
  - test directories
  - support directories
  - harness directories
  - fixture directories

If a directory exceeds the cap and has no written exemption, reorganizing it is
mandatory.

Written exemptions must exist in:

- the governing spec
- roadmap
- workspace rule
- or other explicit written policy

No silent exceptions.

## Completion rule

The test QA loop is complete only when:

- no meaningful findings remain
- the tests are adversarial enough for the milestone bar
- the test code is clean enough for the codebase bar
- the assertions prove the intended contract
- harness abstractions live at the correct layer
- domain-specific and domain-agnostic support are separated honestly
- file-size and directory-size rules are satisfied or explicitly exempted
- no obvious in-scope cleanup remains
- the tests read like proof instead of like a pile of accumulated ceremony

Do not declare victory early.
