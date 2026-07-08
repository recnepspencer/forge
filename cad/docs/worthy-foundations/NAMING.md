# Naming

**Status:** Frozen grammar. The reserved-names list grows by reviewed PR; the
grammar itself does not change without amending `ARCHITECTURE.md`.
**Enforced by:** `tools/boundary-check` (Fence 3) and `tools/agent-context`.

Canonical machine constitution: `tools/boundary-check/config/road1.toml`

This document is the human-read constitutional mirror of the machine-owned Road
1 naming contract. `tools/boundary-check` and `tools/agent-context` consume
`road1.toml`; this file must stay semantically aligned with that contract. If
the table here drifts from the machine constitution, the tree is being taught
wrong - treat edits to this file with public-API care.

---

## The Grammar

Every crate name parses as:

```text
{tier}-{band}-{domain}
```

A crate name that does not parse is a CI failure. A crate name that parses
but uses an unreserved domain is a CI failure unless the same PR extends the
reserved list (a visible, reviewable act - see "Extending the reserved list").

There is no fourth segment. If a crate feels like it needs one
(`worthy-solver-curve-nurbs`), that is the signal to either stay in the
parent crate or propose a new domain noun - not to invent sub-grammar.

---

## Tiers

| Tier | Meaning | Test |
|---|---|---|
| `forge` | runtime substrate (not in this repo's grammar; listed for orientation) | owned by the Forge workspaces |
| `worth` | engineering platform | *would aerospace need this crate unchanged?* -> yes |
| `worthy` | CAD/BIM product tier | *would aerospace need this crate unchanged?* -> no |

Rules:

- `worth-*` crates may never depend on `worthy-*` crates. Platform never
  depends on product.
- When in doubt, default to `worthy-`. Promotion from product tier to
  platform tier is a deliberate, reviewed move; demotion is a rename we never
  want to make.

> **Hazard note (accepted):** `worth`/`worthy` differ by one letter. Reviews
> and prompts should spell out "platform tier" / "CAD tier" when ambiguity
> matters. Revisit per `ARCHITECTURE.md` Open Decision 1.

---

## Bands (fixed set - closed)

| Band | Authority | May import (Fence 1) | Never |
|---|---|---|---|
| `schema` | shared truth grammar, contract nouns | nothing in the tree | executing anything |
| `dsl` | language: syntax, AST, binding, lowering | `schema-*` | runtime execution |
| `entry` | Query-native runtime entry & orchestration | `schema-*`, `resolver-*`, `derived-*`, `forge-query` | replay surfaces (Fence 2) |
| `resolver` | domain semantic decisions | `schema-*`, `solver-*` | Query entry, publication folklore |
| `solver` | pure computation kernels | `schema-*` | `forge-query`, any entry surface |
| `derived` | published derived artifacts | `schema-*`, `solver-*` (math only) | minting source authority |
| `pack` | distributable knowledge bundles | public seams only | runtime internals |
| `app` / `ui` | human- and AI-facing surfaces | `entry-*`, `derived-*`, `dsl-*` | reaching around facades |
| `cert` | hostile, adoption, performance, scale proof | anything | being depended on |

Adding a band requires amending `ARCHITECTURE.md`. Expected never.

---

## Domain nouns - reserved list

A reserved name is a guarantee about what a crate *will* be called, not a
crate. **A crate is born only when real code needs it.**

Legend: ○ reserved name only · ◐ reserved, presumed pack content
(Open Decision 4)

### `worth` tier (platform)

| Domain | schema | entry | derived | pack | cert | Notes |
|---|---|---|---|---|---|---|
| `core` | ○ | | | | | identity, naming, units, tolerance, measure vocabulary. **Size-fenced** - the sole sanctioned "core" |
| `graph` | ○ | | | | | the graph constitution: layers, edge classes, spine grammar, aspect rules, promotion grammar. Reserved until the first graph crate is actually born in code. |
| `registry` | | | | ○ | | the pack seam itself |
| `adoption` | | ○ | | | ○ | entry home for declaration handles, lowering, and obligation/contribution adoption when those surfaces are born later; cert home for Query adoption proof harness (Consumer Kit backed). Both are reserved until later milestones actually birth code. |
| `publication` | | | ○ | | | retained/publication grammar and projection-consumption-facing posture for the platform tier. Reserved until Milestone 4 proves the real `worth-derived-*` surface. |

This table records naming legality and reservation posture only. It does not
act as current-tree inventory authority or milestone progress authority.

Road 1 public routing uses these reserved follow-on names without birthing them:

- `worth-entry-adoption`
  - deferred declaration/adoption facade for Milestone 3
- `worth-derived-publication`
  - deferred retained/publication facade for Milestone 4
- `worthy-derived-brep`
  - deferred consumer-facing retained artifact path for Milestone 4

### `worthy` tier (CAD/BIM)

**schema** - `worthy-schema-*`
○ `topology` · ○ `geometry` · ○ `material` · ○ `structure` · ○ `service` ·
○ `policy` · ○ `assumption` · ○ `physics` · ○ `jurisdiction` · ○ `economics` ·
○ `approval` · ○ `component` · ○ `measure`

**dsl** - `worthy-dsl-*`
○ `syntax` · ○ `ast` · ○ `binding` · ○ `lowering` · ○ `format` · ○ `lint` ·
○ `analysis` · ○ `lsp`

**entry** - `worthy-entry-*` (operation families, matching how requests enter)
○ `construct` · ○ `boolean` · ○ `transform` · ○ `component` · ○ `route` ·
○ `structure` · ○ `analysis` · ○ `assumption` · ○ `measure` · ○ `policy` ·
○ `jurisdiction` · ○ `cost` · ○ `approval` · ○ `recovery`

**resolver** - `worthy-resolver-*`
○ `component` · ○ `placement` · ○ `assembly` · ○ `structure` · ◐ `envelope` ·
◐ `interiors` · ◐ `mep` · ○ `policy` · ○ `assumption` · ○ `tolerance` ·
○ `jurisdiction` · ○ `physics-scenario` · ○ `routing-policy` · ○ `approval`

**solver** - `worthy-solver-*`
○ `curve` · ○ `intersection` · ○ `surface`† · ○ `boolean`† · ○ `blend`† ·
○ `route` · ○ `clearance` · ○ `structure` · ○ `physics` · ○ `tolerance` ·
○ `optimization` · ○ `sdf`
† *the curve/surface/boolean/blend cut is unproven; the first blend operation
decides (Open Decision 2). Do not seed these on speculation.*

**derived** - `worthy-derived-*`
○ `brep` · ○ `cost` · ○ `approval` · ○ `compliance` · ○ `fabrication` ·
○ `conflict` · ○ `assumption-impact` · ○ `measure-audit` · ○ `physics-report` ·
○ `structural-analysis` · ○ `downstream-impact` · ○ `scene`

**pack** - `worthy-pack-*`
○ `policy` · ○ `assumption` · ○ `jurisdiction` · ○ `physics` · ○ `measure` ·
○ `component` - plus named content packs as they ship
(`worthy-pack-wall-corrugated-steel` style names are legal: the domain
segment for content packs is the component family noun)

**app / ui** - `worthy-app-*`, `worthy-ui-*`
○ `workbench` (app) · ○ `scene` · ○ `inspector` · ○ `review` · ○ `approval` ·
○ `ai-collaboration`

**cert** - `worthy-cert-*`
○ `replay` (home of Fence 2 proofs) · ○ `scale` (home of Fence 4, the scale
ladder) · ○ `approval` · ○ `perf` · ○ `workflows`

Product-tier names in this file are reserved vocabulary unless and until a
reviewed PR births the corresponding crate in code. This naming document does
not act as milestone progress authority.

---

## Exemplars

One crate per band is the designated **reference implementation** - the crate
`BOUNDARIES.md` routes imitation to and per-crate agent docs name. Exemplar
status is a field in the crate manifest and this table; changing an exemplar
is a reviewed act, because updating an exemplar propagates through all future
imitation.

| Band | Exemplar | Status |
|---|---|---|
| schema | `worthy-schema-topology` | reserved exemplar target |
| dsl | *unassigned* | first dsl crate to land assigns the exemplar |
| entry | `worthy-entry-boolean` | reserved exemplar target |
| resolver | *unassigned* | first resolver crate to land assigns the exemplar |
| solver | `worthy-solver-intersection` | reserved exemplar target |
| derived | `worthy-derived-brep` | reserved exemplar target |
| pack | first content pack (the corrugated wall) | first born pack assigns the exemplar |
| app/ui | *unassigned* | |
| cert | `worthy-cert-scale` | reserved exemplar target |

Rule: **an unassigned exemplar slot blocks breadth.** Do not fan agents out
across a band until that band has a designated exemplar. The first crate in a
band is written slowly, by hand or under close review, because it will be
copied thousands of times.

---

## Extending the reserved list

A PR that introduces a new domain noun must:

1. Add the noun to the table above in the same PR as the first crate using it.
2. Add or update the corresponding `BOUNDARIES.md` row(s), including the
   graph columns (layer, edge classes, aspects, spine scope).
3. State the pack-or-platform decision explicitly: why is this a platform
   crate rather than pack content?
4. Pass the fillet-style sniff test in review: given the new noun, can an
   outsider predict what lives there from the name alone?

If a capability has no obvious home under the grammar, **the taxonomy is
incomplete and this file gets clarified before implementation begins** - the
gap is the finding, not an inconvenience to route around.

---

## Unrepresentable names

The following are rejected by Fence 3 outright, with routing errors:

- `*-common-*`, `*-utils-*`, `*-helpers-*`, `*-logic-*`, `*-engine-*`, and
  any `core` outside the single size-fenced `worth-schema-core`
- milestone-shaped names (`*-m8-*`, `*-phase2-*`) and provenance-shaped names
  (`*-legacy-*`, `*-from-worth-*`) - compatibility code is named by the
  constraint it serves, inside a grammatical crate
- `worthy-query-*` - permanently confusable with `forge-query`; the band is
  `entry`
- `*-products-*` - the band is `derived`; "product" is reserved for the thing
  we sell
- legacy buckets: no new crate named `*-topo`, `*-spatial`, `*-kernel` as a
  band substitute

---

## Enforcement summary

| Check | Tool | Failure mode |
|---|---|---|
| name parses as `{tier}-{band}-{domain}` | boundary-check | reject with grammar explainer |
| domain is reserved or extended in-PR | boundary-check | reject with pointer to this file |
| band dependency law | boundary-check | reject naming the correct lane and the band exemplar |
| replay fence | boundary-check | reject: "replay is cert-only; consume `worthy-derived-*` through projection" |
| tier direction (`worth` never depends on `worthy`) | boundary-check | reject |
| per-crate agent doc current | agent-context | regenerate; hand-edited files fail CI |
| exemplar fields consistent with this table | agent-context | fail |
