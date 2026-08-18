---
name: code-quality-qa
description: Review and correct WORTH production-code composition and domain topology. Use when auditing directory structure, future insertion, file responsibilities, function decomposition, naming, facade honesty, helper placement, module boundaries, or applying the 400-line file limit and 60-line and five-argument function advisories to every dirty workspace file against `composition_laws.md` and `domain_structure_laws.md`.
---

# Code Quality QA

Attempt to falsify the claim that the code's physical structure preserves its
meaning. Judge against WORTH's laws, not local precedent or generic clean-code
taste.

## Establish the structural model

Read the repository instructions, `composition_laws.md`,
`domain_structure_laws.md`, the governing specification or roadmap, the target
files, and enough adjacent topology to understand ownership and dependency
direction.

Before narrowing the semantic review, enumerate the complete Git worktree
change set: staged, unstaged, renamed, and untracked files. The numeric
discipline below applies to every dirty code, test, fixture, and support file,
not only the requested feature, current diff hunk, or files changed by the
reviewing agent.

In WORTH Rust worktrees, run
`python scripts/quality/scrutinize_rust_functions.py --dirty .` and inspect
every reported candidate. If that repository tool is unavailable, perform an
equivalent dirty-file inventory rather than narrowing the rule.

Identify:

- current and committed future responsibilities
- semantic, authority, truth-source, lifecycle, and volatility axes
- the public facade and dependency direction
- which responsibilities should be independently understood, replaced, tested,
  evolved, or deleted

## Exhaust primary structural discovery

Do not launch an independent reviewer during initial structural discovery. The
primary agent must inspect the complete dirty set, every numeric advisory,
relevant parent and child modules, sibling owners, facades, dependency edges,
and the committed destination topology. Search each finding's semantic family
for parallel bags, mixed-level coordinators, dishonest facades, misplaced
support, duplicated owners, and unstable naming. Correct supported findings,
rerun the dirty inventory, and repeat until another primary pass finds no new
in-scope structural defect.

Record a search-coverage manifest containing the dirty files, adjacent topology,
queries, symbols, ownership boundaries, numeric candidates, and enforcement
commands inspected. Freeze the stable candidate revision or deterministic
source fingerprint before outside review.

## Independent review after self-exhaustion

Only after the primary structural search is exhausted, use a fresh, read-only
independent reviewer to attack the frozen candidate. Do not use the reviewer
for initial inventory, broad repository search, file discovery, or first-pass
decomposition. Give it a compact packet containing repository laws, governing
specification, frozen revision, scoped diff, destination topology, search-
coverage manifest, enforcement results, and the raw files needed to look for
missed composition defects. Keep conclusions and preferred fixes out of its
first prompt.

Verify every reviewer finding directly. If a material correction follows,
repeat the affected primary searches to self-exhaustion and use a new reviewer
for the final-source closure pass rather than retaining the old reviewer as a
search assistant.

## Review

Apply these lenses:

- **Destination topology:** Committed successors can enter without
  reclassification, authority reversal, bucket growth, or facade relocation.
- **Semantic responsibility:** Each directory, file, type, and function owns one
  predictable idea at its visibility radius.
- **Composition:** Orchestrators read as named semantic steps; decisions,
  effects, failure paths, and control transfers are not hidden in mixed-level
  bodies.
- **Placement:** Logic and support code live at the narrowest honest semantic
  radius, and stable meaning owns volatile mechanisms.
- **Naming:** Names predict contents, exclusions, phase, authority, and truth
  status without depending on implementation archaeology.
- **Boundary honesty:** Facades aggregate rather than implement, external
  mechanisms remain behind domain-owned ports, and dependency direction
  preserves authority.
- **Deletion and extension:** Removing or adding a responsibility has an obvious
  location and does not require unrelated edits.
- **Size discipline:** Every dirty code, test, fixture, and support file must
  remain at or below 400 lines unless the repository explicitly exempts it. A
  non-exempt dirty over-limit file prevents clean QA closure. Inspect every
  function in that dirty set; more than 60 lines or five or more explicit
  arguments triggers inspection for collapsed responsibility or a dishonest
  signature, but remains advisory rather than a defect by itself. If unrelated
  user work cannot safely be restructured, report the unresolved hard-limit
  violation rather than silently excluding the file.

The hard file limit does not make smaller code coherent. Judge responsibility
and topology independently of whether a file or function stays below a numeric
threshold.

Keep this pass structural. Follow semantic, performance, or test defects only
when structure conceals or causes them.

## Findings

Report findings before summaries. For each finding state:

1. affected structural responsibility
2. concrete defect and evidence
3. violated composition or domain-structure law
4. required destination structure or decomposition
5. evidence that would close the finding

Do not report subjective taste or demand movement that merely exchanges one
equally honest structure for another.

## Correct and repeat

Correct responsibility and topology rather than shuffling lines. Search the
affected semantic family for the same structural defect, preserve public
contracts where required, and verify compilation and repository enforcement
after the structure stabilizes.

Reassess naming, placement, dependency direction, deletion boundaries, and
committed future insertion after each material correction. Continue until no
meaningful structural findings remain. Closure requires both the exhausted
primary pass and a final independent pass over the same frozen source state.
