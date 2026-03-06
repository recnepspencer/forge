# Determinism Rules (forge-topo)

This policy defines collection and ordering requirements for any data that can be observed outside local scratch computation.

## Rule 1: No nondeterministic iteration in observed paths

Use deterministic containers for any collection that feeds into:
- replay payloads or replay log ordering
- lineage event ordering
- serialization output
- structural hashing input
- public query results
- test goldens or byte-level comparisons

Preferred containers:
- `BTreeMap`
- `BTreeSet`
- `IndexMap` (only when insertion order is itself deterministic)

`HashMap`/`HashSet` are allowed only for transient scratch where iteration order is never observed.

## Rule 2: Canonicalize before output

If data originates from order-unstable sources, canonicalize before output:
- sort entity refs by `(kind, index, generation)`
- sort IDs by stable key (`index`, then generation where relevant)
- keep face-loop output ordered as `outer` then `inners`

## Rule 3: Stable traversal semantics

Walkers and iterators must not depend on pointer map iteration order.
They must be driven by explicit topology pointers (`next`, `prev`, `radial_next`) and stable seeds.

## Rule 4: Review gate for new maps/sets

Any new map/set added in `forge-topo` must be reviewed for observability:
- if observable: use deterministic container or sorted output
- if unobservable scratch: document that assumption near use

## Rule 5: Scoped cache invalidation first

Tier-0 cache effects must be scoped by affected entity IDs whenever possible.

- allowed: `RadialLinksChanged { half_edges }`
- allowed: `FaceHalfedgesChanged { faces }`
- allowed: `VertexHalfedgesChanged { vertices }`
- allowed: `ShellFacesChanged { shells }`
- restricted fallback: `GlobalInvalidate { domain, reason_code, site }`

Global invalidation is disabled by default in runtime policy and may only be
used in sanctioned cache-runtime paths.

## Rule 6: Deterministic cache refresh trace

Cache refresh scheduling is part of determinism, not an internal detail.

- each operation records a canonical cache refresh trace
- trace entries are ordered by deterministic domain + scoped target order
- replay determinism includes cache refresh trace equality

## CI Enforcement

- `python3 scripts/ci/check_determinism_guards.py`
  - bans `println!`/`dbg!` in non-test `forge-topo` sources
  - bans `HashMap`/`HashSet` tokens in observable paths:
    - `provenance/`
    - `persistent_naming/`
    - `transactions/`
    - `semantic_attributes/`
  - bans direct `TopoCacheEffect::GlobalInvalidate` usage outside sanctioned paths
- `make check` runs determinism guards by default.

## Canonical Utilities

- Shared canonical sort helpers live in:
  - `crates/forge-topo/src/canonical.rs`
- Use these helpers for externally observed entity lists and lineage/replay payload field ordering.
