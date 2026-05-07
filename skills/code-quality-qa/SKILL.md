---
name: code-quality-qa
description: Run a Forge-quality structural review of production code. Use when the question is whether code is well composed and well organized under `composition_laws.md` and `domain_structure_laws.md`, especially for file structure, function structure, naming, helper placement, module boundaries, and directory topology.
---

# Code Quality QA

Use this skill when the main question is whether production code is
structurally well-composed and well-organized.

This skill is only about:
- `composition_laws.md`
- `domain_structure_laws.md`

It is not about:
- feature completeness
- milestone semantic compliance
- test adversarial strength
- performance proofs except where bad structure hides cost

## Mandatory reading order

Read these in this order before running the pass:

1. `_docs/coding_guidelines/composition_laws.md` if it is populated
2. `_docs/coding_guidelines/domain_structure_laws.md`

Then read:
3. the target production files
4. any directly adjacent files or folders needed to judge structure honestly

## Standard

Review as a hostile engineer who assumes code can be technically correct while
still being structurally bad.

The bar is:
- files own one real responsibility
- functions read like named semantic steps
- names predict meaning
- helpers live at the right layer
- folders teach the subsystem
- the next correct edit is obvious

## Scope of this skill

Focus only on:
- file composition
- function composition
- naming quality
- helper placement
- module boundaries
- folder boundaries
- directory structure
- file-size and directory-size discipline
- whether the code teaches the domain or hides it

Do not use this skill to judge:
- whether the feature should exist
- whether the implementation matches the full milestone spec
- whether the tests are adversarial enough

## Core review questions

1. Does this file have one real responsibility?
2. Does the filename predict its contents?
3. Do the main functions read like semantic steps instead of raw mechanics?
4. Is domain classification named, or buried inline?
5. Are helpers local, child-module, sibling-module, or shared at the correct layer?
6. Is the directory structure organized by real responsibility instead of buckets?
7. Are any files acting like god files or any functions acting like god functions?
8. Are there vague bucket names like `helpers`, `common`, `utils`, or `logic` hiding structure?
9. Is the next correct edit obvious from the structure?

## Typical findings

Look aggressively for:
- god functions
- god files
- mixed abstraction levels
- inline unnamed policy or classification logic
- vague names
- fake helpers that only moved code without naming the responsibility
- bucket folders
- flat directories that need substructure
- files over 400 lines without exemption
- directories over 10 files without honest subdivision

## Required workflow

1. Read the two governing docs.
2. Read the target files and nearby structure.
3. Perform a hostile review.
4. Report findings first.
5. Fix the findings.
6. Reassess whether better decomposition is still needed.
7. Repeat until no meaningful findings remain.

## Required output discipline

For each finding, state:
1. what is wrong
2. why it matters
3. what authority it violates
4. what correction is required

## Canonical QA prompt

Preserve this wording exactly when you use it internally as your review frame.

```text
Perform a brutal code-quality QA pass focused only on composition and structure.

Evaluate the code against:
- `composition_laws.md`, if it is populated
- `domain_structure_laws.md`

Assume the bar is production-grade. Look for god functions, god files, vague naming, bucket modules, bad helper placement, mixed abstraction levels, fake decomposition, flat directory sprawl, and any structure that makes the next correct edit harder than the next convenient edit.

Report findings first.

For each finding, state:
1. what is wrong
2. why it matters
3. what authority it violates
4. what correction is required

If there are no findings, say so explicitly only after genuinely hostile review.
```
