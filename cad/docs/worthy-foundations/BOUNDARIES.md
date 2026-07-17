# Boundaries

**Status:** Living routing table. Written before the first crate; every new
domain noun adds rows here in the same PR (`NAMING.md`, "Extending the
reserved list").
**Audience:** any agent or human asking *"where does this go?"*

This document answers one question, repeatedly: **given a task sentence,
which crate owns the change, and what graph shape does it touch?** If a task
cannot be routed by the procedure below, that is a taxonomy gap - stop, file
it against `NAMING.md`, and do not improvise a home.

---

## The Routing Procedure

Answer in order; each answer narrows the next.

1. **Pack or platform?** Does this require *new runtime capability* (a new
   entry lane, a new solver kernel, new contract grammar) - or only *declared
   knowledge over admitted seams* (a component, a policy bundle, a
   jurisdiction rule)? Knowledge -> pack. Capability -> continue.
2. **Tier.** Would aerospace need this unchanged? Yes -> `worth-`.
   No -> `worthy-`.
3. **Domain noun.** What is this *about*? One word from the reserved list
   (or extend the list, visibly).
4. **Band.** What kind of authority does the change hold? Defining meaning ->
   `schema`. Speaking intent -> `dsl`. Entering the runtime -> `entry`.
   Deciding what should be true -> `resolver`. Computing what is true ->
   `solver`. Publishing consequences -> `derived`. Facing a user -> `app`/`ui`.
   Proving -> `cert`.
5. **Name = `{tier}-{band}-{domain}`.** If the crate doesn't exist, check
   `NAMING.md`: reserved -> birth it; unreserved -> extend the list in-PR.
6. **Graph shape.** Before writing code, fill the graph columns: which
   layer(s), which of the five edge classes, which aspects, what spine scope.
   If you cannot fill them, you do not yet understand the change.
7. **Imitate the exemplar** named for that band in `NAMING.md`.

All `entry` routes share one installed operating-world root. A task may select
a typed operation-family facade, but that family cannot create a runtime, bind
raw graph handles, or act as a second public authority root.

Two standing reminders:

- **A Query budget denial reached during the work is design feedback**, not
  an error to route around. Narrow the touch, add the index, or split the
  neighborhood - and if the operation's declared envelope was wrong, fix the
  envelope in the entry crate.
- **If the canonical implementation costs more than ~2x the naive one in
  tokens, file an ergonomic-parity bug against the entry facade** before
  writing the naive version anywhere.

---

## Road 1 Public Routing Proof

Milestone 1 closes with two born platform-tier specimens and three deferred
follow-on names. This is a routing proof only: the born facades are public now,
and the later lanes are named now, but no placeholder `worth-entry-*` or
`worth-derived-*` crate is born just to make the story look complete.

| Specimen or follow-on | Public route now | Deferred next home | Why this is enough in Milestone 1 |
|---|---|---|---|
| Foundational identity / naming / tolerance family | platform · `worth-schema-core` facade (`Identity`, `Name`, `Tolerance`, `Unit`) | `worth-entry-adoption` for Query-native declaration/adoption · `worth-derived-publication` for retained/publication posture · `worthy-derived-brep` for the first real consumer-facing retained artifact path | the born schema crate proves pure meaning only; Query adoption and retained consumption stay deferred to later milestones |
| Pack-seam descriptor specimen | platform · `worth-pack-registry` facade (`ContributionKind`, `PackRegistration`) | `worth-entry-adoption` for contribution adoption into runtime-owned work | Milestone 1 proves registration shape only, not real pack admission or ordinary runtime execution |
| Deferred declaration/adoption lane | named in `NAMING.md` as `worth-entry-adoption` | Milestone 3 birth only | legal route target now; no placeholder entry crate in Milestone 1 |
| Deferred retained/publication lane | named in `NAMING.md` as `worth-derived-publication` | Milestone 4 birth only | legal route target now; no placeholder derived crate in Milestone 1 |
| Deferred product retained artifact path | named in `NAMING.md` as `worthy-derived-brep` | Milestone 4 real downstream pressure specimen | the ordinary consumer path is named publicly now without smuggling later-class behavior into the born platform crates |

Two denials are part of this proof:

- `worth-schema-core` must not absorb Query-native declaration or obligation
  adoption behavior.
- `worth-pack-registry` must not absorb retained/publication or pack-admission
  behavior.

If a change needs those authorities before Milestone 3 or 4, the finding is
that the milestone boundary was violated, not that the born seed crates should
grow sideways.

---

## The Routing Table

Columns: **Task -> Route** (pack/platform · crate(s)) · **Graph** (layer ·
edges · aspects · spine) · **Why, in one line**. Detailed walkthroughs for
the hard rows follow the table.

| # | Task sentence | Route | Graph shape | Why |
|---|---|---|---|---|
| 1 | Fillet this edge | platform · `worthy-entry-transform` (lane), `worthy-solver-curve`/`-intersection` (math), promotion via `worth-schema-graph` grammar | L1 feature entity · REFERENCES promoted edge, CONTAINS to cell, DERIVES down · geometry-parameters aspect only · one cell | a fillet is authored intent about one component's geometry, not a graph-wide event |
| 2 | Add a corrugated steel wall component | **pack** · `worthy-pack-wall-corrugated-steel`, registered through `worth-pack-registry`; forces birth of `worthy-resolver-component` | L1 component + parameters · CONTAINS to cell, PARTICIPATES in systems, DERIVES to resolution · aspects partitioned by consumer (geometry / structural / cost / finish) | components are knowledge, not runtime capability - the pack seam is the home |
| 3 | Reroute this duct run around the new beam | platform lane + pack policy · `worthy-entry-route`, `worthy-resolver-mep`(◐), `worthy-solver-route`, `worthy-solver-clearance` | L1 corridor entity (grouped neighborhood) · PARTICIPATES from every crossed cell, DERIVES to route artifact · routing aspect · declared-wide corridor scope | wide work is *declared* wide: one corridor entity, one grouped publication - never per-cell fragments |
| 4 | Change primary structure steel->timber; show cost delta | platform · `worthy-entry-component` (parameter mutation), cost flows through `worthy-derived-cost` | L1 parameter write on structural-system aspects · invalidation down DERIVES: resolution -> quantities -> cost · spine scope = the system's PARTICIPATES set | cost never reads geometry; it re-derives from quantity facts - the pyramid, not a scan |
| 5 | Encode a Salt Lake City setback rule | **pack** · `worthy-pack-jurisdiction` (+ `worthy-schema-jurisdiction` grammar if the rule *kind* is new) | L0 regime bound to spine cells · applicability via CONTAINS walk · advisory/denial surfaces as L4 compliance facts | jurisdictions are knowledge bundles; only a genuinely new rule *kind* touches platform grammar |
| 6 | Undo my manual tweak from yesterday | platform · declared inverse or compensation through `worthy-entry-recovery`; certification replay remains `worthy-cert-replay` only | L1 authored declarations only · reversal or compensation is a new operation referencing the original receipt · derived layers rebuild | ordinary undo is typed aftermath, not graph surgery or replay; journal reconstruction stays behind Fence 2 |
| 7 | "Make the entry more dramatic" | `worthy-ui-ai-collaboration` -> decomposed into ordinary L1 declarations through typed `worthy-entry-*` family views borrowed from the one operating-world root | many small L1 writes · each with its own bounded touch · no special "AI lane" in the graph | AI authorship is ordinary authorship: one authority root, the same entry families, obligations, and receipts |
| 8 | The 7.5->7.6 handoff (stage N+1 consumes stage N) | platform · consumer depends on `worthy-derived-brep` **only**, via projection consumption | L3 artifact consumed through typed projection receipts · zero upstream re-derivation | replay from an ordinary consumer is a **compile error** (Fence 2) - the three-minute test, made unwritable |
| 9 | Pull *this* edge tighter (manual refinement) | platform · promotion grammar (`worth-schema-graph`), refinement entity via `worthy-entry-transform` | edge promoted to L1 identity on reference · REFERENCES edge · lineage-bound to carrying BREP artifact | identity is sparse and earned by reference; the other 99.9% of edges stay artifact payload |
| 10 | Run structural physics on the current model | platform · `worthy-resolver-physics-scenario` assembles from admitted regimes, `worthy-solver-physics` computes pure, `worthy-derived-physics-report` publishes | reads L0 regimes + L2 facts (never L3 payload directly) · DERIVES to L4 report · scope = the analyzed system's PARTICIPATES set | physics consumes resolution-layer facts through the pyramid; the solver sees extracted arrays, never the graph |
| 11 | Change an assumption set (e.g. soil bearing capacity) | platform lane + pack content · `worthy-entry-assumption`; bundles live in `worthy-pack-assumption` | L0 regime write · invalidation down DERIVES to every scenario that consumed it · scope = the regime's spine binding | assumptions are context truth: authored at L0, consumed by resolution, never ambient defaults in solver code |
| 12 | Add crown molding through the public spaces | pack content (molding component) + `worthy-entry-component` placement | L1 components · CONTAINS + ATTACHES · **finish-language aspect only** · public-space cells | the aspect partition is the point: finish edits never wake the structural solver |
| 13 | Render the scene after an edit | platform · `worthy-derived-scene`, consumed by `worthy-ui-scene` | L4 product derived from L2/L3 · aspect-filtered DERIVES · re-derives only touched cells | the scene is a derived product like cost - display never reads truth directly |
| 14 | Add a new blend/chamfer kernel | platform · decides Open Decision 2: lands in `worthy-solver-curve`/`-intersection` or births `worthy-solver-blend` | no graph contact at all - solvers take extracted inputs | the first blend *is* the experiment that settles the solver cut; route the decision to NAMING.md, not to a guess |
| 15 | Prove edit cost stays flat at 10^7 entities | platform · `worthy-cert-scale` | canonical edits (#1, #3, #4) benchmarked at 10^3/10^5/10^7 · asserts flat touch-shape + fan-out | the scale ladder is the graph-scaling claim, certified |
| 16 | Coordinate work across two independently authoritative graphs | platform · install one named participation adapter per genuine graph authority, then bind one typed cross-graph operation through the shared operating-world root | source meaning consumed through typed projection · target change emitted as typed effect · no graph-to-graph edge invented by the app | same-commit graphs may bind atomically; otherwise compensation and partial failure are declared before execution rather than hidden behind an adapter |

---

## Worked Examples (the rows that fight back)

### Row 1 - the fillet, end to end

The classic "no obvious home" case from v1. Under the grammar it decomposes
cleanly *because* the graph constitution splits it:

1. **Identity:** the target edge is promoted on reference - L1 entity,
   lineage-bound to the BREP artifact carrying its geometry
   (`worth-schema-graph` promotion grammar).
2. **Intent:** the fillet is an L1 feature entity - REFERENCES the promoted
   edge, CONTAINS-anchored to the owning component's cell, touching only the
   geometry-parameters aspect. Authored through `worthy-entry-transform`.
3. **Obligations:** selected from the touch shape by Query - geometric
   validity, continuity of other REFERENCES into the region, structural check
   *only if* a PARTICIPATES edge consumes the touched aspect. Zero traversal.
4. **Math:** the resolver extracts the local neighborhood into compact
   inputs; the blend kernel (row 14) computes with zero graph contact.
5. **Consequences:** invalidation down DERIVES - resolution -> BREP artifact ->
   quantity facts -> cost; physics only if section properties moved.

The crate question ("curve vs surface vs intersection?") is deliberately
unresolved until step 4 happens for real. The *routing* is resolved now; the
*cut* is Open Decision 2.

### Row 2 - the corrugated wall (Sequence step 6)

This row is the acceptance test for the domain axis. The wall arrives as a
pack or the pack seam is not honest:

- noun system and parameter vocabulary -> contract contribution
  (`worthy-schema-component` grammar, pack-declared nouns)
- placement and assembly semantics -> forces the birth of
  `worthy-resolver-component` (which becomes the resolver-band exemplar)
- any kernel math -> existing solver crates
- cost/fabrication/approval consequences -> derived families
- creation and mutation -> ordinary `worthy-entry-component` declarations

If building this wall requires touching runtime internals, the pack seam has
a gap - file it as a platform bug, do not special-case first-party content.

### Row 3 - the corridor

The MEP row is the stress test for Axis 3. The failure shape to refuse: a
route stored as per-cell fragments with ATTACHES edges, which a consumer must
reassemble - that is the 7.6 entanglement rebuilt inside the graph. The
corridor is one first-class spine entity, declared as a grouped neighborhood,
budgeted as one wide-but-declared touch, published as one grouped
publication. "Wide" is legal; "emergently wide" is not.

---

## The Unfillable-Row Protocol

When a task genuinely cannot be routed:

1. Do not improvise a home, a helper crate, or a "temporary" location.
2. File the gap against `NAMING.md` with the task sentence and the two
   nearest candidate routes.
3. The resolution extends the reserved list, clarifies a band definition, or
   - rarely - amends the grammar in `ARCHITECTURE.md`.
4. The task's row is added here in the same PR that resolves it.

An unfillable row found today costs a conversation. The same gap found after
a hundred crates costs a migration.
