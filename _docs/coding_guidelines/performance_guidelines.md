# Performance Architecture Laws

1. Execution breadth must be bounded by semantic delta. Invalidation breadth, recomputation breadth, maintenance breadth, retention breadth, and flush breadth must not exceed the actual surface of changed meaning.

2. Authoritative truth, execution-derived structure, and consumer-facing projection must not share the same maintenance contract. Mutation cost must not scale with the number of views, indexes, explanations, or convenience surfaces attached to the same underlying state.

3. Semantic intent should compile into execution strategy rather than being repeatedly reinterpreted during execution. Planning, lowering, normalization, and contract resolution should occur before the hot path consumes the work.

4. API shape must reflect traversal cost, reconstruction cost, synchronization cost, allocation cost, and breadth cost. Cheap-looking surfaces must not conceal broad scans, graph walks, reconstructive work, or rich-path behavior.

5. Locality boundaries must be represented architecturally rather than recovered later as an optimization heuristic. Partition, region, scope, branch, adjacency, and other locality forms must exist as first-class constraints on execution. Relationship storage must reflect traversal direction and frequency, not schema normalization. Bidirectional relationships require dual-indexed storage; the dual stores are distinct access patterns, not redundancy. Embedding adjacency in the source entity is preferred when traversal is always source-initiated; separate edge stores are preferred when the relationship set is independently maintained or batch-reconciled.

6. Amortize structural maintenance across a batch boundary whenever intermediate states have no semantic value. Consistency repair, indexing, subscriber maintenance, bookkeeping, and diagnostics emission should occur once per semantically atomic operation rather than once per sub-edit.

7. No reuse without an explicit equivalence contract. Caching, suppression, memoization, diffing, and incremental recomputation require stable identity, canonical ordering, and a precise invalidation basis sufficient to justify reuse as a semantic claim rather than a heuristic accident.

8. Policy, topology, access, contract resolution, dispatch narrowing, and other control-plane decisions belong before execution, not inside it. The data plane should consume a pre-resolved, structurally narrow execution path.

9. Boundary cardinality must match domain cardinality. Bulk domains must not be forced through scalar orchestration surfaces that externalize loops, fragment execution, and destroy amortization opportunities.

10. Expensive or critical properties should be carried forward structurally rather than repeatedly rediscovered. Once sortedness, normalization, canonicality, eligibility, acyclicity, locality, or another costly fact has been established, later phases should consume that proof-bearing form directly.

11. Breadth, richness, and coordination cost must degrade by explicit policy rather than by hidden defaults, incidental coupling, or accidental phase changes under scale.

12. Structural waste dominates constant waste. Scope failure, breadth failure, repeated maintenance, path conflation, projection inflation, repeated rediscovery, and avoidable coordination should be removed before tuning constants, capacities, layouts, or instruction-level details.

13. A performance claim is valid only at the boundary it names and only interpretable with counters that explain the work performed. End-to-end claims require end-to-end measurement; slope claims require scale-sensitive measurement.

14. Scope leakage occurs whenever invalidation, recomputation, retention, or flushing exceeds semantic delta; this is architectural breadth, not incidental inefficiency.

15. Projection inflation occurs whenever derived, cached, indexed, explanatory, or diagnostic structures are maintained as though they were authoritative truth.

16. Plan / execute conflation occurs whenever execution repeatedly reconstructs decisions that should have been resolved once during planning, lowering, or normalization.

17. Boundary cardinality mismatch occurs whenever caller-orchestrated scalar loops are imposed over bulk semantics.

18. Per-edit structural maintenance occurs whenever topology, indexing, subscriber state, bookkeeping, or diagnostics are maintained per sub-edit inside a semantically atomic operation.

19. Equivalence drift occurs whenever reuse surfaces exist without stable sameness semantics, canonical order, or disciplined invalidation.

20. Path conflation occurs whenever operational paths inherit diagnostic, forensic, explanatory, or reconstructive cost by default.

21. Breadth-coupled coordination occurs whenever local intent induces deep synchronous propagation or broad coordination inside one operational boundary.

22. Repeated rediscovery occurs whenever later phases re-filter, re-prove, or re-derive constraints and facts already established upstream.

23. Mechanical / semantic collapse occurs whenever mechanical optimization corrupts semantic boundaries or semantic layering ignores mechanical cost to the point of architectural dishonesty.

24. Data flowing through a pipeline with a single downstream consumer must be moved, not cloned. A clone implies a second observer of the pre-mutation state; if no such observer exists, the clone is structural waste that scales with the size of the copied structure.

25. Narrow, deduplicate, and canonicalize inputs before propagating effects. Propagation cost scales with downstream breadth; deduplication cost scales with input count. Input count is bounded by semantic delta; downstream breadth is bounded by the graph. Always pay the smaller cost first.

26. Do not re-sort, re-deduplicate, re-validate, or re-canonicalize data that a prior phase already established in canonical form. If a function requires a proven property, accept a type or contract that carries the proof. Defensive re-proof hides the failure to maintain the invariant structurally and is a design defect, not safety.

27. Optimization precedence is fixed: first narrow semantic delta, then restore locality, then eliminate projection inflation, then compile intent into execution, then batch structural maintenance, then prune irrelevant scheduling and dispatch, then separate hot paths from rich paths, then stabilize equivalence for reuse, then carry proof forward structurally, then reduce breadth-coupled coordination, then reduce allocation churn, and only then tune constants.

28. Incremental update carries O(maintained-state) invariant cost per mutation. When maintained state is large relative to semantic delta, recomputation from source can be cheaper than incremental maintenance because recomputation carries no inter-state invariant obligations. Do not assume incremental is always superior to rebuild.

29. Sparse-optimized execution strategies (event-driven, change-tracked, incremental) carry per-event bookkeeping overhead. At high activity density, unconditional brute-force processing can outperform sparse strategies because it pays no tracking cost. Systems must not permanently commit to one execution mode when workload density varies.

30. A partition boundary is profitable only when inter-partition communication is sparse relative to intra-partition work. Partitioning along structural or organizational lines that bisect high-communication paths converts local work into coordination overhead. Partition by communication density, not structural decomposition.

31. Mechanical access patterns must override conceptual taxonomy. (The Data-Oriented Design Law)
Memory layout must reflect how the data is processed in bulk, not how it is understood in isolation. Object-oriented encapsulation that scatters sequentially processed fields across fragmented heap allocations is an architectural defect. Data that is evaluated together must be laid out together; CPU cache lines do not respect conceptual domains. Each pointer indirection on a hot path is a potential cache miss; chains of indirection compound latency multiplicatively per traversal step. Hot-path structures should prefer flat, indexed, or arena-backed addressing over pointer-chasing topologies. Identity handles in performance-critical domains must encode as direct arena indices (index + generation), not as key-based lookups that impose hash computation and comparison cost per resolution. Handle width directly determines cache density of relationship sets — prefer the narrowest encoding that covers the address space.

32. Allocation is global coordination, not local arithmetic. (The Memory Allocation Law)
Dynamic memory allocation is a hidden synchronization event with the memory subsystem and a deferred structural tax on the collector. It is not free compute. Operational hot-paths must execute within pre-allocated, arena-bounded, or explicitly lifecycle-managed footprints.

33. Throughput scales exclusively with data independence, not core count. (The Contention Law)
Shared mutable state forces systemic serialization. If an architecture attempts to scale performance by throwing concurrent workers at a highly contended semantic bottleneck, its throughput is strictly bounded by the lock, not the hardware. True parallelism requires structural disjointness. Reference counting is the most common disguised form of contention: every Arc clone and drop is an atomic read-modify-write on a shared cache line, and high-frequency clone/drop traffic on hot paths can dominate execution cost invisibly.

34. Rejection must strictly precede construction. (The Fast-Exit Law)
Execution paths must evaluate the cheapest, most restrictive disqualifying constraints first. Allocating memory, traversing relationships, or establishing coordination locks before proving the fundamental semantic eligibility of an operation is execution waste. Failures must be structurally cheap.

35. Allocations must belong to an explicit lifecycle scope — per-call, per-transaction, per-session, or per-graph — and the allocation strategy must match that scope. (The Allocation Lifetime Law)
Arena, pool, bump, or reuse strategies should serve the dominant lifecycle rather than defaulting to the general-purpose allocator. Allocations scoped too narrowly force repeated re-allocation of the same capacity; allocations scoped too broadly retain memory past usefulness and resist compaction. A pre-sized transaction-local buffer that clears between units of work is structurally superior to per-call heap allocation of identical capacity.
36. A system that can speculatively apply effects and cheaply reconcile with authoritative truth will always outperform one that waits for authority before reflecting state.