# Forge Relational DX Compatibility Transition Plan

## Purpose

DX cleanup dies when every change gets treated like a sacred compatibility
crisis.

This document exists to stop that.

Phase 5 is where we decide how Relational moves from the current public surface
to the cleaner product shape from Phases 1 through 4.

This is not another philosophy doc.

It is the migration policy.

---

## Cleanup Principles

- preserve real power where it is architecturally honest
- remove fake seams without apology
- prefer guided replacements over long ceremonial deprecation ladders
- let docs and examples move first when the code name is still acceptable
- batch real rename churn instead of thrashing the facade one symbol at a time

---

## Migration Tools

Allowed migration strategies:

- immediate removal
- containment without rename
- docs-first guided wording
- additive guided aliases
- deprecation ladders when they buy real clarity
- bundled rename waves once the target shape is stable enough

---

## Requirement

Phase 5 work should record migration posture while the cleanup is being
designed, not after the code is already halfway changed.

That means every major surface should have one of these postures:

- `Keep`
- `Keep But Reword`
- `Keep And Contain`
- `Add Guided Alias Later`
- `Deprecate Later`
- `Remove Now`

---

## Default Compatibility Posture

Until the bridge-facing facade is fully locked:

- preserve real capability
- change prominence aggressively
- use docs and examples to move the center of gravity first
- remove fake seams immediately when they were never honest product lanes
- defer rename churn when the current formal name is tolerable and the docs can
  carry the better story for now

---

## Preferred Migration Order

1. decide whether the current surface is real, fake, or merely badly named
2. remove fake surface immediately
3. if the surface is real, assign it a clean lane
4. move docs and examples to the guided story
5. add guided aliases only where the code name is now actively teaching the
   wrong thing
6. deprecate older names only after the better path is actually real

---

## Immediate Removal Rule

The following classes should not get a long deprecation ladder:

- harness and certification-only product leakage
- internal substrate authority seams
- empty helper shells
- backdoors that defeat the boundary

These were never the product promise.

Relational already applied this rule correctly to:

- `publication_authority()`
- `storage_authority()`
- `lineage_access()`
- `lineage_authority()`
- `MergeAccess::runtime()`
- non-test `facade::harness`

Those should stay removed.

---

## Containment Rule

When a surface is real but too loud for the main path:

- keep it public
- move it into a clearly specialist or contained lane in the docs
- do not force power users through fake simplification

This applies to:

- merge
- replay
- durability
- commit strategies
- validation
- compiled artifacts
- retention

---

## Docs-First Rule

When the code name is not great but not yet worth immediate churn:

- keep the formal method for now
- teach the better product name in docs and examples
- delay alias or rename work until a bigger facade cleanup batch is justified

This matters because naming churn has real cost.

We should spend that cost only when the gain is clearly worth it.

---

## Alias Rule

Add a guided alias later when all of these are true:

- the underlying lane is architecturally real
- the current name teaches the wrong product memory
- the new name is clearly better
- the alias can be added without creating a worse two-name mess

If those conditions are not all true yet, docs-first wording is enough.

---

## Deprecation Rule

Deprecation is not the default.

Use it when:

- we already have the better real path
- the old name is still common enough that a quiet docs move is not enough
- the rename is precise and codemod-able

Do not deprecate:

- fake seams that should just be removed
- formal names that are still acceptable even if the guided docs phrase is
  better
- surfaces that are likely to get regrouped again soon

---

## Surface Posture Matrix

## Primary Runtime Story

| Surface | Posture | Why |
| --- | --- | --- |
| `RelationalRuntimeApi::builder()` | `Keep` | already the right setup door |
| transaction flow (`begin_transaction`, `push_batch`, `commit`) | `Keep` | already the right primary write-truth shape |
| `plan_bulk_mutation_batch(...)` | `Keep And Contain` | good guided advanced write helper |
| `admit_*` trio | `Keep And Contain` | real power, but not primary story |
| `visibility_reads()` | `Keep But Reword` | real lane, current name is formal; docs should teach `read_truth` |
| `query` surface | `Keep` | already real and central |

Compatibility call:

- do not rename `visibility_reads()` yet
- teach `read_truth` in docs first
- revisit additive alias only if the docs-first shape still feels too leaky

## Operator Readback Story

| Surface | Posture | Why |
| --- | --- | --- |
| `inspection_access()` | `Keep But Reword` | real lane; docs should teach "inspect what happened" |
| `publication_access()` | `Keep But Reword` | real lane; docs should teach "what published" and publication diagnostics |
| publication diagnostics helpers | `Keep And Contain` | useful, but not first-memory nouns |
| retention reads under inspection | `Keep` | already coherent as operator readback |

Compatibility call:

- do not force cute renames here yet
- the current code names are acceptable
- the docs just need to teach the jobs first

## Contained Real Lanes

| Surface | Posture | Why |
| --- | --- | --- |
| `invariant_access()` | `Keep But Reword` | real lane; docs should teach `validation` |
| `certify_current_state()` | `Keep` | already good |
| `simulation_access()` / `simulation_authority()` | `Keep But Reword` | real lane; docs should teach `compiled_artifacts` |
| `retention_authority()` | `Keep But Reword` | real lane; docs should teach `retention` |
| `durability_access()` / `durability_authority()` | `Keep But Reword` | real lane; docs should lead with recovery/checkpoint jobs |

Compatibility call:

- no immediate renames
- no deprecations yet
- if later code work adds aliases, do it in one guided-lane batch:
  - `validation()`
  - `compiled_artifacts()`
  - `retention()`
  - maybe `read_truth()`

## Specialist Lanes

| Surface | Posture | Why |
| --- | --- | --- |
| `history_access()` | `Keep` | already the right historical door |
| `replay_access()` / `replay_authority()` | `Keep And Contain` | real and specialist |
| `merge_access()` + merge verbs | `Keep And Contain` | real and specialist |
| `commit_strategies()` / authority | `Keep And Contain` | real pipeline surface |
| indexes, storage, performance | `Keep And Contain` | real support lanes, not public center |

Compatibility call:

- no rename churn unless a concrete docs failure appears
- containment and docs priority are enough for now

## Removed Surface

| Surface | Posture | Why |
| --- | --- | --- |
| `publication_authority()` | `Remove Now` | fake lane |
| `storage_authority()` | `Remove Now` | fake lane |
| `lineage_access()` | `Remove Now` | empty shell |
| `lineage_authority()` | `Remove Now` | empty shell |
| `MergeAccess::runtime()` | `Remove Now` | boundary backdoor |
| non-test `facade::harness` | `Remove Now` | support leakage |

Compatibility call:

- no deprecation ladders
- no apologetic transitional story
- these stay gone unless they come back later as honest new lanes

---

## Naming Transition Calls

These are the concrete Phase 5 naming decisions.

## 1. `visibility_reads()`

Current posture:

- `Keep But Reword`

Guided product story:

- `read_truth`

Why:

- the lane is real
- the current name is formal, not fake
- docs can carry the better memory immediately
- a premature code rename would create churn before we know whether we want a
  wrapper, alias, or regrouped runtime lane

Later option:

- add `read_truth()` as an alias if Phase 6 or bridge-facing code proves that
  the formal name is still too leaky

## 2. `invariant_access()`

Current posture:

- `Keep But Reword`

Guided product story:

- `validation`

Why:

- lane is real
- current name is okay internally but weaker publicly
- docs-first wording is enough for now

Later option:

- add `validation()` as an alias in a bundled guided-lane pass

## 3. `simulation_access()` / `simulation_authority()`

Current posture:

- `Keep But Reword`

Guided product story:

- `compiled_artifacts`

Why:

- "simulation" undersells the job
- but the lane is specialist enough that docs containment buys a lot already

Later option:

- add compiled-artifact aliases if this lane becomes more central to bridge or
  publish-facing workflows

## 4. `retention_authority()`

Current posture:

- `Keep But Reword`

Guided product story:

- `retention`

Why:

- current name is precise but clunky
- docs can fix most of the usability problem immediately

Later option:

- add `retention()` as a cleaner lane alias if we do a grouped runtime-lane
  cleanup batch

## 5. `inspection_access()` / `publication_access()`

Current posture:

- `Keep But Reword`

Guided product story:

- inspect what happened
- inspect what published

Why:

- these names are already good enough technically
- the bigger problem is docs framing, not code naming

Decision:

- do not rename

---

## Docs And Examples Transition Policy

Starting now:

- quickstart and overview docs should lead with guided job names
- reference docs should continue to show the formal API names
- examples should prefer the canonical workflow order:
  - build runtime
  - write truth
  - read truth
  - inspect what happened
  - history
  - replay / merge / validation / retention / recovery when needed

Rule:

- docs should not wait for code renames before teaching the better product
  story

---

## Bridge Rule

The bridge must target the intended post-cleanup facade, not the raw leftover
shape that happened to exist earlier in the project.

That means:

- bridge docs should speak in the guided lane names
- bridge-facing design should not depend on removed seams
- bridge-facing design should not force premature rename churn just to feel
  cleaner in the moment
- if bridge work needs a clearer alias, add it deliberately as part of the
  compatibility plan, not as a random one-off

---

## Release Gate For Transitional Surface

No transitional wording state should live forever.

Before publication, every meaningful surface should land in one of these
states:

- kept as a legitimate public formal name with guided docs wording
- kept as a legitimate public name plus guided alias
- intentionally contained specialist API
- fully removed

What we should avoid is the worst middle state:

- formal names that docs apologize for
- aliases that docs barely mention
- fake seams left around because we were scared to commit

---

## Phase 5 Bottom Line

The migration posture is now clear:

- fake seams: remove immediately
- real lanes with acceptable formal names: keep and teach better
- real lanes with clunky formal names: docs-first now, aliases later only if
  clearly worth it
- specialist power: contain, do not flatten

That is enough compatibility structure to keep cleanup moving without letting
the bridge freeze accidental surface area into the future.
