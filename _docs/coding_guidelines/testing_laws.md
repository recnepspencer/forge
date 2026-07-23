# Testing Evidence Laws

> A test exists to expose a plausible defect in a real production claim. If it
> cannot name that defect, establish the world in which it matters, reach the
> relevant authority boundary, and observe the result independently, it is
> theatre.

## I. Proof Laws

1. Every test must own a falsifiable production claim, a plausible fault that would violate it, and an observation capable of distinguishing the violation. Passing proves only that claim under the established world; test count, line coverage, execution time, and assertion count are not evidence by themselves.

2. Choose the narrowest test boundary that can honestly prove the claim, not the boundary that is easiest to make green. Prefer end-to-end certification for product behavior, integration tests for contracts between real subsystems and authority boundaries, property, model, fuzz, or metamorphic tests for broad semantic spaces, and focused local tests only when the claim and an independent oracle are genuinely local.

3. The oracle must be epistemically independent of the implementation. A test may not derive its expected result through the production classifier, normalizer, comparator, formatter, query, or algorithm whose correctness it claims to prove. Shared constants and types are acceptable only when they are not the disputed semantics.

4. A test must fail for the intended reason. Establish relevant preconditions, trigger one named cause, inspect the typed outcome and consequential state, and distinguish setup failure, rejection, partial execution, cleanup, and observation failure. A panic, generic error, empty result, or unchanged value is not sufficient when multiple causes can produce it.

5. Important tests must be demonstrably sensitive to removal, inversion, bypass, stale reuse, or misrouting of the behavior they certify. Mutation probes, adversarial controls, positive and negative twins, or equivalent fault injection should show that the test detects the named defect rather than merely surviving execution.

6. Each test or deliberately parameterized family owns a unique proof obligation. If deleting a test removes no distinct evidence, consolidate or delete it. Repeating the same implementation path with different literals is not broader proof unless those values represent named semantic classes or boundary conditions.

## II. Fixture and World Laws

7. Fixture construction is part of the proof. Every identity, authority, capability, revision, relationship, token, resource, and persisted fact used as valid input must have causal provenance from the production mechanism that grants validity or from a fixture compiler proven to establish the same postconditions.

8. Tests declare semantically named worlds and scenario deltas rather than imperatively rebuilding incidental history. Maintain a small portfolio of canonical, causally complete baselines such as an empty installation, an ordinary governed tenant, contested collaboration, partial failure, and version boundary; derive each test world by applying only the delta relevant to its claim.

9. A fixture may bypass irrelevant public workflow history only through a narrow, explicitly privileged world compiler. That compiler must produce the same representation, invariants, indexes, authority relationships, versions, and recovery posture as production. It may not bypass the behavior under test or create a state that production could neither create nor encounter, except through an explicitly named corruption or recovery fixture.

10. Valid identities and authority-bearing values are issued by world construction and consumed through semantic handles such as `world.alice`, `world.project_owner`, or `world.pending_commit`. Tests must not counterfeit validity with copied integers, UUIDs, digests, tokens, timestamps, or opaque strings. Exact literals are reserved for invalid-input, wire-compatibility, canonicalization, and corruption claims where the literal itself is the subject.

11. Expensive setup must be architected for reuse without shared mutable fate. Compile immutable worlds once where possible, then clone snapshots, allocate isolated namespaces, restore checkpoints, or apply transactional deltas. Reuse must never introduce test ordering, mutable cross-test state, clock leakage, identifier collision, or cleanup dependence.

12. Fixture cost has explicit layers: workspace or process compilation, suite-level immutable baselines, test-level isolated deltas, and assertion-local actions. A test must not pay repeatedly for a lower-frequency layer merely because the harness lacks a truthful reuse boundary.

13. Fixture realism means causal completeness for the claim, not maximal data volume or accidental resemblance to one production sample. Canonical worlds cover named semantic regimes; deterministic generators, properties, fuzzing, and recorded seeds vary values and topology within those regimes. One golden world cannot certify a state space.

14. World construction, action, observation, and teardown are separately diagnosable phases. Fixture APIs return typed handles and failures, record the seed and scenario identity needed for reproduction, and expose which invariant or dependency failed. A broken fixture must never be reported as a product regression or allowed to satisfy an expected-failure assertion.

15. Fixtures obey production privacy, authority, retention, and secret-handling rules. Synthetic data is preferred; captured or replayed data requires explicit provenance, minimization, redaction, access policy, expiry, and deterministic replacement of credentials and identities.

## III. Test-Form Laws

16. An end-to-end test begins at a real product entry surface and observes a real user- or operator-visible consequence through the production composition root. If it replaces the authority owner, persistence semantics, scheduler, protocol, or observation path relevant to the claim, it is an integration test and must be named and scoped as one.

17. Integration tests cross real semantic and authority boundaries. They certify serialization, persistence, routing, lifecycle, cancellation, recovery, migration, and policy interactions using production wiring. Calling several in-memory objects from one test does not make the test integrative.

18. A focused local test is justified when it is the cheapest strong proof for a dense local semantic surface: an algorithm, parser, state machine, numerical invariant, canonicalizer, or exhaustive transition table. Tests that mirror private branches, assert constructor assignments, exercise getters, or freeze incidental call sequences are implementation surveillance and should not exist.

19. Mocks, fakes, and stubs prove only the behavior of the code against that substitute. They are valid for fault injection, rare external outcomes, and caller-side protocol logic when the substitute has an explicit contract; they cannot certify the real adapter, external system, persistence behavior, timing, or failure topology. Contract tests must bind substitutes and real implementations to the same observable obligations.

20. Compile-pass and compile-fail tests are reserved for public compile-time guarantees whose product value is that invalid programs are unrepresentable. Each negative case needs a corresponding valid case, must fail because of the intended authority or type boundary, and must not canonize incidental compiler prose. Consolidate cases into the fewest practical compiler sessions; do not use `trybuild` to test private implementation shape, ordinary runtime behavior, naming preference, or facts already proven by compiling production code.

21. Snapshot and golden tests are valid only when the serialized, rendered, diagnostic, migration, or protocol artifact is itself the contract. Review semantic differences, normalize only declared nondeterminism, and pair broad snapshots with focused invariants where a reviewer could miss a meaningful change. Snapshots of incidental debug output or enormous mostly irrelevant structures are approval theatre.

22. Stateful and asynchronous systems require adversarial lifecycle evidence: cancellation at each effect boundary, bounded backpressure, exhaustion, retries, duplicate and reordered delivery, concurrent conflict, partial persistence, crash and reopen, checkpoint plus journal recovery, schema coexistence, and irreversible commit posture. Happy-path completion cannot certify a lifecycle.

## IV. Cost and Integrity Laws

23. Test topology is performance architecture. In Rust, each integration-test file is a distinct crate and often a distinct compile and link unit; organize scenario families as modules within a small number of intentional harnesses, centralize fixture infrastructure behind stable support crates or modules, and prevent duplicated generic instantiation and dependency graphs across targets.

24. Every suite has named cost lanes and budgets for clean compilation, incremental compilation, linking, fixture construction, execution, external startup, retained artifacts, and flake retries. The ordinary change gate contains the cheapest evidence sufficient to reject unsafe changes; exhaustive fuzzing, soak, broad compatibility matrices, and destructive recovery may occupy scheduled lanes, but no correctness claim may depend solely on a lane that is rarely run or routinely ignored.

25. Test-only production branches, alternate composition roots, weakened validation, hidden constructors, and privileged mutation backdoors are forbidden. Testability must come from honest boundaries, explicit clocks and schedulers, observable effects, replaceable external ports, and governed fixture authority that preserves production semantics.

26. Flakiness is an unresolved correctness or harness defect, not statistical inconvenience. Quarantine removes a test's claim from the certified set and therefore requires an owner, reason, expiry, and explicit accounting of the lost evidence. Blind retries, widened timeouts, ordering constraints, and ignored failures may not manufacture green status.

27. Performance tests must measure the claimed boundary under named workloads, scale axes, environments, cold or warm posture, saturation, percentiles, and structural counters. A test that asserts only that work completed before a generous timeout is neither a performance contract nor a regression detector.

28. Test code obeys production composition, domain structure, and deletion discipline. Fixture infrastructure is a real subsystem with named ownership and boundaries, not a `helpers`, `common`, or `utils` bag. Remove obsolete scenarios, redundant assertions, stale snapshots, unused builders, and harness capabilities when their proof obligations disappear.

## V. Named Failure Modes

**Counterfeit world:** Validity is asserted through hard-coded identities, tokens, revisions, or relationships rather than causally established.

**Fixture amnesia:** Each test rebuilds expensive history because the harness has no reusable world or isolated-delta architecture.

**Universal fixture:** One enormous mutable setup obscures relevant preconditions, couples unrelated tests, and makes failures nonlocal.

**Self-certifying oracle:** Expected results are produced by the same semantics or implementation path under test.

**Wrong-reason green:** The assertion passes because setup, routing, or observation failed before the intended behavior was exercised.

**Boundary cosplay:** An in-memory call graph is labeled integration or end-to-end while replacing the authority or effect boundary that matters.

**Identity theatre:** A literal identifier is mistaken for a causally valid entity or authority relationship.

**Compile-contract inflation:** Compiler sessions are multiplied to police incidental syntax or private implementation rather than valuable public impossibility.

**Coverage theatre:** Executed lines, snapshots, assertions, or test count are presented as proof without fault sensitivity or an independent oracle.

**Test-only architecture:** Production semantics change under test configuration or a privileged bypass creates worlds unavailable to the real system.

**Sedimentary testing:** New tests accumulate without unique proof obligations, consolidation, deletion, or cost accounting.

## VI. Operational Review Vector

For every proposed test, identify the production claim, plausible defect, authority boundary, fixture provenance, world delta, independent oracle, intended failure cause, consequential observations, mutation sensitivity, isolation mechanism, unique evidence, and total compile plus execution cost. If these cannot be stated precisely, redesign the test or omit it.
