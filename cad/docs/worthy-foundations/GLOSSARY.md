# Glossary

**Status:** Living. Every term an outsider would trip on, in plain English.
**Rule:** if a review, prompt, or doc uses a term not defined here, either
define it here or use a plainer word. Agents will quote this file back at
us - the quotable form must be the clear one.

Organized by the confusions it exists to prevent, then an A-Z of single
terms.

---

## The Pairs (near-synonyms that are not synonyms)

### Resolver vs Solver

**Resolvers decide what should be true. Solvers compute what is true.**

A resolver is a semantic engineering family: which constraints apply here,
which placements are admissible, which solver calls to make, which advisories
to surface. A solver is a pure kernel: what curve intersection exists, what
route satisfies these constraints, what structure carries these loads.
Resolvers may call solvers. Solvers never touch the graph, the runtime, or
resolver authority. In plain English these words are near-synonyms; in this
tree they are different bands with a fence between them.

### Worth vs Worthy

**Worth is the engineering platform. Worthy is the CAD/BIM product built on
it.** The tier test: *would aerospace need this crate unchanged?* Yes ->
`worth-`. No -> `worthy-`. One letter apart, permanently easy to misread -
say "platform tier" / "CAD tier" out loud when it matters.

### Worth (platform tier) vs Worth (legacy codebase)

Two different things sharing a name. **Legacy Worth** is the old kernel tree
(`worth-topo`, `worth-spatial`, `worth-kernel`): a research corpus and
migration reference, closed to new capability. **The `worth-` tier** is the
live platform prefix. "No new code in Worth buckets" refers to the former;
`worth-schema-graph` is the latter. When ambiguous, say "legacy Worth."

### Derived vs Product

**Derived** is the band name for published, rebuildable artifacts
(`worthy-derived-cost`, `worthy-derived-brep`). **Product** is reserved for
the thing we sell. v1 used "products" for both; this tree does not.

### Entry vs Query

**forge-query** (usually just **Query**) is the runtime layer we build on.
**`entry`** is our band of crates where declared work enters Query
(`worthy-entry-boolean`). There are deliberately no `worthy-query-*` crates -
that name is permanently confusable with the runtime itself.

### Ordinary consumption vs Replay

The distinction the three-minute test taught. **Ordinary consumption** is the
fast lane: accept a validated upstream result (a derived artifact, through
projection consumption) and move forward without reopening its derivation.
**Replay / reconstruction** is the certification lane: rebuild, audit, or
prove the truth from history. Fence 2 makes the separation mechanical -
replay surfaces are importable only from `cert` crates. If ordinary code
"needs" replay, the derived artifact it should be consuming is missing.

### Entity vs Aspect vs Payload

The three granularity dials, coarse to fine. An **entity** is a graph node
with identity (a wall). An **aspect** is a named semantic facet of an entity
that dependency and invalidation key on (the wall's finish-language vs its
structural participation). **Payload** is bulk data carried by a derived
artifact and referenced from the graph, with no graph identity of its own
(the wall's ten thousand tessellation triangles). The graph scales because
almost everything is payload, entities are sparse, and aspects keep one
entity from being an invalidation bomb.

### Touch vs Traversal

A **touch** is a declared statement of what graph meaning an operation
affects - entity kinds, edge classes, aspects, spine scope - from which Query
*derives* obligations and invalidation. A **traversal** is walking the graph
to discover consequences. The constitution's governing property: consequences
are derivable from declared touch shape; discovery traversal on the ordinary
path is the smell that the schema is wrong.

---

## Query-Speak (the runtime's tribal vocabulary, translated)

These come from the Forge Query docs and appear throughout ours. Precise, but
tribal - translations:

- **Lane** - a runtime-owned path for a category of work (declaration,
  projection consumption, recovery). "Use the lane" = don't build your own
  version beside it.
- **Admitted / Admission** - the runtime has checked a request or capability
  and accepted it as legal *now*, with evidence. Visibility in the API is not
  admission; the support matrix is.
- **Posture** - the current honest status of a surface or capability:
  supported, deferred, denied, preview-local, etc. Machine-checkable, not
  guessed from autocomplete.
- **Honest** - a path whose claims are backed by typed evidence (receipts,
  support rows) rather than by convention or hope. "Structurally honest" =
  the architecture makes the dishonest version hard to write.
- **Folklore** - caller-owned reinvention of something the runtime already
  owns: local status enums, string identities, hand-rolled validation loops,
  copied digests, pseudo-Query wrappers. The failure mode this whole tree is
  designed against. Folklore never merges.
- **Declaration** - work expressed once with canonical identity, then lowered
  by the runtime. The opposite of imperative poking.
- **Projection consumption** - the typed, receipt-backed lane for reading
  facts the runtime already materialized, without reopening the authority
  that produced them. How every layer of the pyramid reads the layer below.
- **Receipt / Envelope** - typed artifacts recording what actually happened
  (what was touched, under which authority, with what evidence), so later
  code never reconstructs the story from logs or deltas.
- **Obligation** - a check the runtime owes before an operation is honest
  (blocking invariant, schema validator, advisory, gate). Selected from
  declared touch shape, not from caller memory.
- **Basis** - the specific world a read or write is bound to (authoritative,
  branch, preview, historical), carried as a typed capability rather than a
  raw id.
- **Grouped neighborhood** - an operation whose members belong together
  *semantically* (a corridor, an assembly edit), declared as one grouped unit
  rather than a loop of isolated declarations.
- **Budget denial** - the runtime refusing work whose declared shape is too
  wide (`BudgetExceeded`, `persistent_index_required`). **Design feedback,
  not an error**: narrow the touch, add the index, or split the neighborhood.
- **Consumer Kit** - Query's owned machinery for proving a downstream crate
  consumes Query correctly. The alternative to hand-rolled proof, and the
  quarantine mechanism behind "folklore never merges."
- **Residue** - folklore that survived adoption: local proof structs, raw
  support-row reads, debug-derived identity strings. Audited, named, capped.

---

## Our Vocabulary (this tree's own terms)

- **Band** - the authority axis of the grammar: `schema`, `dsl`, `entry`,
  `resolver`, `solver`, `derived`, `pack`, `app`/`ui`, `cert`. Fixed set.
- **Domain** - the meaning axis: what a crate is *about*. Open set, grown by
  reviewed extension of the reserved list in `NAMING.md`.
- **Tier** - `forge` / `worth` / `worthy`: runtime, platform, product.
- **The grammar** - `{tier}-{band}-{domain}`. The actual architecture; the
  folder tree is packaging.
- **Reserved name** - a guarantee of what a crate *will* be called. Not a
  crate. Crates are born on first real code.
- **Exemplar** - the designated reference implementation for a band, named in
  `NAMING.md`. What agents imitate; updating it is how patterns propagate.
- **Fence** - a mechanically enforced law (`tools/boundary-check`): band
  dependencies, the replay fence, grammar enforcement, the scale ladder.
  Everything not fenced is advice.
- **The graph constitution** - Part IV of `ARCHITECTURE.md`, embodied in
  `worth-schema-graph`: layers, edge classes, spine, aspect discipline,
  promotion.
- **Layers (L0-L4)** - the graph's vertical bands: context -> intent ->
  resolution -> geometry -> products. L0-L1 authoritative; L2-L4 derived;
  invalidation flows only downward along DERIVES edges.
- **The five edge classes** - CONTAINS, ATTACHES, REFERENCES, PARTICIPATES,
  DERIVES. Typed by *propagation semantics*, not domain meaning. Domains add
  entity kinds forever; they almost never add edge classes.
- **The spine** - the containment hierarchy (site -> building -> storey ->
  zone/bay -> component) as first-class graph infrastructure. Locality is
  graph-native; "which regime governs this?" is a shallow walk, never a scan.
- **Corridor** - a first-class spine entity for legitimately wide work (MEP
  runs, load paths): declared as a grouped neighborhood, budgeted as one wide
  touch. Wide is legal; *emergently* wide is not.
- **Promotion (on reference)** - a subelement acquires graph identity at the
  moment something durable refers to it, and not before. The sparse-identity
  answer to persistent naming.
- **Pack** - a distributable knowledge bundle (components, policies,
  jurisdictions, physics models) extending the platform through declared
  seams. The domain axis. First-party knowledge ships as packs too.
- **Touch envelope** - an entry operation's named complexity contract
  ("touches O(component neighborhood)"), asserted against budget receipts.
- **The scale ladder** - `worthy-cert-scale`: canonical edits at 10^3/10^5/10^7
  entities, asserting flat touch-shape and fan-out. The certified form of the
  scaling claim.
- **Ergonomic parity** - the standing metric that the canonical
  implementation of a task must cost within ~2x the naive one in tokens.
  Violations are facade bugs, because agents take the locally cheapest
  continuation.
- **The three-minute test** - the origin story: a stage-N+1 test that took
  three minutes because it reopened stage-N replay instead of consuming a
  derived artifact. Now the name for the whole failure class Fence 2 makes
  unwritable.
- **The pyramid** - the reconciliation shape: each layer consumes the layer
  below through projection consumption and publishes shaped facts upward;
  cost reads quantities, not geometry; compliance reads zones, not faces.
  Edit cost proportional to consequence, not world size.
- **BOUNDARIES row** - the unit of routing truth: task sentence -> route ->
  graph shape -> reason. An unfillable row is a taxonomy gap, filed - never
  improvised around.
