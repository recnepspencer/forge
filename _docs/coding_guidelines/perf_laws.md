Performance Architecture Laws
I. Core Laws
Execution breadth must be bounded by semantic delta. Invalidation breadth, recomputation breadth, maintenance breadth, retention breadth, and flush breadth must not exceed the actual surface of changed meaning.

Authoritative truth, execution-derived structure, and consumer-facing projection must not share the same maintenance contract. Mutation cost must not scale with the number of views, indexes, explanations, diagnostics, or convenience surfaces attached to the same underlying state.

Semantic intent should compile into execution strategy rather than being repeatedly reinterpreted during execution. Planning, lowering, normalization, and contract resolution should occur before the hot path consumes the work.

API shape must reflect traversal cost, reconstruction cost, synchronization cost, allocation cost, and breadth cost. Cheap-looking surfaces must not conceal broad scans, graph walks, reconstructive work, or rich-path behavior.

Locality boundaries and graph topology must be represented architecturally. Partition, region, scope, branch, adjacency, and similar locality forms must exist as first-class constraints on execution rather than being recovered later as optimization heuristics. Relationship storage must reflect traversal direction and frequency, not schema normalization. Bidirectional relationships require dual-indexed storage; the dual stores are distinct access patterns, not redundancy. Embed adjacency in the source entity when traversal is always source-initiated; use separate edge stores when the relationship set is independently maintained or batch-reconciled.

No reuse without an explicit equivalence contract. Caching, suppression, memoization, diffing, and incremental recomputation require stable identity, canonical ordering, and a precise invalidation basis sufficient to justify reuse as a semantic claim rather than a heuristic accident.

Policy and topology decisions belong before execution. Access, contract resolution, dispatch narrowing, and other control-plane decisions should be resolved before execution unless runtime facts are themselves the object of execution. The data plane should consume the narrowest execution path that can be resolved honestly upstream.

Boundary cardinality must match domain cardinality. Bulk domains must not be forced through scalar orchestration surfaces that externalize loops, fragment execution, and destroy amortization opportunities.

Expensive or critical properties must not be repeatedly rediscovered. Once sortedness, normalization, canonicality, eligibility, acyclicity, locality, or another costly fact has been proven within a trust boundary, later phases should consume that proof-bearing form or an equivalent explicit contract.

Breadth, richness, and coordination cost must degrade by explicit policy. Do not let these properties degrade by hidden defaults, incidental coupling, or accidental phase changes under scale.

Structural waste dominates constant waste. Scope failure, breadth failure, repeated maintenance, path conflation, projection inflation, repeated rediscovery, and avoidable coordination should be removed before tuning constants, capacities, layouts, or instruction-level details.

A performance claim is valid only at the boundary it names. Claims are only interpretable with counters that explain the work performed. End-to-end claims require end-to-end measurement; slope claims require scale-sensitive measurement.

Mechanical access patterns must reflect bulk processing, not conceptual taxonomy. Data evaluated together must be laid out together; CPU cache lines do not respect conceptual domains. Each pointer indirection on a hot path is a potential cache miss, and chains of indirection compound latency multiplicatively. Prefer flat, indexed, or arena-backed addressing over pointer-chasing topologies. Identity handles in performance-critical domains must encode as direct arena indices (index + generation), not as key-based lookups that impose hash computation and comparison cost per resolution.

Allocation is global coordination, not local arithmetic. Dynamic memory allocation is a hidden synchronization event with the memory subsystem and a deferred structural tax. It is not free compute. Operational hot-paths must execute within pre-allocated, arena-bounded, or explicitly lifecycle-managed footprints.

Allocations must belong to an explicit lifecycle scope. Arena, pool, bump, or reuse strategies should serve the dominant lifetime rather than defaulting to the general-purpose allocator. A pre-sized transaction-local buffer that clears between units of work is structurally superior to per-call heap allocation of identical capacity.

Throughput scales exclusively with data independence, not core count. Shared mutable state forces systemic serialization. Reference counting is a common disguised form of contention: every clone and drop of a shared pointer is an atomic read-modify-write on a shared cache line, and high-frequency traffic here can dominate execution cost invisibly. True parallelism requires structural disjointness.

Rejection must precede expensive construction. Execution should evaluate the cheapest, most restrictive disqualifying constraints before allocating memory, traversing topology, establishing coordination, or constructing rich intermediate state.

II. Tradeoff Laws
Structural maintenance should be amortized across the largest semantically honest boundary. Batch when intermediate states are not semantically observed; maintain incrementally when latency slicing, memory pressure, visibility, or interaction semantics require intermediate observability.

Carry proof forward when maintenance is cheaper than rediscovery. Re-derivation is acceptable when proof maintenance would increase invalidation breadth, representation cost, or coupling more than recomputation costs.

Data should be moved rather than cloned. Move by default; clone only to buy an explicit semantic, temporal, ownership, isolation, or retry boundary. A clone without a second observer, boundary shift, or failure-containment purpose is structural waste.

Defensive re-proof is a defect inside a trusted boundary. It is a necessity at an untrusted boundary. Re-validation, re-canonicalization, and re-eligibility checks should occur where authority, version, ownership, or trust changes—not blindly at every layer.

Incremental maintenance is superior only when invariant cost remains lower than recomputation. Large maintained surfaces, fragile invariant webs, or broad invalidation can make rebuilds from source cheaper than incremental repair.

Sparse-optimized strategies are superior only while activity density remains low. Event-driven, change-tracked, and incremental systems must preserve an exit path to denser, brute-force execution modes when tracking overhead dominates saved work.

Partition boundaries are profitable only when they improve the total cost surface. Communication density is a major axis, but partition only when the gains in locality, ownership, or isolation exceed the added coordination cost.

Speculative reflection of effects is a latency strategy, not a throughput law. Speculate only when divergence, rollback, reconciliation, and authority-lag costs remain lower than the latency cost of waiting for authoritative truth.

III. Named Failure Modes
Scope leakage: Invalidation, recomputation, retention, or flushing exceeds semantic delta.

Projection inflation: Derived, cached, indexed, explanatory, or diagnostic structures are maintained as though they were authoritative truth.

Plan / execute conflation: Execution reconstructs decisions that should have been resolved once during planning, lowering, or normalization.

Boundary cardinality mismatch: Scalar orchestration is imposed over bulk semantics.

Per-edit structural maintenance: Topology, indexing, subscriber state, bookkeeping, or diagnostics are maintained per sub-edit inside a semantically atomic operation.

Equivalence drift: Reuse surfaces exist without stable sameness semantics, canonical order, or disciplined invalidation.

Path conflation: Operational paths inherit diagnostic, forensic, explanatory, or reconstructive cost by default.

Breadth-coupled coordination: Local intent induces deep synchronous propagation or broad coordination inside one operational boundary.

Repeated rediscovery: Later phases re-filter, re-prove, or re-derive facts already established upstream inside the same trust boundary.

Mechanical / semantic collapse: Mechanical optimization corrupts semantic boundaries, or semantic layering ignores mechanical cost to the point of dishonesty.

IV. Operational Judgments
For any disputed design, judge it on these axes:

Semantic scope: How much meaning actually changed?

Execution breadth: How much work fanout does the design induce?

Maintenance surface: What derived structures must stay coherent?

Equivalence basis: What justifies reuse?

Locality preservation: Does work remain near the data and relationships it touches?

Boundary honesty: Does the API reveal or conceal actual cost?

Coordination depth: How much synchronous coupling is induced?

Proof strategy: Should facts be preserved, recomputed, or revalidated at a boundary?

Lifecycle fit: Do allocation and buffering strategies match the dominant lifetime?

Density regime: Is the workload sparse, dense, bursty, or phase-shifting?

Authority model: Where does truth live, and where are projections allowed to lag?

Measurement boundary: What counters would prove the claim at the boundary being optimized?

V. Condensed Tradeoff Templates
Batch vs incremental: Choose the strategy with the lower total invariant cost across the true semantic boundary.

Recompute vs maintain: Recompute when maintaining correctness across state transitions costs more than rebuilding from authority.

Move vs clone: Move by default; clone only to buy a real boundary.

Carry proof vs rediscover: Carry proof inside trusted boundaries; rediscover or revalidate at trust shifts.

Sparse vs brute-force: Use sparse execution only while tracking overhead stays below recomputation waste.

Partition vs unify: Partition only when the gains in locality, ownership, or isolation exceed the added coordination cost.

Speculate vs wait for authority: Speculate only when reconciliation is cheaper than latency.

Pre-resolve vs runtime-resolve: Resolve upstream unless the runtime fact is genuinely unavailable before execution or is itself semantically dynamic.