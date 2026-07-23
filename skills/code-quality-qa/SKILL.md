---
name: code-quality-qa
description: Run a hostile WORTH-quality structural review of production code. Use when the question is whether code truly lives up to `composition_laws.md` and `domain_structure_laws.md`, especially directory topology, file names, function decomposition, parameter shape, naming, helper placement, and module boundaries.
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

1. `_docs/coding_guidelines/composition_laws.md`
2. `_docs/coding_guidelines/domain_structure_laws.md`

Then read:
3. the target production files
4. any directly adjacent files or folders needed to judge structure honestly

## Standard

Review as a hostile engineer who assumes code can be technically correct while
still being structurally bad.

The bar is stricter than ordinary "clean code", stricter than Google-style
readability, and biased toward WORTH's opinionated architecture standards.

Assume the structure is guilty until it proves itself:
- a directory must truly earn its boundary, not merely group related files
- a filename must let a new engineer predict its contents before opening it
- a function must truly be decomposed into named semantic steps, not just be
  short enough to tolerate
- a helper must reveal the parent responsibility it serves, not just reduce
  line count
- a module boundary must make the next correct edit easier than the convenient
  edit
- a name must carry phase, authority, truth status, and domain meaning when
  those distinctions matter

Do not grade on an industry curve. A structure that would pass ordinary code
review can still fail this skill if it is less explicit, less navigable, or
less predictive than WORTH's laws demand.

Assume the surrounding codebase may not yet meet this bar. Existing local
patterns are evidence to inspect, not precedent to copy. When reviewing new or
touched work, prefer setting the better structural precedent over matching
nearby weaker files, unless compatibility with the existing shape is a real
architectural constraint.

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

Ask these with real skepticism, not as checklist affirmations:

1. Does this directory truly earn its boundary, or is it just a folder for
   related things?
2. Does each filename truly predict its contents without opening the file?
3. If a filename is broad, is it an honest facade/aggregation point, or a
   responsibility sink?
4. Does each file truly own one semantic responsibility that can be deleted,
   replaced, tested, and reviewed as one idea?
5. Do the main functions truly read like named semantic steps, or do they hide
   classification, policy, transformation, projection, or formatting inline?
6. Are the functions decomposed because the domain has named substeps, or only
   because the body got long?
7. Does every branch that represents a domain classification have a name?
8. Does every helper live at the narrowest honest semantic radius?
9. Are any helpers fake decomposition that moved code without clarifying
   responsibility?
10. Are names carrying the phase, authority, proof status, and domain meaning
    that callers need at the point of use?
11. Are there vague bucket names like `helpers`, `common`, `utils`, `logic`,
    `types`, `model`, `service`, `manager`, or `processor` hiding structure?
12. Would a new engineer know where to make the next correct edit without grep
    archaeology?
13. Is the current structure merely acceptable, or does it teach the subsystem
    under pressure?
14. Is the code copying a weak surrounding pattern when it should instead set a
    stronger precedent for future work?

## Typical findings

Look aggressively for:
- god functions
- god files
- mixed abstraction levels
- inline unnamed policy or classification logic
- vague names
- fake helpers that only moved code without naming the responsibility
- bucket folders
- filenames that are technically accurate but not predictive enough
- directories that group related things without encoding structural fate
- files that would become deletion-resistant if one behavior were removed
- functions that are small but still hide domain decisions
- broad facade-looking modules that implement rather than aggregate
- new code inheriting weak local structure as though existing code were the
  standard
- flat directories that need substructure
- files over 400 lines without exemption
- directories over 10 files without honest subdivision

## Mandatory function scrutiny

From the worktree root, inventory every dirty Rust file before judging function
composition:

```text
python scripts/quality/scrutinize_rust_functions.py --dirty .
```

If `python` is unavailable, resolve the workspace Python runtime and invoke the
same script with that executable. Use the other composable modes when the
review scope is not the dirty worktree:

```text
python scripts/quality/scrutinize_rust_functions.py path/to/folder
python scripts/quality/scrutinize_rust_functions.py --workspace path/to/workspace
python scripts/quality/scrutinize_rust_functions.py --dirty path/to/worktree --format json
```

The dirty mode includes staged, unstaged, and untracked Rust files. It reports:

- every function spanning more than 60 lines
- every function with at least 5 explicit parameters; method receivers do not
  count as parameters

Treat every reported function as a cleanup candidate requiring deliberate
review. Call attention to every candidate in the QA inventory, including its
path, function name, measured size, parameter count, and the reason it was
selected. A candidate is not automatically a finding and these thresholds are
not bans. For each candidate, determine whether named semantic decomposition,
a typed parameter object, a narrower phase input, or a better ownership
boundary would improve the code. Record an explicit no-change disposition when
the current shape is genuinely the clearest honest expression.

Do not add suppressions or allowlists merely to silence the inventory. Use
`--fail-on-candidates` only when a caller explicitly requests gating; the
standard QA workflow remains advisory and judgment-based.

## Required workflow

1. Read the two governing docs.
2. Run the mandatory dirty-worktree function scrutiny inventory.
3. Read every reported candidate, the target files, and nearby structure.
4. Perform a hostile review.
5. Report findings first and distinguish scrutiny candidates from actual
   findings.
6. Fix the findings.
7. Reassess whether the new directory names, file names, and function names
   truly predict their contents.
8. Rerun the inventory and repeat until no meaningful findings remain; retain
   explicit dispositions for candidates that honestly require no change.

Do not run tests before structural edits are complete unless a compile error is
needed to unblock the refactor. Prefer static review, targeted reads, and name
audits while editing; verify once the structure has stabilized.

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
- `composition_laws.md`
- `domain_structure_laws.md`

Assume the bar is stricter than ordinary production code, stricter than Google readability, and biased toward WORTH's most opinionated standards.

Start from skepticism. Ask whether each directory truly earns its boundary, whether each filename truly predicts its contents before opening it, and whether each function is truly decomposed into named semantic steps rather than merely being short, tidy, or locally understandable.

Assume the surrounding codebase may fail this skill. Treat nearby patterns as context, not permission. New or touched code should set the better precedent unless an explicit architectural constraint requires alignment with existing weaker structure.

Look for god functions, god files, vague naming, bucket modules, filenames that are accurate but not predictive, bad helper placement, mixed abstraction levels, fake decomposition, flat directory sprawl, deletion-resistant files, helper swamps, new code copying weak local precedent, and any structure that makes the next correct edit harder than the next convenient edit.

Report findings first.

For each finding, state:
1. what is wrong
2. why it matters
3. what authority it violates
4. what correction is required

If there are no findings, say so explicitly only after genuinely hostile review.
```
