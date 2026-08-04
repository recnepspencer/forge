# Milestone 9.16 Runtime Phase 7 Closure Ledger

**Owner:** Runtime Hardening Track, Phase 7
**Canonical specification:** `milestone-9.16.md`
**Status:** Phase 7.3 closed; in progress at Phase 7.4 — Milestone 9.16.1 is
closed and its managed graph-work session handoff remains the Phase 7 substrate
**Policy:** A requirement is `PROVED` only when its production owner, public
consumer evidence, adversarial evidence, performance posture, and residue
posture agree. A finding is `CLOSED` only when the root cause and every
causally dependent guarantee have been rechecked.

This ledger is the durable meaning of the `R7.*` and `Q7.*` identifiers used by
the milestone. A green broad test does not change a row's status. New findings
that require stronger composition receive an append-only corrective phase or
milestone and block unfinished dependents; they do not relabel completed rows.

Milestone 9.16.1 added the canonical graph-obligation and provider-session
prerequisite discovered after Phase 7.2. Its separate closure ledger is closed.
The `R7.*` and `Q7.*` statuses below remain the historical Phase 7 record while
Phase 7.3 resumes from that exact handoff. The prerequisite migrated only the
named competing authority surfaces with downstream parity; it preserved the
Phase 6 feature contracts and the proved 7.1-7.2 lower-owner meaning. Phase 7.3
receives the same application-query identities, lane semantics, and public
front door with a single session-bound graph authority, not a replacement
Query product.

## Requirement ledger

| ID | Gate | Guarantee | Status | Required closure evidence |
|---|---:|---|---|---|
| R7.1 | 7.1 | Capability identity is typed and distinct from role, relationship, authentication, policy result, operation authority, and runtime proof. | **PROVED** | Typestate-complete contracts, typed category and context-slot compiler denials, descriptive-capability denial, installed-authority root/runtime/generation/operation hostility, and Bank denial of premature execution authority. |
| R7.2 | 7.1 | Action, resource, relation, field, purpose, amount, cardinality, workflow stage, validity timeline, active grant status, delegation, provenance, and constrained context are explicit identity-bearing dimensions; required-versus-optional field presence is preserved without sentinels; grant workflow is aligned to an explicit resource-side current field. | **PROVED** | Typed declaration and macro surfaces carry field presence into canonical schema identity and exact Foundational lowering; missing optional, missing required, and wrong present-value types are independently exercised; all 17 honest Bank contracts install without sentinels and retain their validity, status, and workflow bindings. |
| R7.3 | 7.1 | Allow, deny, conflict, separation-of-duty, distinct-actor, delegation, and disclosure composition is installed canonical application meaning. | **PROVED** | The full declaration/installation and Bank suites pass against the repaired currentness basis; all 17 contracts retain their exact generic composition and actor anchors. |
| R7.4 | 7.1 | Capability meaning is prepared once through Foundational canonical basis and typed sequence digest, while Query installation supplies the stronger authority binding. | **PROVED** | Structured comparison, digest identity, public compiler hostility, and installed authority hostility all pass after field presence became part of the bound schema identity. |
| R7.5 | 7.1 | Capability installation has bounded contract count and canonical bytes; warm installed-capability lookup performs zero basis preparation, hashing, or digest text materialization. | **PROVED** | Exact contract and canonical-byte ceilings plus 4,096 retained lookups were rerun after the identity change; the Bank population-scale admission proof remains exact-zero for every canonical/SHA/text counter. |
| R7.6 | 7.2 | Explicit purpose and constrained request context become attempt-bound access authority from current relational truth and installed contracts. | **PROVED** | The public Bank request derives every varying dimension from the exact owned `EstateAction`; six consumer tests cover current admission, one-axis request binding, runtime affinity, and scale. Query retains the resolved resource, exact grant witness, trusted time sample, purpose, context, principal currentness, and policy decision in a move-only access proof. |
| R7.7 | 7.2 | Exact authorization decision facts and read sets can be retained and revalidated at a later snapshot without rebuilding policy meaning, substituting an equivalent grant/path, or granting reusable authority. | **PROVED** | Principal and policy freshness are typed separately, provider fact cardinality is installed and exact, equivalent grant and composition-path replacement deny, current time is resampled, and every commit/idempotency token is bound to the actual admission. The complete authorization source cone proves zero warm canonical/SHA/text work. Full lane consumption remains R7.14. |
| R7.8 | 7.3 | Internal computation authority is distinct from consumer disclosure authority and both narrow Foundational projection/diagnostic masks. | **PROVED** | Foundational rejects diagnostic-as-projection substitution; Query rejects a selector whose real projection mask exceeds the installed `AspectContract`; internal influence denial has positive/negative predicate, guard, ordering, live-scope, and live-target twins; the canonical protected-label pair is equal across one-shot, continuation, history, preview, and live observables. |
| R7.9 | 7.3 | Publication accepts only typed disclosed-or-omitted results and cannot inspect protected values or decide policy. | **PROVED** | Execution denies consumer construction of the admitted result; publication rejects raw values plus projection and diagnostic masks; the Bank public consumer publishes only the admitted shape; certification proves no protected-value/policy token lane in publication. |
| R7.10 | 7.4 | Delegation is a proof-carrying narrowing transition with exact lineage and current parent re-admission. | **OPEN** | Every dimension widening, copied-parent, depth/width, expiry, and revocation attacks. |
| R7.11 | 7.5 | Allow, deny, conflict, separation-of-duty, and distinct-actor rules form one installed decision over exact actor and touched-graph facts. | **BLOCKED BY 7.4** | Contract-derived interaction matrix and hostile drift sequences without Cartesian expansion. |
| R7.12 | 7.6 | Emergency elevation is a linear installed typestate with a bounded upper authority and mandatory distinct-actor review. | **BLOCKED BY 7.5** | Lawful lifecycle plus self-approval, conflict, widening, expiry, revocation, copying, and repeated-review denial. |
| R7.13 | 7.7 | Bank estate operations use the public Query progression with no bank-local authority executor. | **BLOCKED BY 7.6** | Complete estate courtroom, consumer transcript, dependency direction, and legacy residue. |
| R7.14 | 7.7 | Every supported query lane re-admits identical capability, purpose, disclosure, and conflict meaning without changing query identity or result meaning. | **BLOCKED BY 7.6** | One-shot, continuation, history, preview, and live consumer evidence. |
| R7.15 | 7.7 | Warm authorization work depends only on declared touched evidence, not unrelated grants, relationships, fields, cases, rows, or consumers. | **BLOCKED BY 7.6** | Growth measurements with exact-zero canonical preparation, hashing, digest text, and fallback. |
| R7.16 | 7.2 | Relational owns current graph observation, Signal owns installed boolean composition, Runtime Bridge owns correspondence, and Query alone combines those proofs into attempt authority; no layer silently recreates another layer's decision. | **PROVED** | Relational's 11 authorization tests prove neutral complete witnesses; installed nested composition lowers through Signal and Bridge; prohibited-path precedence remains Bridge-owned; only top-level Query authorization combines current evidence into private move-only access/operation authority. No competing production authorization owner or legacy import remains. |
| R7.17 | 7.2 | Capability plans compile at the cold application-runtime publication seam; warm admission performs typed lookup plus touched-graph work and cannot recompile or rehash installed meaning as unrelated graph population grows. | **PROVED** | Publication compiles and indexes each plan once; 4,096 retained installation lookups and Bank population-scale admission remain one registry probe with exact-zero basis, digest, SHA, and text work. The recursive source-cone oracle includes the complete authorization owner. |
| R7.18 | 7.3 | Internal-computation and disclosure decisions are minted only after the canonical managed graph-work session exists and are affine to its runtime, query, parameters, principal, scope, session, managed run, branch, basis, and provider. | **PROVED** | Admission constructs governance only after `start_query`; the private binding compares every named affinity. Real plans prove honest matching plus fresh-session, foreign-runtime, installed-query, admitted-parameter, second-principal, and second-scope denial. Inherited 9.16.1 provider-session branch/basis/provider hostility and compiler sealing cover axes that cannot be independently counterfeited. Every readmission consumes pending governance and mints a fresh session binding. |
| R7.19 | 7.3 | Projection and diagnostic masks remain category-distinct, AspectContract-admitted artifacts inside Query's private decision; neither a descriptive mask nor a diagnostic mask can open an internal read, result projection, or publication boundary. | **PROVED** | The private admitted field retains both typed masks; ordinary reads can obtain only exact projection-field admission. Foundational category compile-fail, Query's real contract-incompatible mask denial, admitted-result construction denial, and publication projection/diagnostic-mask denials all pass. |
| R7.20 | 7.3 | Every protected fact that can affect predicates, root guards, ordering, cursors, counts, aggregates, explanations, history, preview, or live delivery requires the matching installed influence permission before the read can occur. | **PROVED** | Production source-cone review accounts for predicates, guards, ordering, continuation, historical, preview, and live identity reads. The production-used influence predicate rejects each implemented observable when omitted, and real forbidden predicate, root-guard, ordering, live-scope, and live-target contracts deny before result construction. Aggregate and explanation remain outside this gate because no execution surface exists. |
| R7.21 | 7.3 | Protected internal working values may participate only in admitted computation and never enter domain projection, typed omissions, terminal receipts, publication, diagnostics, formatting, or retained serialization. | **PROVED** | Hidden-ordering execution proves the value orders rows but is absent from both ordinary and disclosed projection access; the recursively consuming working-to-disclosed transition is compiler-visible; cross-lane paired worlds remain equal; certification finds no protected-value or policy lane in publication. |
| R7.22 | 7.3 | The terminal disclosure receipt preserves each exact result-slot decision and omission without deduplication, value leakage, or reusable capability/internal-computation authority. | **PROVED** | The one-shot pair proves ordered unique slot keys and four repeated required-disclosure values remain four decisions. Bank proves its relation and field remain two ordered decisions with the same value and exact outcome through the publication terminal. Private constructors, move-only governance, and publication-receipt compile denial prevent reuse as authority. |
| R7.L | all | The ledger maps every normative Phase 7 requirement, causal dependency, relevant risk category, consumer boundary, and performance claim without duplicating tests into a Cartesian matrix. | **OPEN** | Skeptical ledger audit at every gate closure; discoveries append findings and block unfinished causal rows or create an append-only corrective phase. |

## Finding ledger

| ID | Impact | Finding | Status | Closure evidence |
|---|---|---|---|---|
| Q7.1 | Critical | Query's existing application authorization grammar installed only allow/deny graph paths for named abilities. Bank estate capability, purpose, disclosure, delegation, conflict, and elevation meaning therefore remained bank-owned descriptive structs, policy-name placeholders, and independent oracles rather than generic installed Query meaning. | **CLOSED** | R7.1-R7.5 now install the full generic meaning; Bank declarations consume it, while the older courtroom remains an independent oracle and opens no Query authority. |
| Q7.2 | High | A capability declaration could become an unvalidated schema bag unless every typed reference participates in identifier validation, member closure, canonical identity, installed authority, and public compiler hostility. | **CLOSED** | Every member family is identifier-validated, schema-closed, canonical, installed through a keyed authority seal, and covered by public category and authority denials. |
| Q7.3 | High | Treating an omitted dimension as an implicit wildcard would silently create global authority. | **CLOSED** | Contract typestate requires all dimension families; optional dimensions require explicit `NotApplicable`; schema fields preserve lawful absence rather than sentinel values; structured identity records that posture; narrowing rejects `Bound`/`NotApplicable` substitution in both directions. |
| Q7.4 | High | Capability identity or installation that hashes per grant or admission would make the warm authorization lane population-dependent. | **CLOSED** | Canonical preparation and SHA derivation occur once per installed contract; retained typed lookups prove exact-zero preparation, digest, and digest-text work over 4,096 repetitions. |
| Q7.5 | High | Capability context and provenance initially existed only as unchecked names, and exact action actors were represented by broader estate-level paths. | **CLOSED** | Contexts, entity slots, and provenance are declared typed schema members; member closure rejects aliases and foreign anchors; Bank request, approval, authority, and review rules anchor the exact action entity. |
| Q7.6 | High | Exact actor conjunction initially expanded into a budget-hostile disjunctive normal form, then two different conjunction grammars could encode the same meaning with different identities. | **CLOSED** | The sole grammar is clause = one path, requirement = OR, rule = AND; same-leaf OR/AND twins prove distinct identity, and all 17 Bank contracts remain inside the unchanged 256-entry/64-KiB budget. |
| Q7.7 | Medium | Installed authority seals and registry keys accepted ordered groups of same-typed strings, leaving identity-field swaps compiler-representable. | **CLOSED** | Seal transcripts derive all semantic fields from one erased contract; registry keys derive from either that contract or typed capability and operation references. |
| Q7.8 | High | The new public context-slot macro changed the facade while the governed facade snapshot and generated `worth-query-decl` context still described the old surface. | **CLOSED** | The official boundary and agent-context generators updated the exact artifacts; both constitutional checks pass on the repaired tree. |
| Q7.9 | Critical | Phase 7.2 boundary tracing showed that installed capability meaning did not identify the active grant-status predicate or the resource-side workflow field. A runtime admission could therefore prove revocation and current workflow only by inventing application policy or trusting caller convention. | **CLOSED** | Typed currentness meaning now owns active status, grant workflow, resource workflow, and validity. Closure, canonical identity, authority, budget, Bank installation, Clippy, boundary, generated-context, and composition evidence were rerun before execution work resumed. |
| Q7.10 | Critical | Validity named two fields but no installed timeline interpretation or trusted current-time owner. A caller-authored `now` value could make an expired or not-yet-valid grant appear current. | **CLOSED** | Installation owns the typed validity timeline; Query's private clock samples it, retains the exact typed value, and resamples at access progression, idempotency inspection, and commit. Caller time cannot enter the authority API; future, expired, exhausted-clock, and pre-epoch cases fail closed. |
| Q7.11 | Critical | A capability request can describe an amount, field, relation, purpose, or context separately from the operation input or query parameters that later execute. Without exact binding, a caller could understate the authorization request beside a wider consumer action. | **CLOSED** | Capability admission owns the exact input and derives its projection internally. Operation progression recomputes and compares resource, action, purpose, relation, field, amount, cardinality, and constrained context; the Bank consumer independently exercises every one-axis mismatch. |
| Q7.12 | Critical | At the Phase 7.2 audit boundary, Relational emitted its own allow/deny decision and application-query admission dropped the resulting evidence. Signal and Runtime Bridge were subsequently given the installed nested grammar and Relational was neutralized, but the installed capability grammar still lacks one retained Query admission authority. | **CLOSED** | Relational emits neutral witness evidence, Bridge retains Signal-backed decision evidence, and Query combines both with principal, request, time, and exact-grant facts in one move-only admitted access proof. Conventional application-query plans also retain their exact principal/ability facts and revalidate them before every read. |
| Q7.13 | High | R7.7 originally required every query lane during gate 7.2, duplicating the explicit Phase 7.7 cutover and obscuring whether 7.2 owned proof capability or consumer adoption. | **CLOSED** | R7.7 now owns retained revalidation semantics; R7.14 remains the sole full-lane cutover guarantee. |
| Q7.14 | Critical | Relational authorization rehashed plan and observation meaning on every warm read and collapsed each traversal frontier to entity IDs, so future cross-ordinal constraints could synthesize one witness from unrelated graph branches. | **CLOSED** | Relational now emits policy-neutral evidence with a constant-work non-authority correlation token, retains complete path witnesses, proves cross-branch field values cannot join, and contains no authorization SHA/canonical-basis residue. |
| Q7.15 | High | Warm operation authorization canonically hashed a redundant scope fingerprint, and even an empty mutation-precondition request derived a digest. | **CLOSED** | The scope fingerprint now carries typed runtime/schema/operation/principal/scope identity directly; empty preconditions carry `None` plus exact-zero canonical-work evidence, while nonempty preconditions retain the bounded digest lane. |
| Q7.16 | Critical | The first Phase 7.2 implementation collapsed current capability admission directly into full operation authority. Bank estate commands intentionally lack executable operation programs until Phase 7.7, so the API either required premature Bank cutover or could not prove capability admission through the real consumer. | **CLOSED** | Public capability admission returns a private-field, move-only access proof that owns input, projection, trusted sample, resolved resource, and decision facts but has no execution method. Only `authorize_capability_operation` can consume it, and that transition requires an independently installed capability-mode operation. |
| Q7.17 | High | Conventional operation admission determined whether a capability governed an operation by scanning every compiled capability plan. Unrelated installed capabilities therefore entered ordinary warm admission work before any relational fact was observed. | **CLOSED** | Installation derives one exclusive principal/abilities/capability enum on the operation contract. Conventional admission checks `requires_capability()` in constant work before graph observation; no capability registry scan remains. |
| Q7.18 | Critical | Installed operation decision-fact contracts budgeted only named ability requirements. They reserved no fact for mapped-principal currentness and no capability decision fact, so a capability-authorized operation could not enter provider comparison under an honest exact fact contract. | **CLOSED** | The installed authorization mode derives exact fact count: principal `1`, abilities `1 + n`, capability `2`. Provider binding emits the principal fact plus exact policy facts, and decision-read-set capture enforces the installed exact cardinality. |
| Q7.19 | Critical | Mutation admission checked the mapped principal only at admission and then retained authentication expiry plus policy observations. Disabling or remapping that principal before provider commit was not an atomic authorization dependency and could leave otherwise current policy evidence attached to a stale identity binding. | **CLOSED** | Every authorization family retains the exact mapping and target observation as its first decision fact. Query and provider comparison revalidate it with principal-before-policy precedence; disabling the mapping denies before projection, commit, or idempotent disclosure. |
| Q7.20 | Critical | Capability evidence retained the trusted admission-time sample and compared only graph-backed facts afterward. An unchanged grant could therefore pass access progression or provider comparison after its installed `not_after` bound elapsed. | **CLOSED** | Retained capability authority owns installed validity bounds and the last trusted sample. Every privileged re-admission resamples through Query's private time source, re-evaluates the exact anchored grant, and rejects expiry without canonical reconstruction or caller time. |
| Q7.21 | Critical | Query erased application field presence while lowering every declared field as Foundational-required. Legitimate Bank grants with a non-applicable field or amount therefore failed installation, and the Phase 7.2 fixture concealed the defect with semantically false field and one-cent sentinels that the independent Bank oracle correctly rejected. | **CLOSED** | Required/optional presence is typed and identity-bearing, lowers exactly into Foundational, and is tested for lawful absence plus present-value validation. Bank fixtures contain no field or amount sentinels, and Query admission agrees with the independent estate oracle across the positive and one-axis currentness worlds. |
| Q7.22 | High | Authorization implementation is split between the committed top-level `domain_computation/authorization` destination and a competing `primary_graph/authorization` owner, obscuring which layer owns installed policy, current observation, and attempt authority. | **CLOSED** | `domain_computation/authorization` is the only production decision owner. `primary_graph` supplies graph substrate, execution integration, and audience re-exports; the old module/import path has no tracked residue. |
| Q7.23 | High | Capability admission reports a hard-coded zero canonical-work value while the Phase 6 warm-path residue scan omits the authorization directory. New warm canonicalization or hashing could therefore survive both reported evidence and the current source guard. | **CLOSED** | The recursive Phase 6 warm-consumer oracle now includes the complete authorization directory and rejects direct SHA, legacy digest helpers, canonical preparation, digest derivation, and text rendering. Cold counters and Bank population-scale typed counters independently remain exact. |
| Q7.24 | Critical | Application commit resolves an equivalent idempotency binding before re-admitting the retained principal and capability facts. A disabled principal, revoked grant, or expired capability can therefore receive the prior commit receipt through a stale admission even though the ordinary commit path would deny current authority. | **CLOSED** | Entry, public inspection, stale-read-set, and final commit paths re-admit retained authorization before resolving or disclosing idempotency. The token borrows the actual admission and rechecks request/authentication lifecycle at `govern`; drift denies at the decision-read-set boundary. |
| Q7.25 | Critical | Capability reauthorization re-runs the installed policy against any currently allowing grant and replaces the retained decision. Because the admitted authority does not retain the exact matched grant/path witness, revoking grant A while adding or activating equivalent grant B can migrate an existing access context instead of staling it. | **CLOSED** | Relational retains the complete matched witness; installation owns a typed grant-path index and entity ordinal; Query retains that exact grant and original decision. Later observations are anchored to it, so equivalent grant or same-grant path replacement cannot rescue stale authority. |
| Q7.26 | Critical | The first exact-grant repair relied on an undocumented cross-file convention that the grant-bearing policy path was index zero and the grant entity was witness ordinal one. Reordering lowering could silently bind request predicates and retained authority to a composition path instead of the installed grant path. | **CLOSED** | Capability lowering now installs one typed grant-witness binding; request predicates, exact anchors, and witness extraction consume it, malformed indices or ordinals deny as invalid installed policy, and the complete 19-test capability family includes the equivalent-grant replacement proof. |
| Q7.27 | Critical | Splitting retained capability evidence for provider execution dropped the original authorization decision from the final commit basis. Final serialized re-admission could therefore accept a different currently valid policy path under the same grant instead of rejecting stale decision evidence. | **CLOSED** | The commit basis retains the original typed decision fact and checks principal plus decision freshness before exact-grant observation. The production-boundary regression first reproduced acceptance, then proves grantor-to-custodian replacement denies as `StaleAuthorization`; all 19 capability tests pass afterward. |
| Q7.28 | Critical | Commit authorization is minted from retained facts plus a caller-supplied admission identity rather than the actual admitted operation. The token can therefore be paired with a different admission identity, and cancellation or authentication expiry after token minting is not rechecked when the governed idempotency or commit action runs. | **CLOSED** | Each commit basis now retains its originating admission identity; every serialized token borrows the actual admitted operation and rechecks lifecycle inside `govern`; caller-supplied identity governance is gone. Cancellation returns the unconsumed governed subject for explicit cleanup, mismatched bases deny, and the focused token, capability, plus application authority-lifecycle suites pass. |
| Q7.29 | High | Application-query execution collapsed retained principal currentness and authorization-observation currentness into one boolean, misreporting a disabled mapped principal as stale policy. Older lane tests also expected a freshly evaluated permission denial after retained evidence changed, contradicting exact freshness comparison. | **CLOSED** | The authorization owner now returns a typed currentness result with principal-before-policy precedence. Disabled principal mappings remain `StalePrincipal`; changed ownership evidence becomes `StaleAuthorization`; serialized application commit denies at the decision-read-set boundary before any equivalent receipt can be disclosed. |
| Q7.30 | Medium | The installation warm-lookup residue oracle parsed obsolete literal counter field initializers, so the full installation suite failed after lookup evidence moved behind the typed canonical-work carrier even though behavior remained exact-zero. | **CLOSED** | The source oracle now checks the zero-work typed constructor plus forbidden canonical/SHA helpers; the Bank installation and admission consumers independently assert typed registry-probe, basis-preparation, digest-derivation, and digest-text counters. |
| Q7.31 | Medium | The Bank installation consumer still expected the removed `MissingAbility` denial for an estate operation, encoding the obsolete rule that every capability-governed operation also needs a parallel ability contract. | **CLOSED** | The consumer now proves that Phase 7.1 still opens no estate execution authority because the operation program is absent. Capability meaning alone supplies the exclusive authorization mode; only a separately installed program can make the operation executable. |
| Q7.32 | Critical | Phase 7.3 governance currently mints its internal-computation proof before the managed graph-work session exists, so the proof has no session, managed-run, branch, basis, or provider affinity and relies on a later conventional identity check. | **CLOSED** | Governance now mints after `start_query`, retains every affinity, and is consumed by protected reads. Real-plan affinity tests cover honest, fresh-session, foreign-runtime, query, parameter, principal, and scope cases; inherited provider-session tests and the sealed exact matcher cover branch, basis, managed run, and provider without counterfeit construction. |
| Q7.33 | Critical | Disclosure contract admission validates Foundational projection and diagnostic masks but discards them, leaving later protected reads authorized only by copied field names and influence enums. | **CLOSED** | The private admitted field retains both typed masks. Production mask-category admission returns the exact artifacts, and contract-incompatible projection-mask admission denies against the real installed layout. |
| Q7.34 | Critical | Root-path guards read protected fields and change row membership without participating in disclosure influence validation or consuming internal-computation authority. | **CLOSED** | Root guards participate in the membership cone and consume exact internal projection admission. Lawful and forbidden real contracts plus protected-label paired worlds prove both directions. |
| Q7.35 | High | A protected result field may be permitted to influence ordering while omitted from consumer disclosure, but materialization removes it before ordering, so the declared separation between internal use and disclosure is not executable. | **CLOSED** | Private working material survives through bounded ordering, then the consuming disclosed-tree transition removes it before projection. The positive hidden-ordering journey and forbidden-ordering twin both pass. |
| Q7.36 | High | The disclosure receipt deduplicates required disclosure values and loses their result-slot identities, so it cannot carry the exact decisions and omissions required by the specification. | **CLOSED** | Receipts carry sorted `ApplicationQueryResultSlotKey` decisions. The canonical one-shot test retains four repeated private-label decisions, and Bank retains two equal customer-identity decisions through publication. |
| Q7.37 | High | The existing paired-world test observes only one-shot rows, a subset of counters, and a lossy receipt; it cannot falsify leaks through continuation boundaries, history, preview, live delivery, or publication. | **CLOSED** | Source checkpoints `a117753c3` and `7f5accb47` establish the canonical pair and compare one-shot, two continuation pages, history, preview, live payloads, omissions, lifecycle, and declared work. Bank supplies the real public publication boundary and exact receipt equality. |
| Q7.38 | Critical | The first checkpoint populated the internal-field rule map from both `.use_field_by(...)` and result disclosure selectors. A disclosed result rule could therefore open a predicate, ordering, or live identity read without separately admitted internal computation, while an omitted result rule could incorrectly veto a lawful internal rule for the same field. | **CLOSED** | Only `InternalField` selectors enter computation rules. Result-rule predicate hostility and real forbidden predicate, guard, ordering, live-scope, and live-target contracts prove result disclosure cannot mint internal authority. |
| Q7.39 | Critical | The hidden-ordering repair retained the protected ordering value in the same projection node later presented to application projection. A projection implementation using the ordinary required-field accessor could therefore recover working material that should have become a typed omission. | **CLOSED** | `WorthQueryApplicationWorkingProjectionTree::into_disclosed` recursively strips internal-only fields and yields a distinct disclosed node. The projection test requires ordinary accessor denial and typed omission; the compiler oracle now records the disclosed-node constructor boundary. |
| Q7.40 | High | Influence validation does not model the actual consumer-observable cone. Ordering omits historical, preview, and live influence requirements, while predicate validation demands aggregate and explanation permissions even though those surfaces are not implemented, and live validation treats every result rule as a membership influence regardless of the installed live cause. | **CLOSED** | Production derives membership and ordering cones from enabled continuation/history/preview/live lanes, and live checks only the installed scope and target identities. A production-used one-axis family rejects every implemented observable while explicitly excluding aggregate and explanation. |
| Q7.41 | High | The checkpoint retained typed masks but ordinary read wrappers merely assigned them to underscore variables. Exact projection-mask consumption therefore remained conventional, and diagnostic masks were made available beside data reads even though they carry a different category and purpose. | **CLOSED** | Every protected read requires `WorthQueryApplicationInternalProjectionAdmission` minted from the retained projection mask and exact field path. No diagnostic accessor exists; Foundational and publication compiler denials prove category substitution cannot open the lane. |
| Q7.42 | High | The authorization-owned governance implementation imports and constructs its receipt from `primary_graph::application_query`, reversing the intended authority direction and leaving the exact disclosure decision artifact owned by its consumer integration layer. | **CLOSED** | Authorization owns the sole receipt definition; application query only re-exports it. Certification proves one definition, no authorization-to-application-query import, and no publication policy/value lane. |
| Q7.43 | Medium | The Phase 7.3 disclosed-tree cutover changed the private projection-row constructor parameter, but the public compile-fail oracle still expected the pre-cutover working-node signature. The boundary remained private, yet the stale oracle made the complete compiler lane red. | **CLOSED** | The expected diagnostic now records `WorthQueryApplicationDisclosedProjectionNode<'row>` while still requiring private construction. All 39 application-schema compiler cases and the complete Query workspace pass. |

## Phase 7.1 closure evidence

- `cargo test -p worth-query-declaration -p worth-query-installation`
- `cargo test -p worth-query-execution registry_lowering::tests`
- `cargo test -p bank-domain`
- `cargo test -p bank-server estate_capability_admission -- --nocapture`
- `cargo test -p worth-query-certification --test application_schema_compile_fail`
- strict Clippy over the Query declaration, installation, execution, facade,
  and certification packages and over `bank-domain` plus `bank-server`
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`
- all Phase 7.1 Rust files remain within the 400-line limit; every advisory
  function was inspected against its semantic responsibility

## Phase 7.2 closure evidence

- `cargo test -p worth-query-execution --lib -- --format terse` (`399 passed`)
- `cargo test -p worth-query-execution --doc -- --format terse` (`16 passed`)
- `cargo test -p worth-query-installation -- --format terse` (`146 unit` and
  `3 doc` tests passed)
- `cargo test -p worth-relational authorization:: -- --format terse`
  (`11 passed`)
- `cargo test --manifest-path workspaces/worth-query-bank-world/Cargo.toml
  -p bank-domain --test estate_capability_installation -- --format terse`
  (`11 passed`)
- `cargo test --manifest-path workspaces/worth-query-bank-world/Cargo.toml
  -p bank-server estate_capability_admission -- --format terse` (`6 passed`)
- strict `--no-deps` Clippy with `-D warnings` for Query installation/execution
  and Bank domain/server; the unrelated transitive Relational warning baseline
  remains outside this slice
- dirty Rust line-cap audit: every changed production, test, and fixture file is
  at or below 400 lines; function scrutiny reports zero scan errors
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`

## Phase 7.3 restart audit

The restart audit froze source at `e33cca85d` after Milestone 9.16.1 closure.
It reran the inherited lower-owner proof families before changing Phase 7.3:

- `worth-query-declaration`: `98 passed`;
- `worth-query-installation`: `151` unit tests passed and both documentation
  test groups passed;
- `worth-query-execution --lib`: `414 passed`;
- root-workspace `worth-relational authorization::`: `12 passed`;
- Bank `estate_capability_installation`: `11 passed`; and
- Bank `estate_capability_admission`: `9 passed`.

Those results refresh the stale historical counts for R7.1-R7.7 and
R7.16-R7.17. They do not prove any Phase 7.3 row. The boundary review that
followed opened Q7.32-Q7.37 and expanded the ledger with R7.18-R7.22 before
implementation began.

### Phase 7.3 implementation checkpoint

The first corrective implementation checkpoint relocates disclosure-policy
admission into the authorization owner, binds governed computation to the
managed graph-work session, retains category-specific Foundational masks,
admits root guards, ordering, and live target reads through the internal
decision, and preserves exact result-slot outcomes in terminal receipts.

Checkpoint verification:

- `cargo test -p worth-query-execution`: `414` unit/integration tests and `17`
  documentation compile-fail tests passed;
- `cargo test --workspace` in `workspaces/worth-query`: the complete Query
  workspace, certification, compile-fail, integration, and documentation lanes
  passed;
- `cargo check --workspace` in `workspaces/worth-ui`: passed against the
  checkpoint facade;
- strict `cargo clippy -p worth-query-execution --all-targets -- -D warnings`:
  passed;
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`:
  passed;
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`: passed;
  and
- every Rust file changed by this checkpoint is at or below 400 lines and the
  dirty-file function scrutiny completed with zero scan errors. The global
  line-cap guard remains red on the repository's pre-existing, out-of-slice
  over-cap baseline, so this evidence does not claim repository-wide cap
  closure.

This is an implementation checkpoint, not Phase 7.3 closure. R7.8-R7.9 and
R7.18-R7.22 remain open until the hostile and paired-world proofs named in
their rows close Q7.32-Q7.37.

### Phase 7.3 mechanical disclosure boundary checkpoint

Source checkpoint `08ea80ea9` corrects the second-pass defects discovered in
the first implementation checkpoint without claiming Phase 7.3 closure:

- internal computation is derived only from `.use_field_by(...)` selectors;
  result disclosure selectors cannot mint predicate, guard, ordering, or live
  read authority;
- every protected read consumes an exact projection-field admission derived
  from the retained Foundational `ProjectionMask`, while the retained
  `DiagnosticMask` is inaccessible to ordinary data reads;
- the observable influence cone now follows the implemented predicate, guard,
  ordering, continuation, historical, preview, and live surfaces without
  inventing aggregate or explanation execution;
- internal projection material is consumed into a recursively sanitized
  disclosed tree before domain projection, so protected ordering values cannot
  reach either the ordinary or disclosed field accessor;
- terminal disclosure receipts preserve an ordered decision for every exact
  result slot, including repeated disclosure values; and
- the disclosure receipt is owned by
  `authorization/application_disclosure`, leaving application query as an
  audience re-export rather than a reverse authority dependency.

Focused evidence at this checkpoint includes a result-selector predicate
hostility test, a hidden-ordering working-value test, exact repeated-slot
receipt assertions, `418` passing execution tests, strict execution Clippy,
and both constitutional boundary checks. The global line-cap guard remains red
only on the pre-existing repository baseline; all checkpoint files are within
the cap.

This checkpoint corrects the production roots of Q7.33, Q7.35-Q7.36, and
Q7.38-Q7.42, but those findings remain `OPEN` until their named compiler,
contract-hostility, cross-lane, publication, and residue evidence is complete.
Q7.32, Q7.34, and Q7.37 still require additional implementation evidence.
Consequently R7.8-R7.9 and R7.18-R7.22 remain `OPEN`.

### Phase 7.3 closure

The closure batch retains source checkpoints `a117753c3` and `7f5accb47` as
the canonical cross-lane paired-world evidence and adds the missing hostile,
affinity, compiler, publication, and consumer proofs:

- Foundational mask categories remain distinct at compile time, and Query's
  production mask-admission step rejects a real selector mask containing a
  field outside the installed `AspectContract`;
- the production-used influence predicate rejects each implemented observable
  independently, while real installed contracts deny forbidden predicate,
  root-guard, ordering, live-scope, and live-target influence before result
  construction;
- real governed plans match only their exact session-bound governance. A fresh
  session, foreign runtime, alternate installed query, different admitted
  parameters, second valid principal, and second resolved scope all deny;
  sealed lower-owner session evidence and the exact matcher retain branch,
  basis, managed-run, and provider affinity without test-only construction;
- consumers cannot construct the admitted disclosed result, and publication
  rejects raw values, projection masks, and diagnostic masks;
- the Bank public consumer proves its relation and field remain two distinct,
  ordered slot decisions with the same disclosure requirement and outcome,
  and that the complete terminal disclosure receipt survives publication
  unchanged; and
- certification proves one authorization-owned disclosure receipt, no reverse
  authorization-to-application-query import, and no protected-value or policy
  lane in publication.

The QA loop also found Q7.43: the private projection-row compiler oracle still
described the pre-sanitization working node. The expected diagnostic now names
the disclosed node while preserving the same public construction denial.

Closure verification on the final Phase 7.3 source:

- `cargo test -p worth-query-execution`: `430` unit tests and `18`
  documentation compile-fail tests passed;
- `cargo test --workspace` in `workspaces/worth-query`: the complete Query
  workspace, all compiler suites, integration lanes, and documentation tests
  passed;
- `cargo test -p worth-query-certification --test
  application_schema_compile_fail`: all `39` compiler cases passed;
- `cargo test -p worth-query-publication --doc`: all `5` compile-fail cases
  passed;
- `cargo test -p worth-foundational --doc`: the projection/diagnostic mask
  category denial passed;
- `cargo test -p bank-server estate_capability_admission -- --nocapture`:
  all `9` Bank admission/publication tests passed;
- `cargo check --workspace` in `workspaces/worth-ui`: all consumer crates and
  the Platform Pulse application passed;
- strict `-D warnings` Clippy passed for Query execution, publication,
  certification, and Bank server; and
- the disclosure ownership/publication residue test, dirty-file function
  scrutiny, dirty Rust line-cap audit, both constitutional checks, formatting,
  and `git diff --check` passed.

The Phase 7.3 ledger audit closes R7.8-R7.9, R7.18-R7.22, and Q7.32-Q7.43.
Aggregate and explanation are not falsely claimed: they have no executable
consumer surface in this gate. R7.10 is now `OPEN`; R7.11-R7.15 remain blocked
by their declared predecessors. R7.L remains `OPEN` until all Runtime Phase 7
gates close, so this section does not claim Phase 7 completion.

## Test-selection policy

Evidence is selected from the contract rather than multiplied across all input
combinations:

1. one positive exemplar for each legal transition;
2. one one-axis negative for each independent dimension or predicate;
3. one test for each interaction explicitly declared by a composition rule;
4. one hostile test at each privileged phase boundary;
5. one public consumer transcript per materially distinct facade; and
6. growth probes over every variable that could accidentally enter warm work.

Security, privacy, performance, architecture, lifecycle, and other risk
categories are included only where the causal ledger says they can invalidate
a guarantee. They are prompts for skeptical review, not a fixed Cartesian test
matrix.
