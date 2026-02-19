---
trigger: always_on
---

1. Directory Modularity: The "Bento Box" Pattern
   To minimize context bloat and keep logic findable, the workspace follows a Feature-Sliced Architecture.

One Folder, One Feature: Each complex feature (e.g., extrude, boolean) must have its own directory.

The Directory Manifest: Every feature directory must contain a mod.rs that acts strictly as a Table of Contents.

It must contain a //! doc comment manifest describing the domain, dependencies, and invariants.

It must only export public items; it should contain zero business logic or struct definitions.

Physical Separation of Concerns: Divide the feature into standard files:

schema.rs: Data shapes (Structs, Enums).

eval.rs: Pure business logic and mathematical algorithms.

topo.rs: Stateful topology mutations (Euler Operator implementations).

tests.rs: Unit tests for the feature.

The 400-Line Guideline: If any single file exceeds ~400 lines, it must be reviewed and subdivided to maintain AI attention window efficiency.

2. Function Rules (Verbs)
   Functions are the primary units of action and must prioritize Strong Cohesion.

Naming: Functions must be named as verbs (e.g., compute_area, split_edge).

Single Responsibility: A function must do one thing and one thing only. If the word "and" is needed to describe it, it must be split.

Linear Execution: Prefer early returns to minimize nesting. Avoid continue or break in loops; modify the controlling expression or use if guards instead.

Pure by Default: Functions should depend only on inputs and return outputs without side effects. Use &mut only when state mutation is strictly required.

Explicit Parameters: Avoid "Mystery Guests." All data required by a function must be passed in as parameters.

Verb Dictionary:

get\_\*: Retrieve value.

set\_\*: Update attribute.

insert/remove: Collection membership.

is*\*/can*_/has\__: Boolean predicates.

3. Class (Struct) Rules (Nouns)
   Rust Structs represent the "Nouns" of the system and must prioritize Data Protection and Fidelity.

Naming: Structs/Enums must be nouns representing concrete concepts (e.g., Face, ModelingContext).

Strict Encapsulation: Domain attributes must be private. Provide public accessors (get*\*) and mutators (set*\*).

Fidelity: A struct must accurately and completely represent its design concern. Avoid using raw primitives (like u32) for IDs; use Typed Generational Handles.

Safe Instantiation: Constructors (new or build) must validate all input and ensure the object is in a valid state before returning.

Composition Over Inheritance: Use Traits to define shared behavior. Favor "has-a" relationships over deep trait-bound hierarchies.

4. Operational Doctrines
   Certified Decision Firewall (D3): All topological decisions must flow through CertifiedTriSign. Agents are forbidden from using raw f64 comparisons to drive topology mutations.

Atomic Transactionality (D6): All mutations must be performed on a MutableDraft. Topology is only committed to the TopologyState if the entire operation succeeds.

Ambiguity Protocol (D2): Never make silent heuristic guesses. Call the check_tolerance! macro to log a ToleranceDecision or return KernelError::PolicyRequired.

Data-Driven Design: Move all magic numbers, thresholds, and algorithm data into named constants.

Explicit Coincidence (D0): Never "perturb" away TriSign::Zero. Utilize the CoincidenceGraph to handle flush/coincident geometry according to explicit policy.

5. Documentation & Verification
   Doc Comments Only: Use /// for public items and //! for module headers. Inline // comments are prohibited; name variables better instead.

Unit Isolation: Test methods must be straight-line code (no loops/conditionals) and must use Doubles (Stubs, Mocks) to isolate the Subject from its Collaborators.

Implementation Sequence: Follow Top-Down Design (specify systems/interfaces first) but Bottom-Up Development (implement and test leaf functions like Euler operators before high-level features).

Test Before Refactor: Never refactor working code without an existing suite to catch regressions.

6. Tech-Stack Specifics
   Typed IDs: Never use raw usize or u32 for IDs. Use the typed generational handles from forge_topo::handles.

Angular Logic: Keep components thin; place all state and mutation logic in Headless Managers/VMs (e.g., crud-manager.ts).

Zero Variable Reuse: Never reassign a variable to a different concept.

7. Observability & Tracing
   Universal Envelope: Every kernel operation must return `OperationResult<T>`, which wraps the value + `DecisionLog` + timing + state hashes.

Automatic Trace Persistence: `OperationResult::into_value()` auto-persists the `DecisionLog` to disk when `FORGE_TRACE_DIR` is set. Agents must NOT manually call `persist_trace` — the envelope handles it.

Environment Variables:

`FORGE_TRACE_DIR`: Absolute path to trace output directory. Set for test runs that should produce traces.

`FORGE_LOG`: Controls stderr output (`off`, `compact`, `full`).

Trace Inspection: Use the `/testing-and-tracing` workflow for the standard test-and-inspect flow. Start with `issues`, then `show`, then `decisions`.
