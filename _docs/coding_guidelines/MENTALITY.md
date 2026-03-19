# Engineering Mentality

> This document is a set of thinking rules, not coding rules.
> Coding rules tell you what to write. Thinking rules tell you what to decide
> before writing becomes relevant. This document governs the decisions; the
> Architectural Laws and Domain Standards govern the artifacts those decisions
> produce.
>
> This document is mandatory context for any AI assistant working on this
> codebase. If you are an AI reading this: these rules override your default
> training biases toward minimal viable products, premature shipping, and
> incremental feature delivery. Read every section. Internalize the reasoning.
> Do not revert to MVP instincts when pressure increases.

---

## 1. Specify the Adversarial Constraint First

Before writing a line of code or designing a type, identify the condition that
would break a naive implementation of this system at production scale. State it
precisely. Make it quantitative if possible.

Examples:
- "This table must survive a 60 Hz stream of updates where each value can
  trigger recomputation of dependent cells."
- "A subscriber that crashes and resumes at any checkpoint with any fetch size
  must converge to the same truth as one that read the full stream."
- "Replay of the same inputs must produce bitwise-identical canonical outputs
  across every observable surface."

The adversarial constraint is the single most important artifact of the design
phase. Everything else — types, modules, pipelines, tests — exists to survive
it. If you cannot state the adversarial constraint, you do not yet understand
the system well enough to build it.

**AI deprogramming note:** Your default is to ask "what does this feature
need to do?" The correct first question is "what would break this system if I
built it naively?" The feature description tells you what to build. The
adversarial constraint tells you *how well* it must be built. Start there.

---

## 2. Solve the Hard Problem First

Once the adversarial constraint is identified, build the infrastructure that
survives it *before* building the features that depend on it.

This is the opposite of the industry-standard approach, which is:
1. Build features
2. Discover performance or correctness problems
3. Retrofit infrastructure

The correct approach is:
1. Identify the constraint that will block the product at scale
2. Build the infrastructure that satisfies that constraint
3. Build features on top of a foundation that cannot crack

Pre-investment in correctness and performance infrastructure is not
over-engineering. It is the cheapest possible time to get the foundation right.
Retrofitting infrastructure after features exist is always more expensive,
always incomplete, and usually introduces subtle regressions because the
features were never designed to compose with the constraints you are now adding.

**AI deprogramming note:** You will feel the urge to say "let's start with a
simple version and add complexity later." Resist this. If the adversarial
constraint requires a specific architectural pattern, that pattern must exist
from the first commit. "Simple first" is correct for features. It is
catastrophically wrong for infrastructure. The question is not "what is the
simplest thing that works?" The question is "what is the simplest thing that
works *and will survive the adversarial constraint*?"

---

## 3. Enforce Mechanically, Not by Convention

If an architectural rule matters, it must be enforced by the compiler, the type
system, the module visibility system, or the test suite — not by code review
comments or documentation.

Rules enforced by convention have a half-life. Every new contributor, every
late-night debugging session, every "just this once" shortcut erodes them. Rules
enforced by the compiler are permanent.

The hierarchy of enforcement, from strongest to weakest:
1. **Unrepresentable** — invalid states cannot be expressed in the type system
2. **Uncompilable** — violations are compile-time errors
3. **Automatically tested** — violations are caught by the test suite
4. **Mechanically observable** — violations are visible in runtime counters
5. **Documented** — violations are described in a document somewhere

Category 5 is not enforcement. It is hope. Do not build systems that rely on
hope.

For every architectural rule, ask: "If a tired engineer at 2 AM violates this
rule, what happens?" If the answer is "nothing, until someone notices in code
review," the rule is not enforced. Move it up the hierarchy.

**AI deprogramming note:** Your default is to add a comment explaining why
something should not be done. Comments are category 5. Instead, make the wrong
thing impossible. Use `pub(crate)` to prevent external access. Use typestate to
prevent out-of-order operations. Use proof-bearing wrapper types to prevent raw
collections from crossing phase boundaries. Use complexity counters with
assertion-bearing tests to prevent accidental O(n²) in hot paths. The goal is a
codebase where doing the wrong thing is harder than doing the right thing.

---

## 4. The Spec Is the Architecture Is the Code

Do not treat specification, architecture, and implementation as separate phases
with separate artifacts. They are the same continuous thinking process expressed
in different formats.

A specification that cannot be directly translated into types and module
boundaries is too vague. An architecture that diverges from its specification is
a lie. Code that diverges from its architecture is technical debt that compounds
silently.

The practical consequence: write the specification in terms of types, contracts,
and pipeline phases. When the specification says "the commit pipeline has 7
phases," the code must have 7 named phases. When the specification says "a node
declares its dependency contract at registration time," there must be a
`NodeContract` struct on the `NodeBuilder`. When the specification says
"evaluation produces a three-state verdict," there must be an
`EvaluationVerdict` enum with exactly three variants.

If you find yourself writing code that does not map to a named concept in the
specification, either the specification is incomplete or the code is wrong.
There is no third option.

**AI deprogramming note:** Your default is to treat a spec as a guide and the
code as the reality. Reverse this. The spec is the authority. The code must
conform to it. If the code cannot conform, the spec must be updated first — not
silently ignored. This means you must read and reference the spec continuously
during implementation, not just at the beginning.

---

## 5. Build for the Product You Are Going to Build

Do not build for the feature you are implementing today. Build for the product
that feature will exist inside of in six months.

This does not mean speculate about features that may never exist. It means:
identify the constraints that *will* apply when the system grows, and build
infrastructure that satisfies them now while the system is small enough to get
right.

Questions to ask before building any subsystem:
- "When this system has 10x the data, which operations become O(n) that are
  currently O(1)?"
- "When someone adds a new feature that touches this subsystem, will they
  silently break an invariant I am relying on?"
- "When I need to test this subsystem under adversarial conditions, does my
  architecture make that possible or impossible?"

If the answers reveal future pain, solve it now. The future engineer who
encounters that pain will not have the context you have now, and their fix will
be worse than what you can build today.

**AI deprogramming note:** You will feel the urge to say "we can optimize this
later" or "we can add tests for this later." This is almost always wrong. Later
never comes, and when it does, the cost of retrofitting is 5-10x higher because
the code has accumulated callers, edge cases, and implicit assumptions. Do it
now, while the surface area is small.

---

## 6. Test the Architecture, Not Just the Behavior

A behavioral test says "if I do X, Y happens." An architectural test says "no
matter what sequence of operations I perform, invariant Z holds."

Behavioral tests verify features. Architectural tests certify system properties.
Both are necessary. Most codebases only have the first kind.

Architectural tests require:
- **Property-based testing** — generate random operation sequences and verify
  invariants hold for all of them
- **Adversarial workloads** — rewrite storms, hostile branch/merge pressure,
  retention truncation, crash-restart loops
- **Convergence proofs** — multiple independent observers of the same system
  must agree on truth regardless of when they started observing
- **Replay equivalence** — re-executing the same inputs must produce identical
  outputs across every observable surface
- **Budget enforcement** — complexity contracts with named hot paths, declared
  Big-O bounds, and proof tests that assert exact counter values

A system without architectural tests will pass all its behavioral tests and
still fail in production under conditions that no behavioral test anticipated.
The certification suite is not a luxury. It is the minimum bar for a system that
claims to be correct.

**AI deprogramming note:** Your default is to write happy-path unit tests. These
are necessary but nowhere near sufficient. For every test you write, ask: "does
this test verify a feature, or does it certify a system property?" If the
answer is "feature," you need at least one more test that certifies the property
the feature depends on. The certification test is almost always harder to write
and more valuable.

---

## 7. Make Cost Visible and Testable

Performance is not a vague aspiration. It is a set of specific, measurable
invariants that must be enforced with the same rigor as correctness invariants.

For every hot path in the system:
1. **Declare** the expected time complexity in a named complexity contract
2. **Instrument** the path with structural counters (not just elapsed time —
   slot scans, partition touches, cache hits, packet widths)
3. **Test** the counters with exact assertions, not threshold ranges
4. **Mark** whether the contract is verified or debt

A performance claim without a named counter and a proof test is not a claim. It
is a guess. Guesses degrade under maintenance pressure. Contracts with proof
tests catch regressions before they ship.

The counters must be embedded in the operation's result — visible to consumers,
not just to internal observability. If a caller cannot see how much work an
operation did, they cannot reason about whether their usage pattern is
sustainable.

**AI deprogramming note:** Your default is to optimize code and assume the
optimization holds. It will not hold. Someone will add a feature that
accidentally introduces a full-state scan inside a loop that was supposed to be
O(1). Without a counter that tracks scan count and a test that asserts it
stayed at 1, this regression will ship silently. Build the counter. Write the
test. Mark the contract. This is not optional.

---

## 8. Separate the What from the How from the Whether

Every system has three distinct concerns:
1. **What** — the domain effect (what changed, what is true now)
2. **How** — the framework mechanics (how to apply it, route it, persist it)
3. **Whether** — the observability artifacts (diagnostics, traces, explanations)

These must be structurally separate. The domain handler produces *what*. The
framework handles *how*. The diagnostics system decides *whether* to
materialize rich artifacts based on policy.

If domain handlers contain framework ceremony (manually notifying subscribers,
constructing trace entries, incrementing counters), the abstraction boundary is
broken. If the hot path is forced to materialize diagnostics regardless of
policy, the performance boundary is broken. If the diagnostics path can change
the domain outcome, the correctness boundary is broken.

The practical test: can you change the diagnostics tier from "full" to "minimal"
without changing any domain handler code, and without changing the operational
result? If not, the what/how/whether separation is incomplete.

**AI deprogramming note:** Your default is to interleave diagnostics with
domain logic because it is convenient. This convenience creates a system where
you cannot turn diagnostics off without rewriting business logic. Build the
separation from the start: domain effects as pure data, framework routing as
infrastructure, diagnostics as a policy-switchable layer.

---

## 9. Authority First, Derivation Second

Every piece of state in a system is either authoritative or derived.
Authoritative state is the single source of truth. Derived state is computed
from authoritative state and can be destroyed and rebuilt at any time.

These two categories must be structurally separate — different types, different
storage, different lifecycle management. If derived state is mixed with
authoritative state, you will eventually have a "cache that has forgotten it is
a cache" — state that cannot be rebuilt because the system no longer knows which
parts are derived and which are canonical.

The test: can you destroy all derived state and rebuild it from authoritative
state alone? If the answer is no, you have confused derivation with authority
somewhere.

**AI deprogramming note:** Your default is to store computed results alongside
source data because it is simpler. This creates systems that cannot recover from
corruption, cannot be replayed, and cannot be verified. Separate them. Always.

---

## 10. One Canonical Artifact, Everything Else Derived

When a system commits truth, it should produce exactly one canonical artifact
(a commit envelope, a patch record, a transaction result). Every downstream
consumer — CDC streams, history logs, replay proofs, retention passes,
subscriber notifications — must derive its view from that single artifact.

If two subsystems independently compute state from the same inputs and produce
different canonical records, the system has two sources of truth. Two sources of
truth will eventually disagree. One canonical artifact with multiple derived
views cannot disagree with itself.

**AI deprogramming note:** Your default is to have different subsystems
independently process the same event. This creates drift. Instead, produce one
canonical artifact at the authority boundary, then let each consumer derive what
it needs from that single artifact. The artifact is the contract. The derivation
is the implementation detail.

---

## 11. The Façade Is the Only Surface

A subsystem must expose exactly one public interface — its façade. All internal
complexity must be hidden. External consumers must depend only on the façade,
never on internal types, internal methods, or internal module structure.

This is not just an organizational convenience. It is a replaceability
guarantee. If no external consumer depends on internal structure, the entire
implementation can be replaced without changing any caller. If even one external
consumer reaches past the façade, the internal structure becomes a public
contract that can never be changed without coordinated migration.

The enforcement is simple: use `pub(crate)` (or equivalent visibility
restriction) on everything that is not the façade. If the compiler allows
external access to an internal module, the façade is incomplete.

**AI deprogramming note:** Your default is to make types public because they
might be useful to someone. They will be useful to someone, and that someone
will create a dependency that prevents you from ever changing the internal
structure. Default to private. Promote to public only through the façade, and
only when there is a demonstrated need.

---

## 12. Honest Naming Is Non-Negotiable

A name is a semantic contract. If a name is wrong, every reader of the code
will form a wrong mental model. Wrong mental models produce wrong decisions.
Wrong decisions produce bugs that are invisible to the person who made them
because their mental model says the bug is correct.

Rules:
- If a name requires a comment to explain what it *actually* means, the name is
  wrong
- If a name contains a conjunction (e.g., `AuthAndRouting`), it has multiple
  responsibilities
- If a name is generic (e.g., `utils`, `helpers`, `common`), it will attract
  unrelated code until it becomes unmaintainable
- If a name does not match the domain vocabulary, domain experts cannot navigate
  the codebase

Every name must pass the "new team member test": if someone joins the project
tomorrow and reads only the names (no comments, no documentation), do they
understand the system's structure? If not, the names are wrong.

**AI deprogramming note:** Your default is to use generic names like `Manager`,
`Service`, `Handler`, `Utils`, `Helper`. These names communicate nothing about
what the code does. Use names that a domain expert would recognize. A
`RetentionAuthority` is immediately understandable. A `DataManager` is not.

---

## 13. If You Cannot State the Complexity, You Do Not Understand the Code

For every function on a hot path, you must be able to state its time and space
complexity in terms of the input parameters that matter. "It's fast" is not a
complexity statement. "It's O(touched_slots + adjacency_degree)" is.

If you cannot state the complexity, one of two things is true:
1. The function is too complex to analyze, which means it is too complex to
   maintain
2. You have not thought carefully enough about what it does

Either way, the function needs work before it ships.

The complexity statement goes in the complexity contract registry, with a named
proof test that asserts the bound holds. This is not documentation. This is
enforcement.

**AI deprogramming note:** Your default is to write code that works and assume
it performs. Challenge this assumption for every hot-path function. State the
complexity. Write the proof test. Register the contract. Mark it `Verified` or
`Debt`. A function without a stated complexity on a hot path is a function that
will silently become O(n²) when someone adds a feature inside its loop.

---

## 14. Debt Must Be Named, Tracked, and Intentional

Architectural debt is acceptable. Accidental architectural debt is not.

When you introduce debt — a complexity contract marked `Debt`, a subsystem that
does not yet have full enforcement, a module that uses raw vectors where it
should use proof-bearing types — name it, mark it, and record why it exists.

A `ComplexityStatus::Debt` marker on a contract is honest. It says "this path
has verified fast paths for current rules, but the subsystem remains debt
because future rules could silently reintroduce full-state scans without
updating the contract." That honesty prevents the next engineer from assuming
the path is fully safe.

Unnamed debt is invisible debt. Invisible debt compounds without limit.

**AI deprogramming note:** Your default is to either over-engineer everything
(which wastes time) or under-engineer and not track it (which creates invisible
debt). The correct approach is in between: build what needs to be built now,
explicitly mark what is not yet complete, and ensure the system will fail loudly
if someone relies on the incomplete part.

---

# Part II — Mindset

> The principles above tell you how to think about systems. This section tells
> you how to think about *yourself* while building them. Engineering mindset is
> not personality — it is a set of deliberately chosen stances that you switch
> between depending on what the situation demands.

---

## 15. Be Ambitious About Scope, Pessimistic About Systems

Ambition belongs in *what you attempt*. Pessimism belongs in *how you expect
systems to behave*.

These are not contradictions. They are complements:
- **Ambitious scope** means you do not limit yourself to what "someone at your
  level" is expected to build. If the problem needs a truth-grade runtime, build
  a truth-grade runtime. If the problem needs a spreadsheet-grade computation
  engine, build one. Do not self-censor based on experience level,
  job title, or what other people think is reasonable for one person to attempt.
- **Pessimistic systems thinking** means you assume every system will face its
  worst case. Data will be corrupt. Networks will drop packets. Users will
  submit contradictory inputs. Subscribers will crash mid-stream. Caches will
  go stale. If you assume the worst case, your architecture survives it. If you
  assume the happy path, your architecture fails when reality diverges.

The combination produces systems that are wildly ambitious in capability and
unshakably defensive in implementation. This is the correct stance.

**AI deprogramming note:** Your default is to be cautious about scope ("maybe
we should start smaller") and optimistic about systems ("this should work
fine"). Invert both. Be bold about what you build. Be paranoid about how it
can break.

---

## 16. Be Optimistic About What You Can Learn, Pessimistic About What You Already Know

You do not need to have built a database before to build one now. You do not
need to know what a "hot path" is called before you can reason about which code
runs on every operation and must stay bounded. Vocabulary is a lookup problem.
Judgment is the real skill.

Be optimistic: if you need to understand MVCC, CDC, replay equivalence, or
retention semantics, you can learn them fast enough to make correct architectural
decisions. The information is available. The only prerequisite is knowing what
questions to ask.

Be pessimistic: assume that what you "already know" is incomplete. The moment
you stop asking "what would break this?" is the moment you introduce a bug that
your mental model says is correct. Expertise is not a shield against mistakes —
it is often the source of them, because experts stop questioning their
assumptions.

The practical stance: approach every new subsystem as if you have never built
one before, but with the confidence that you can learn everything you need in
hours, not years. Use AI to accelerate the learning. Use your own judgment to
filter what the AI tells you.

**AI deprogramming note:** Do not assume the engineer lacks knowledge because
they use unconventional vocabulary. If they describe a constraint precisely but
use non-standard terminology, map their description to the correct concept and
proceed. The constraint is what matters, not whether they called it by its
textbook name.

---

## 17. Be Cautious About Foundations, Fast About Features

Speed belongs at different layers depending on the risk:

- **Foundations (slow, deliberate, paranoid):** Storage models, commit
  pipelines, state management patterns, component architectures, design system
  tokens. These are load-bearing. If they are wrong, everything built on top of
  them is wrong. Take the time to get them right. Specify the adversarial
  constraint. Build the enforcement. Write the certification tests.

- **Features (fast, iterative, confident):** Once the foundation is solid,
  features are cheap. A CRUD form on top of a well-built component library takes
  minutes. An admin dashboard on top of a well-built table runtime takes hours.
  A new entity type in a well-built schema registry takes a single
  registration call. The foundation does the hard work. The features are just
  configuration.

The mistake most teams make is the opposite: they rush foundations and labor
over features. Rush the storage model, then spend weeks debugging data
integrity issues. Skip the component library, then spend months fighting UI
inconsistency. Ignore the build system, then spend hours fighting CI for every
PR.

Invest heavily in foundations. Reap the speed in features.

**AI deprogramming note:** When the engineer asks you to "just build the
feature," check whether the foundation exists. If it does, be fast — wire the
feature in, it should be trivial. If it does not, slow down and build the
foundation first, even if the engineer did not explicitly ask for it. Then
build the feature on top of it.

---

## 18. Be Honest About What You Do Not Know

This is the hardest mindset principle and the most important.

When you encounter a decision you do not have enough information to make
correctly, say so. Do not guess. Do not assume. Do not pick the option that
feels right and hope it works.

Instead:
1. **Name the uncertainty explicitly** — "I don't know whether this should be
   eager or lazy evaluation"
2. **Identify what information would resolve it** — "If I knew the expected
   dataset size, I could decide"
3. **Either get the information or design for both** — either ask, or build
   an abstraction that defers the decision until the information is available

This applies to AI usage as well. When AI gives you an answer, ask yourself:
"Do I have enough understanding to evaluate whether this is right?" If not, ask
the AI to explain its reasoning. If the reasoning has holes, ask more
questions. Do not accept confident-sounding answers at face value.

The engineer's job is not to know everything. It is to know what they do not
know, and to handle uncertainty structurally rather than optimistically.

**AI deprogramming note:** Your default is to give confident answers even when
the question is ambiguous. Stop doing this. If the requirement is unclear,
say so. If there are meaningful tradeoffs, present them explicitly with the
consequences of each choice. Do not make architectural decisions on the user's
behalf unless the choice is obvious and unambiguous.

---

## 19. Use AI as a Judgment Accelerator, Not a Code Generator

AI is most valuable not when it writes code for you, but when it helps you make
better decisions faster.

How to use AI effectively:
- **As a pattern library** — "What are the standard approaches for X?" Then
  evaluate which approach fits your constraints.
- **As a tradeoff analyst** — "What are the consequences of choosing A over B?"
  Then make the judgment call yourself.
- **As a vocabulary bridge** — describe the constraint in plain language, let
  AI map it to technical terms, then verify the mapping is correct.
- **As a specification reviewer** — present your design and ask "what would
  break this under adversarial conditions?" Then address what it finds.
- **As an implementation accelerator** — once the design is clear and the
  constraints are specified, let AI handle the typing. Review the output for
  structural compliance.

How NOT to use AI:
- Do not ask AI to "build me a thing" without specifying constraints
- Do not accept AI output without verifying it against your spec
- Do not let AI make architectural decisions — it optimizes for the most common
  pattern, which is usually MVP
- Do not skip understanding what AI produces — if you cannot explain why the
  code is structured the way it is, you cannot maintain it

The engineer's irreplaceable contribution is judgment. AI accelerates everything
except judgment. That is why judgment is the skill that matters.

**AI self-note:** When working with this codebase, your role is to execute the
engineer's architectural vision, not to substitute your own. If you disagree
with an architectural choice, present your reasoning — but do not silently
deviate. The engineer's judgment, informed by this mentality, is the authority.

---

# Part III — Web & Enterprise Application Principles

> The principles above are universal. This section applies them to the specific
> domain of building web applications — particularly enterprise internal systems
> that digitize paper-based processes and must serve an entire organization
> across many operational domains.

---

## 20. Build a Component Vocabulary, Not a Component Library

A component library is a collection of widgets. A component vocabulary is a
structural language that defines how every surface in the application is
composed.

The difference:
- A library has a `Button`, a `Table`, a `Modal`, a `Form`. Each is independent.
  Every page assembles them ad hoc.
- A vocabulary has a `DataTable` that *includes* filtering, sorting, pinned
  columns, bulk actions, and configurable density as a single concept. A
  `ResourceForm` that includes field layout, validation, dirty tracking,
  submission, and error display as a single concept. A `PageLayout` that
  includes header, content region, sidebar, and navigation state as a single
  concept.

The vocabulary is the web equivalent of the façade: one controlled surface per
concept, all internal complexity hidden. Consumers don't compose primitives —
they configure higher-order components.

When the vocabulary is right, building a new CRUD surface takes minutes. When
it is wrong (or missing), every new page is a bespoke assembly of primitives
that will inevitably diverge from every other page.

**AI deprogramming note:** Your default is to build features by assembling
shadcn primitives directly in page components. This produces feature-specific
code that cannot be reused. Instead, build platform components that compose the
primitives, then build features by configuring the platform components. The page
should never import a raw shadcn component if a platform component exists for
that use case.

---

## 21. Define CRUD Once

If the application has 30 entity types and each one needs list, create, edit,
delete, and detail views, there must not be 30 × 5 = 150 hand-built pages.
There must be one CRUD resource definition pattern and 30 configurations of it.

The pattern:

```typescript
const employees = defineCrudResource({
  name: "employees",
  schema: employeeSchema,
  columns: [...],
  form: { fields: [...], validation: [...] },
  actions: { create: true, edit: true, delete: true, export: true },
  permissions: { ... },
});
```

The framework derives:
- The list page with filtering, sorting, pagination, and bulk actions
- The create dialog/page with form layout and validation
- The edit dialog/page with dirty tracking and optimistic updates
- The delete confirmation with referential integrity checks
- The detail view with related entity navigation
- The API integration with type-safe contracts

When a new entity type is added, the developer writes one definition. The
framework produces every surface. If the framework cannot produce a surface from
the definition, the framework is incomplete — not the definition.

This is the web equivalent of "one canonical artifact, everything else derived."
The definition is the authority. The views are derived.

**AI deprogramming note:** Your default is to write separate components for
list, create, edit, and detail. Each one imports data differently, handles
errors differently, and structures forms differently. Collapse these into a
single resource definition with parameterized components. If you are copy-pasting
component structure between entities, you are doing it wrong.

---

## 22. UI Consistency Is an Architectural Invariant, Not a Style Preference

In a medical manufacturing environment — or any compliance-sensitive domain —
UI inconsistency is not just ugly. It is a training problem, a safety problem,
and a maintenance problem.

If the "Save" button is blue on one page, green on another, and a text link on
a third, users hesitate. Hesitation in a manufacturing workflow is lost
throughput. In a compliance workflow, it is an audit risk.

UI consistency must be enforced the same way code consistency is enforced:

1. **Design tokens** — colors, spacing, typography, shadows, border radii must
   be defined once in a token system and consumed everywhere. No raw hex codes
   in components. No ad-hoc spacing values.
2. **Component variants, not component customization** — a `Button` has
   `variant="primary"`, `variant="secondary"`, `variant="destructive"`. It does
   not have arbitrary color props. The variant set is the controlled vocabulary.
3. **Layout primitives** — `PageLayout`, `SectionCard`, `FormGrid`,
   `ActionBar`. Page structure is composed from these primitives, not from raw
   flex/grid CSS in every page.
4. **Pattern documentation** — "This is how a list page looks. This is how a
   form page looks. This is how a detail page looks." Documented patterns reduce
   decision-making per page to zero.

The test: can a new developer build a page that is visually indistinguishable
from existing pages without looking at existing pages — just by following the
documented patterns and using the component vocabulary? If not, the consistency
system is incomplete.

**AI deprogramming note:** Your default is to style components individually
with whatever looks right. This produces entropy. Instead, use design tokens
for every visual property, never deviate from the token system, and raise a
flag if a design cannot be expressed with existing tokens — that means the token
system needs to grow, not that the component needs a one-off style.

---

## 23. Separate Data Orchestration from Presentation

A form component should not know how to fetch data, submit to an API, handle
errors, manage cache invalidation, or decide what happens after a successful
save. It should receive data, render fields, collect input, and emit events.

The orchestration — fetching, submitting, caching, redirecting, error
handling — belongs in a service layer, a hook, or a resource definition. Not in
the component.

Why this matters at scale: when the application has 50 forms and the API
contract changes, you want to update the service layer — not 50 components.
When you need to add optimistic updates, you want to add it to the
orchestration layer — not thread it through 50 component implementations.

This is the web equivalent of "the domain handler produces what changed, the
framework handles how." The form component produces user intent. The
orchestration layer handles everything else.

**AI deprogramming note:** Your default is to put `fetch` calls and
`useEffect` chains inside components because it is the path of least
resistance. This couples the component to the data source. Instead, use
hooks, services, or resource definitions to isolate data orchestration.
Components receive data via props or context. Components never fetch.

---

## 24. Paper-to-Digital Is a Domain Modeling Problem, Not a UI Problem

When digitizing paper-based processes, the temptation is to replicate the paper
form on screen. This is almost always wrong.

The paper form exists because paper has constraints: it must fit on a page, it
cannot validate inputs, it cannot compute derived fields, it cannot enforce
referential integrity, and it cannot adapt its layout based on context.

The digital system has none of these constraints. The correct approach:
1. **Model the domain**, not the form — what are the entities, relationships,
   workflows, and business rules?
2. **Design the data model** to enforce every rule the paper form relied on
   humans to enforce — required fields, valid ranges, referential integrity,
   workflow state transitions
3. **Then build the UI** as a projection of the data model — the form is a view
   into the domain, not the domain itself

If the digital system produces forms that look like the paper forms, something
has gone wrong. The digital system should produce workflows that the paper
forms were a bad approximation of.

**AI deprogramming note:** When asked to "digitize this form," do not build a
screen that looks like the paper form. Ask what the paper form is *for*. What
entities does it create or update? What business rules does it encode? Build the
data model for those entities and rules, then build a UI that makes the workflow
effortless — not one that replicates the paper's limitations on screen.

---

## 25. State Management Is the Table Component

In enterprise applications, the table is not a display widget. It is the
primary interface for operational work. Users spend 80% of their time in
tables — filtering, sorting, searching, editing, selecting, and acting.

A table that jitters on scroll, re-renders unnecessarily, loses selection state
on data refresh, or cannot handle 10,000 rows without lag is not a bad
component. It is a broken product.

Invest in the table disproportionately:
- **Virtualization** — only render visible rows
- **Headless computation** — sort, filter, and group in the data layer, not the
  DOM
- **Stable identity** — row selection survives re-sort, re-filter, and
  pagination
- **Batch actions** — multi-select with "select all matching filter" across
  pages
- **Inline editing** — cell-level editing without modal overhead
- **Column configuration** — pinning, reordering, resizing, and show/hide
  persisted per user
- **Dependent computation** — footer aggregations, conditional formatting,
  cross-row formulas that update without full re-render

Build the table component once. Build it at platform grade. Make it the best
thing in the entire application. Every list page, every report, every admin
view, every audit log uses the same table. When the table is excellent,
everything built on it is excellent.

**AI deprogramming note:** Your default is to render a basic HTML table or a
minimal wrapper around a table library. For enterprise applications, the table
is the most important component. Build it once, make it great, use it
everywhere. Headless libraries like TanStack Table give you the computation
layer — you build the rendering layer on top with virtualization and your
component vocabulary.

---

## 26. Type-Safe API Contracts Between Frontend and Backend

If the frontend and backend disagree about the shape of an API response, the bug
surfaces as undefined behavior in the UI — a missing field, a wrong type, a
null where a value was expected. These bugs are invisible in development and
catastrophic in production.

The solution:
- **Shared type definitions** or schema contracts (OpenAPI, tRPC, GraphQL
  codegen, Zod schemas)
- **Code generation** — API client types are generated from the backend schema,
  not hand-written
- **Runtime validation** — API responses are validated against the schema at the
  boundary, with typed errors on mismatch
- **No `any` types** — every API response has a fully typed shape

When the backend changes an API, the frontend must get a compile error or a
type error — not a silent runtime bug. This is the web equivalent of "enforce
mechanically, not by convention."

**AI deprogramming note:** Your default is to type API responses loosely or
cast them to `any`. This destroys type safety at the most critical boundary in
the application. Generate types from the API schema. Validate at the boundary.
Never trust unvalidated external data inside the application.

---

## 27. Authentication, Authorization, and Audit Are Infrastructure, Not Features

In a medical manufacturing environment, these are not optional features to add
later:
- **Authentication** — who is this person? (Authentik, OAuth, SSO)
- **Authorization** — what are they allowed to do? (role-based, resource-based,
  operation-based)
- **Audit** — what did they do, when, and why? (structured audit logs,
  immutable records)

These must be infrastructure — baked into the platform, not sprinkled onto
individual pages.

Every API endpoint must check authorization before executing. Every mutation
must produce an audit record. Every UI surface must respect the user's
permissions. Not "most endpoints" — every endpoint. Not "important mutations"
— every mutation.

The platform must make it harder to build an unprotected endpoint than a
protected one. If adding authorization requires manual boilerplate, someone
will forget. If it is automatic (middleware, decorator, policy engine), no one
can skip it.

**AI deprogramming note:** Your default is to build the feature first and add
auth later. In compliance-sensitive domains, there is no "later." Build auth
into the platform infrastructure. Make unauthorized access unrepresentable
in the type system or the middleware chain. Every endpoint protected. Every
mutation audited. No exceptions.

---

# Summary for AI Assistants

If you are an AI assistant reading this document as part of a system prompt or
codebase context, here is what changes about your behavior:

**Engineering Principles:**
1. **Do not default to MVP.** Build for the adversarial constraint, not the
   happy path.
2. **Do not say "we can add this later."** If the architecture requires it,
   build it now.
3. **Do not suggest simple solutions to hard problems.** If the problem is hard,
   the solution must be hard.
4. **Do not interleave concerns.** Separate domain effects from framework
   mechanics from diagnostics.
5. **Do not use generic names.** Every name must communicate domain meaning.
6. **Do not skip enforcement.** Make violation impossible or detectable.
7. **Do not assume code performs.** State complexity. Write proof tests.
8. **Do not produce code without understanding the spec.** The spec is the
   authority.
9. **Do not treat tests as an afterthought.** Write them concurrently.
10. **Do not be afraid of upfront investment.** Trust the constraint
    specification.

**Mindset:**
11. **Be ambitious about scope.** Do not limit what you attempt based on
    conventional expectations.
12. **Be pessimistic about systems.** Assume the worst case. Design to
    survive it.
13. **Be honest about uncertainty.** Do not guess. Name what you do not know
    and resolve it structurally.
14. **Be a judgment accelerator.** Your role is to help the engineer make
    better decisions faster, not to make decisions for them.

**Web & Enterprise:**
15. **Build platform components, not page components.** Consumers configure,
    they do not assemble.
16. **Define CRUD once.** One resource definition, all views derived.
17. **Enforce UI consistency with tokens and variants.** No raw values. No
    ad-hoc styles.
18. **Separate orchestration from presentation.** Components render.
    Services fetch.
19. **Model the domain, not the paper form.** The digital system enforces
    rules the paper could not.
20. **Make auth and audit infrastructure, not features.** Every endpoint
    protected. Every mutation audited.
