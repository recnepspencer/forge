
CONFIDENTIAL

FORGE
The Geometry Engine for the Age of Autonomous Manufacturing

Product Architecture & Business Overview
February 2026

This document contains proprietary information. Distribution without written consent is prohibited.
  
Executive Summary
The $15 billion CAD industry runs on geometry kernels designed in the 1980s. Every major product — SolidWorks, Revit, Inventor, NX — sits on top of either Parasolid or ACIS, both written in C++ with architectures that predate the internet. These kernels were built for humans clicking buttons. They are fundamentally incapable of serving the next wave: AI agents that design, validate, and manufacture physical objects autonomously.
Forge is a ground-up geometry kernel and manufacturing API built in Rust, designed from first principles for two simultaneous audiences: human engineers who deserve software that never crashes on a fillet operation, and AI agents that need a token-efficient programmatic interface to manufacturable geometry.
The core technical innovations — a reactive dependency engine inspired by modern frontend signal architectures, exact arithmetic that eliminates floating-point failures, and a topology-first computation model that separates combinatorial correctness from geometric approximation — produce a kernel that is more correct, more performant, and two orders of magnitude smaller than incumbent solutions.
But the real disruption is not the kernel. It is the business model. Forge monetizes geometry operations as an API, capturing value from every AI agent that designs a part, every robotic cell that plans an assembly, and every on-demand manufacturer that validates a model — at a fraction of a cent per operation, at volumes that dwarf human CAD usage by orders of magnitude.
We are not building a better CAD application. We are building the geometry layer of autonomous manufacturing.
 The Problem
Broken by Design
Every mechanical engineer and architect has experienced it: a fillet operation fails with a cryptic error. A parametric rebuild produces a red feature tree with no explanation. A Boolean operation between two seemingly simple solids crashes the application. Importing a STEP file from a supplier produces garbage geometry. These are not rare edge cases. They are daily frustrations that cost the manufacturing industry billions of hours annually.
The root cause is architectural, not algorithmic. Incumbent geometry kernels store vertex coordinates as 64-bit floating-point numbers, compute intersections between surfaces to produce new coordinates, and then make discrete topological decisions based on those computed coordinates. Rounding errors in the intersection propagate into classification errors, which produce topological corruption, which causes the operation to fail. Adding tolerance parameters creates new failure modes. The complexity is irreducible within this architecture.
Designed for Humans, Hostile to Agents
The emerging wave of AI-driven manufacturing exposes an even deeper problem. Today’s CAD APIs were designed for human-driven macro automation — verbose, stateful, order-dependent, requiring a running GUI process. Driving SolidWorks programmatically through its COM API to create a simple bracket requires approximately 2,000 tokens of instructions and seconds of execution time. For an AI agent making thousands of geometry decisions per manufacturing job, this is economically and architecturally untenable.
There is no clean, performant, token-efficient API that bridges the gap between an agent’s intent (“I need a mounting bracket with these constraints”) and a manufacturable solid model with tolerances. This gap is the single largest bottleneck in autonomous manufacturing.
The Market Failure
The incumbents cannot solve these problems. Autodesk’s Revit core has resisted modernization for over a decade because the architecture is too deeply entangled. Dassault’s 3DEXPERIENCE platform is a UI layer over the same Parasolid kernel. Siemens owns Parasolid and has no incentive to disrupt their licensing revenue. The entire industry is trapped in a local optimum — too invested in C++ codebases from the 1990s to make the fundamental architectural changes required.
 The Solution: Forge Architecture
Forge is built from first principles around four architectural pillars that together eliminate the failure modes of traditional CAD kernels while enabling AI-native interaction.
Pillar 1: Plane-Based Exact Predicates
Traditional kernels represent vertices as stored coordinates. Forge inverts this: planes are the primary geometric primitives, and vertices are defined implicitly as the intersection of three or more planes. A vertex’s position is never stored — it is derived when needed.
This inversion eliminates cascading floating-point error. Every topological decision during a Boolean operation reduces to evaluating the sign of a 4×4 determinant of original plane coefficients. Since these coefficients are source data — never the output of a previous computation — error does not accumulate. The geometric “truth” flows through the pipeline untouched.
For performance, Forge uses a three-stage filtered predicate system. Stage 1 evaluates the determinant in standard 64-bit floating-point with a computed error bound; if the result exceeds the bound, the sign is correct in approximately 5 nanoseconds. Stage 2 uses compensated arithmetic to resolve 99%+ of remaining cases in approximately 50 nanoseconds. Stage 3 falls back to exact rational arithmetic for the rare cases near zero, at approximately 500 nanoseconds. The amortized cost is effectively that of the fast path.
Unlike legacy kernels that use symbolic perturbation (Simulation of Simplicity) to "nudge" away from zeros, Forge treats TriSign::Zero as a first-class citizen. Coincident geometry — flush faces, shared edges, and merged vertices — is handled explicitly through an Atomic Coincidence Graph. This produces a "clean" topological model matching design intent, entirely eliminating the micro-sliver faces and unpredictable edge classification that plague Parasolid-based workflows.
Pillar 2: Topology-Geometry Separation
Forge completely decouples topological structure from geometric embedding. The topological complex — which faces exist, how edges connect them, what the incidence structure is — is a pure combinatorial data structure computed using exact predicates. The geometric binding — where those faces, edges, and vertices are located in space — is computed separately as a best-effort numerical embedding of the exact topology.
This separation has profound consequences. Boolean operations are executed in two phases: a topological phase that uses exact predicates to determine the combinatorial result (guaranteed correct), followed by a geometric phase that computes intersection curves and vertex positions numerically (approximate but refinable). If the geometry is slightly imprecise, the topology is still correct. The system can iteratively refine geometry without risking topological changes.
For NURBS surfaces — the hard case in mechanical CAD — topological classification uses interval arithmetic on surface subdivision rather than explicit surface-surface intersection. The surface is subdivided until interval arithmetic resolves the sign of the classification predicate, a process that converges geometrically fast and is naturally parallelizable.
Pillar 3: Reactive Signal Dependency Engine
This is Forge’s most architecturally distinctive component. Inspired by Angular’s signal system, every computable value in the system — a sketch solution, a face’s surface equation, a topological classification, a fillet’s geometry — is a reactive signal node in a fine-grained dependency graph.
Push-Pull Hybrid Evaluation
When a parameter changes, the engine pushes a “dirty” notification through the dependency graph in O(edges) time. But actual recomputation is pulled lazily — only when a downstream consumer reads the value. Signals that are off-screen, not currently needed for the active operation, or behind a topology change firewall are never recomputed.
Three-State Invalidation
Forge extends the standard clean/dirty model with a critical intermediate state: MaybeStale. When a signal’s dependency’s dependency changes, the signal is marked MaybeStale rather than Dirty. On read, the engine walks up the graph and checks version counters. If the direct dependency didn’t actually change its value (as determined by structural hashing for topology signals), the MaybeStale signal reverts to Clean without any recomputation.
This mechanism implements what we call a topology change firewall. The vast majority of interactive parameter edits — dragging a dimension, tweaking a fillet radius — do not change the model’s topological structure. With the three-state system, these edits only propagate geometry updates through the graph. The topology signals, and everything that depends only on topology (selectors, feature references, assembly mates), remain untouched.
Multi-Granularity Signals
Each feature’s output signal carries separable aspects with independent version counters: topological structure and geometric embedding. A downstream signal can subscribe to only the aspect it requires. A fillet’s edge selector depends on topology only; a rendering mesh depends on geometry only; a mass property depends on both. Dirty notifications propagate only along matching aspect edges, minimizing unnecessary recomputation.
Automatic Dependency Discovery
Dependencies are not declared manually. The engine discovers them at evaluation time by tracking which signals are read during each computation, identical to Angular’s model. This means dependencies are dynamic — a selector might depend on different edges after a topology change, and the graph rewires itself automatically. There are no stale dependency declarations.
Why This Matters for Performance
In SolidWorks, changing a dimension in feature 5 of a 200-feature model triggers a complete sequential rebuild from feature 5 forward. In Forge, the same change propagates through the dependency graph, hits multiple topology change firewalls (where the topology didn’t actually change), and typically recomputes only 3–5 geometry signals rather than 195 feature rebuilds. Interactive edits that take 5–30 seconds in SolidWorks complete in under 50 milliseconds in Forge.
Pillar 4: Declarative Specification Graph
The user’s model is not a B-rep. It is a declarative specification graph — a directed acyclic graph of features, constraints, and intentions. This graph is the source of truth. B-rep geometry is a derived, cached, lazily-computed projection of the specification graph.
Features reference topology through composable selector queries, not fragile entity IDs. A fillet does not reference “edge #37.” It references “the edges where the face from Extrude-3 meets the face from Extrude-1.” When upstream geometry changes and the topology shifts, the selector re-evaluates against the new topology. If the query matches, the feature updates silently. If the query becomes ambiguous, the user sees a disambiguation prompt — not a crash.
The specification graph also enables Forge’s three concurrent representations of the model: an SDF (Signed Distance Field) evaluation for real-time preview at 60fps, the topological complex with exact predicates for feature operations, and a full B-rep with NURBS geometry for manufacturing export. The user sees the SDF preview updating in real time as they drag. The B-rep materializes in the background. If B-rep export encounters a hard case, the model remains fully interactive — the user never loses their work.
 Technical Moat: Radical Simplicity
Forge’s architecture, combined with Rust’s expressive type system, produces a kernel that is two orders of magnitude smaller than incumbent solutions. This is not a limitation — it is a structural advantage that compounds over time.
Component	Open CASCADE (C++)	Forge (Rust)
Geometry kernel	~1,500,000 lines	~33,000 lines
Full CAD application	~800,000 lines (FreeCAD)	~333,000 lines
Incumbent comparison	~20,000,000 lines (SolidWorks)	~333,000 lines

This compression comes from four sources: algebraic types that replace visitor patterns and factory classes with 5-line match blocks; generic predicates written once and monomorphized at three precision levels; the reactive signal engine that eliminates thousands of lines of manual dependency wiring via a derive macro; and the topology-geometry separation that removes entire categories of tolerance-management code.
Smaller codebases are easier to audit, easier to test, and easier to improve with AI assistance. Every line of code is a potential bug. Forge’s 60x code reduction relative to SolidWorks is a 60x reduction in bug surface area.
 Validation: Differential Testing Engine
The hardest problem in building a new geometry kernel is not implementation — it is validation. There is no formal specification for what a Boolean operation should produce on two complex NURBS solids with near-tangent intersections. The “spec” is the accumulated behavior of Parasolid and ACIS over three decades.
Forge attacks this with a differential testing architecture that uses existing industry kernels as a reference oracle. Every modeling operation is executed simultaneously in Forge and in Open CASCADE (an open-source kernel widely used in industry). The results are compared volumetrically — not by B-rep topology, which can validly differ, but by sampling hundreds of thousands of points in space and verifying that both kernels agree on whether each point is inside or outside the solid.
Volumetric Comparison
Point-in-solid classification is the ground truth for geometric correctness. Two solids that agree on point classification everywhere are geometrically identical regardless of their internal B-rep structure. Forge samples points on a uniform grid, then adaptively refines around any mismatches. Mismatches are classified by pattern: clustered along a surface (minor approximation difference, usually acceptable), entire region flipped (topological bug, high priority), thin sliver region (near-degenerate geometry, investigate), or scattered (boundary classification noise, usually harmless).
Automated Fuzzing at Scale
Forge generates millions of synthetic test cases using domain-specific generators for architectural geometry, mechanical parts, sheet metal, and organic surfaces. Each case executes in both kernels. Results are compared, failures are classified, and regressions are detected automatically. This runs continuously in CI — every code change is validated against the full test corpus.
Parametric Stability Testing
Beyond single-operation correctness, the testing engine validates incremental update behavior. A base model is built, then parameters are changed. The engine verifies that Forge’s incremental result (via the signal graph) matches both a full rebuild from scratch and the reference kernel’s result. This catches the subtlest category of bugs: reactive graph propagation errors where incremental evaluation diverges from clean evaluation.
The testing corpus grows continuously and represents a compounding moat. The longer Forge runs, the more edge cases are cataloged and resolved, and no competitor starting from scratch has access to this labeled dataset.
 The API-First Business Model
The Insight
The traditional CAD business model is selling seats to human engineers at $50–200/month. A human designer produces perhaps 50 parts per month. This is a linear, labor-constrained revenue model.
AI agents operate differently. An agent runs 24/7, makes geometry decisions in milliseconds, and can produce 50,000 parts per month. The value is not in a seat — it is in operations. Forge monetizes at the operation level: a fraction of a cent per geometric operation, at volumes that dwarf human usage by three orders of magnitude.
API Design for Agent Efficiency
Forge’s API is designed from the ground up for minimal token cost. Where SolidWorks’ COM API requires approximately 2,000 tokens and multiple seconds to create a simple bracket, Forge’s declarative API achieves the same result in approximately 200 tokens and milliseconds of execution. For complex models, Forge also supports an intent-level API where the agent specifies functional requirements and Forge’s kernel resolves the geometry:
Agent specifies: “mounting bracket, M8 bolt pattern, 500N load, CNC-machinable aluminum.” Forge produces: a fully toleranced, manufacturable STEP file. Token cost: ~100 tokens. Execution time: <1 second.
The agent never needs to “see” the model. It queries geometric properties — volume, clearance, manufacturability, center of mass — as structured data. Changes are atomic and validated. The reactive engine ensures consistency. This is what API-first means: the core is an API, and the GUI is one client among many.
Revenue Comparison
Metric	Traditional CAD Seat	Forge API (per agent)
Parts per month	~50 (human speed)	~50,000 (24/7 automated)
Operations per part	~20	~20
Price per operation	N/A (seat-based)	$0.01–$0.05
Monthly revenue	$200/seat	$10,000–$50,000/agent
Scaling constraint	Headcount	Compute

A single large manufacturer running hundreds of agents represents potential revenue in the millions per month from API usage alone. This is the Twilio model applied to manufacturing geometry: low per-unit price, astronomical volume, usage-based growth that scales with the customer’s automation, not their headcount.
 The Human-Facing Application
While the API is the primary revenue engine, Forge also ships a full parametric CAD application. This serves three strategic purposes: it validates the kernel against real-world workflows, it generates revenue from the existing market of human engineers, and it ensures that Forge controls the end-to-end experience rather than depending on third-party applications built on the API.
Coverage Model
SolidWorks has thousands of features. Most users use a fraction of them. Forge targets 95% user coverage with approximately 60–70% feature coverage, prioritized by actual usage data:
Tier 1: Core Product (60% of users)
Part modeling (extrude, cut, revolve, fillet, chamfer, pattern, mirror, shell), basic assemblies with mates, full 2D drawing production with ASME Y14.5 tolerancing, STEP/IGES/PDF export. This is the mechanical designer at a small-to-mid-size shop — the largest segment of the market.
Tier 2: Growth Product (85% of users)
Sheet metal with flat pattern unfold, configurations and design tables, complex assemblies (500+ parts), weldments, basic surfacing, bill of materials, rendering for marketing materials, and revision management.
Tier 3: Enterprise Product (95% of users)
Advanced surfacing, mold design, FEA integration, motion simulation, routing (piping and electrical), tolerance stack analysis, Python scripting API, and PDM integration.
Application Scale
Tier	Estimated LOC	Cumulative LOC	User Coverage
Kernel + infrastructure	~85,000	85,000	—
Tier 1: Core product	~102,000	187,000	60%
Tier 2: Growth product	~71,000	258,000	85%
Tier 3: Enterprise product	~75,000	333,000	95%

For context, SolidWorks is estimated at approximately 20 million lines. Forge achieves 95% user coverage in approximately 333,000 lines — a 60x reduction enabled by the architectural choices described above and by Rust’s expressiveness relative to C++.
 Market Opportunity
Existing Market
Segment	Annual Revenue	Key Incumbents
MCAD (mechanical CAD)	$10–12B	SolidWorks, NX, Creo, Inventor, Fusion
AEC (architecture & BIM)	$5–6B	Revit, ArchiCAD, Bentley
Kernel licensing	$300–500M	Parasolid (Siemens), ACIS (Spatial)
Total addressable (existing)	$15–20B	

Emerging Market: Geometry-as-a-Service
The market for programmatic geometry APIs serving AI agents and robotic systems does not fully exist yet. It is being created by three converging trends:
•	Robotic manipulation maturity:  Commodity robotic arms with adequate dexterity will be available within 2–3 years from Boston Dynamics, Figure, and Chinese manufacturers. They lack the ability to understand and generate the geometry of what they build.
•	AI agent reasoning about physical objects:  Large language models can describe mechanical assemblies and reason about spatial relationships. They cannot produce precise, manufacturable geometry. There is no API bridging intent to STEP files.
•	On-demand manufacturing scale:  Xometry, Protolabs, and 3D printing farms accept STEP files and return parts in days. The entire pipeline from intent to physical part can be automated if the geometry generation step is solved.

Forge sits at the exact intersection of these trends. It is the missing piece: a performant, token-efficient, correct-by-construction geometry API that lets agents go from intent to manufacturable solid in milliseconds.
We estimate the geometry-as-a-service market at $1–5B within five years as autonomous manufacturing scales, with Forge positioned as the foundational infrastructure layer.
 Development Strategy
AI-Augmented Development
Forge is built using an AI-first development methodology. A single technical founder directs AI coding agents to implement modules from architectural specifications. The architecture described in this document — exact predicates, topology-geometry separation, reactive signals, declarative specification graph — was deliberately designed to be AI-implementable: well-specified, modular, testable, and composed of components with clear inputs, outputs, and invariants.
AI agents currently perform well on approximately 80% of the codebase: exact arithmetic (well-defined math with reference implementations), the reactive engine (well-understood CS with clean reference designs in SolidJS and Angular), file format parsers (spec-driven, tedious but straightforward), test infrastructure, and UI boilerplate. The remaining 20% — architectural decisions, geometric edge case triage, UX taste, and domain expertise — requires human judgment. Critically, this ratio is shifting: AI capability improves quarterly, and each quarter expands what can be delegated.
Development Roadmap
Phase	Timeline	Deliverable	Revenue
1: Kernel	Months 0–10	Working kernel + basic modeler + test harness	$0 (development)
2: API launch	Months 10–16	Public geometry API with documentation	$10–50K/mo (early adopters)
3: Tier 1 CAD	Months 14–22	Shippable CAD application (60% user coverage)	API growing + $200/seat
4: Tier 2 CAD	Months 20–28	Growth product (85% user coverage)	Combined $200K–500K/mo
5: Platform	Months 28–36	Third-party ecosystem, Tier 3 features	Platform economics

The key strategic sequencing: the API ships before the CAD application. This validates the kernel against real workloads, generates revenue, and battle-tests the geometry engine before the human-facing product depends on it. The CAD application is a client of Forge’s own API, guaranteeing that the API is production-quality.
AI Capability Compounding
The development timeline accounts for improving AI capability over the project’s duration. The hardest, most architecturally sensitive code (the kernel) is built first, when human oversight is most needed. The voluminous but well-specified code (file format importers, UI components, content libraries, drafting details) is built later, when AI agents are better at exactly that class of work. Estimated acceleration from AI improvement over the development period: approximately 30% timeline compression, weighted toward later phases.
 Competitive Positioning
Why Incumbents Cannot Respond
•	Architectural lock-in:  Parasolid and ACIS fuse topology and geometry in their core data structures. Retrofitting topology-geometry separation or reactive dependency tracking would require rewriting the kernel, which would break every application built on it. This is not a feature gap — it is a structural impossibility.
•	File format and API compatibility:  SolidWorks’ entire plugin ecosystem, file format, and automation API depend on the sequential feature tree model. Switching to a reactive specification graph would break every third-party integration. The ecosystem is the moat, and it’s also the prison.
•	Organizational inertia:  Autodesk has attempted to modernize Revit’s core for over a decade without success. The team that maintains a 20-million-line C++ codebase cannot simultaneously rewrite it from scratch. New architecture requires a new organization.

Comparison with Adjacent Competitors
Company	Approach	Forge Advantage
OnShape	Modern cloud CAD, but uses Parasolid kernel	Forge has a fundamentally better kernel; OnShape cannot fix kernel-level failures
nTopology	Implicit/SDF modeling for lattices	Narrow use case (lattices/infill); no parametric CAD, no B-rep export quality
Shapr3D	iPad-native direct modeling	No parametric feature tree; targets hobbyists, not production engineering
FreeCAD	Open-source, Open CASCADE kernel	Same kernel limitations as commercial tools; volunteer development pace
Bricscad	DWG-compatible CAD + BIM	Legacy architecture; competing on Autodesk compatibility, not innovation

Forge’s Compounding Moats
•	Differential testing corpus:  Grows continuously. Every edge case cataloged and resolved is a data asset no competitor can replicate without years of equivalent testing.
•	Kernel architecture:  The topology-geometry separation, exact predicates, and reactive signal engine are not individual features — they are mutually reinforcing architectural decisions. Copying one without the others provides limited benefit.
•	AI-native API surface:  As AI-driven manufacturing scales, every agent integration built on Forge’s API creates switching costs. The API becomes a standard, not just a product.
•	Development velocity:  The 60x code reduction means Forge can iterate faster than incumbents. A bug fix or feature addition that takes a team of 10 engineers a quarter at SolidWorks takes a single developer with AI agents a week at Forge.
 Revenue Model
Three Revenue Streams
Stream 1: Geometry API (Usage-Based)
Per-operation pricing at $0.01–$0.05 per geometric operation (Boolean, fillet, export, manufacturability check). Target customers: robotics companies, manufacturing automation startups, AI agent developers, on-demand manufacturing platforms. Revenue scales with customer automation, not headcount.
Stream 2: CAD Application (Subscription)
Professional seat licenses at $100–$200/month, competitive with SolidWorks. Target customers: mechanical design shops, manufacturing firms, product development teams. Recurring revenue with enterprise expansion dynamics.
Stream 3: Platform (Revenue Share)
Third-party developers build domain-specific tools on Forge’s API. Forge takes a percentage of transaction value for marketplace distribution and API usage. This emerges in later phases as the ecosystem matures.
Projected Revenue Trajectory
Year	API Revenue	CAD Revenue	Total ARR
Year 1	$0	$0	$0 (development)
Year 2	$200K–$600K	$100K–$300K	$300K–$900K
Year 3	$2M–$8M	$1M–$4M	$3M–$12M
Year 4	$10M–$30M	$5M–$15M	$15M–$45M
Year 5	$30M–$100M	$15M–$40M	$45M–$140M

Year 5 projections assume autonomous manufacturing adoption reaches early mainstream. The range reflects uncertainty in adoption timing, not in product capability. The API revenue line has significantly higher upside variance than the CAD line, as it scales with the pace of manufacturing automation industry-wide.
Valuation Benchmarks
Milestone	Likely Valuation Range
Working kernel + demo, pre-revenue	$10–$50M
Vertical app, 1,000 paying seats	$100–$300M
Vertical app, 10,000 paying seats, growing fast	$500M–$2B
Platform with multiple apps on kernel	$2–$5B
Category winner displacing an incumbent	$5–$20B

For reference, OnShape was acquired by PTC for $470M with a modern cloud CAD product but a traditional kernel. Autodesk’s market capitalization is approximately $55B. Capturing 10% of Autodesk’s market with a fundamentally better architecture represents a $5B+ outcome.
 Risks and Mitigations
Risk	Severity	Mitigation
Kernel correctness on real-world geometry	High	Differential testing engine with automated fuzzing against Open CASCADE. Continuous regression testing against 10,000+ real STEP files.
2D drafting quality insufficient for manufacturing	High	Dedicated focus on ASME Y14.5 compliance from Tier 1. Early partnership with machine shops for validation feedback.
Autonomous manufacturing adoption slower than projected	Medium	CAD application provides revenue from existing human-engineer market. API business accelerates when market is ready.
NURBS surface-surface intersection robustness	Medium	Topology-geometry separation means intersection imprecision does not cause topological corruption. Iterative refinement converges to required tolerance.
Ecosystem switching costs (file formats, plugins, content libraries)	Medium	STEP/IGES import from day one. Open plugin API. Bootstrap content libraries from open sources (TraceParts, BOLTS). Compatibility is a feature, not an afterthought.
Single-founder key-person risk	Medium	Architecture documented extensively. Codebase is 60x smaller than incumbents and written in modern Rust — significantly easier to onboard new engineers. AI-augmented development reduces bus factor.
 Conclusion
The CAD industry is at an inflection point. The geometry kernels that have powered engineering software for three decades cannot serve the emerging world of AI-driven design and autonomous manufacturing. They are too fragile, too slow, too verbose, and too architecturally constrained to evolve.
Forge is not an incremental improvement. It is a ground-up rethinking of how geometry computation should work, built on principles — exact arithmetic, topological correctness, reactive dependencies, declarative specification — that eliminate the failure modes of traditional kernels by construction rather than mitigation.
The business model matches the architecture: API-first, usage-based, scaling with the automation of manufacturing rather than with human headcount. As AI agents become the primary consumers of geometry operations, the company that provides the fastest, most correct, most token-efficient geometry API captures a toll on every manufactured object designed by machine.
The convergence of improving AI development capability, maturing robotic manipulation, and scaling on-demand manufacturing creates a window for a new geometry standard. Forge is positioned to be that standard.

FORGE
Building the geometry layer of autonomous manufacturing.
