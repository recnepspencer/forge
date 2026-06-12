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

## 14. Debt Is a Blocker Record, Not a Design Strategy

Architectural debt is sometimes unavoidable. Architectural escape hatches are
not a normal design tool.

The default rule is:
1. identify the missing capability or enforcement
2. work backward to the blocker
3. expand scope as far as necessary to remove the blocker, even across crate
   boundaries
4. build the blocker
5. then build the feature on the real path

Scope expansion is the norm here, not the exception. If the real fix lives in a
different crate, then the work expands into that crate. If the real fix needs a
new lower-authority seam, then the work expands until that seam exists. Do not
protect the current milestone or current crate boundary at the expense of
shipping an unfinished architecture.

Do **not** jump from "this is hard" to "mark it as debt" or "add an escape
hatch." The presence of pressure, uncertainty, or implementation cost is not a
reason to widen the API, add a compatibility shortcut, expose a raw seam, or
document an unfinished public lane as acceptable.

Debt is allowed only when all of the following are true:
1. there is a specific blocker you can name precisely
2. removing the blocker would require a genuinely major follow-on build rather
   than an ordinary scope expansion
3. the real blocker fix is large enough that it would exceed roughly 3000 lines
   of code in the current change program, even if that work belongs in another
   crate
4. the incomplete path is mechanically contained so callers cannot mistake it
   for the finished ordinary lane
5. the debt is attached to an explicit owner and follow-on milestone
6. the tests and support surfaces make the incompleteness obvious

If those conditions are not true, the correct move is not to mark debt. The
correct move is to expand scope and keep building until the blocker is gone.

The 3000-line threshold is not a budgeting suggestion. It is the bar for what
counts as "major enough to defer." Anything meaningfully smaller than that is
ordinary completion work and should be built now.

A `ComplexityStatus::Debt` marker on a contract is honest only when it records
a real blocked edge after the team has already built the strongest complete path
available in the current milestone. It is not permission to stop early, ship a
half-product, or replace a missing foundation with a permanent-looking shortcut.

Unnamed debt is invisible debt. Named-but-unnecessary debt is surrender dressed
up as rigor. Avoid both.

**AI deprogramming note:** Your default is to either over-engineer everything
(which wastes time) or under-engineer and then "be honest about it." That is
still wrong when the missing piece can be built now. The correct approach is:
expand scope, even across crates, build the real blocker, and only mark debt
when the blocker fix is so large that it crosses the major-work threshold of
roughly 3000 lines of code. If you find yourself writing "for now,"
"temporary," "fallback," "escape hatch," "explicit debt," or "deferred
neighbor," stop and ask whether you should be widening the scope and building
the missing foundation instead.

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
3. **Either get the information or build the prerequisite** — either ask, or
   build the missing measurement, proof surface, or lower-authority seam that
   makes the decision decidable

This applies to AI usage as well. When AI gives you an answer, ask yourself:
"Do I have enough understanding to evaluate whether this is right?" If not, ask
the AI to explain its reasoning. If the reasoning has holes, ask more
questions. Do not accept confident-sounding answers at face value.

The engineer's job is not to know everything. It is to know what they do not
know, and to handle uncertainty structurally rather than optimistically.

**AI deprogramming note:** Your default is to give confident answers even when
the question is ambiguous, then compensate by adding flexibility or escape
hatches. Stop doing both. If the requirement is unclear, say so. If the system
is missing prerequisite information, measurements, or authority seams, build
them or ask for what you cannot discover. Do not respond to uncertainty by
making the architecture looser than it should be.

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

