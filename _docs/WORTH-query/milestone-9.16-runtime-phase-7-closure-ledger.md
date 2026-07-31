# Milestone 9.16 Runtime Phase 7 Closure Ledger

**Owner:** Runtime Hardening Track, Phase 7
**Canonical specification:** `milestone-9.16.md`
**Status:** Open — Phase 7.1 proved; Phase 7.2 implementation in progress
**Policy:** A requirement is `PROVED` only when its production owner, public
consumer evidence, adversarial evidence, performance posture, and residue
posture agree. A finding is `CLOSED` only when the root cause and every
causally dependent guarantee have been rechecked.

This ledger is the durable meaning of the `R7.*` and `Q7.*` identifiers used by
the milestone. A green broad test does not change a row's status. New findings
reopen the earliest affected gate and every causal dependent.

## Requirement ledger

| ID | Gate | Guarantee | Status | Required closure evidence |
|---|---:|---|---|---|
| R7.1 | 7.1 | Capability identity is typed and distinct from role, relationship, authentication, policy result, operation authority, and runtime proof. | **PROVED** | Typestate-complete contracts, typed category and context-slot compiler denials, descriptive-capability denial, installed-authority root/runtime/generation/operation hostility, and Bank denial of premature execution authority. |
| R7.2 | 7.1 | Action, resource, relation, field, purpose, amount, cardinality, workflow stage, validity timeline, active grant status, delegation, provenance, and constrained context are explicit identity-bearing dimensions; grant workflow is aligned to an explicit resource-side current field. | **PROVED** | Schema closure, one-axis canonical identity twins, and all 17 Bank contracts prove the installed validity timeline, active status, and resource workflow binding. |
| R7.3 | 7.1 | Allow, deny, conflict, separation-of-duty, distinct-actor, delegation, and disclosure composition is installed canonical application meaning. | **PROVED** | The full declaration/installation and Bank suites pass against the repaired currentness basis; all 17 contracts retain their exact generic composition and actor anchors. |
| R7.4 | 7.1 | Capability meaning is prepared once through Foundational canonical basis and typed sequence digest, while Query installation supplies the stronger authority binding. | **PROVED** | Structured comparison, digest identity, and installed authority hostility include the validity timeline and currentness fields. |
| R7.5 | 7.1 | Capability installation has bounded contract count and canonical bytes; warm installed-capability lookup performs zero basis preparation, hashing, or digest text materialization. | **PROVED** | Exact entry/byte ceilings and 4,096 retained lookups prove zero warm basis preparation, digest derivation, and digest text materialization. |
| R7.6 | 7.2 | Explicit purpose and constrained request context become attempt-bound access authority from current relational truth and installed contracts. | **OPEN** | Currentness, touched-graph, stale/foreign/copied-context, and independent policy-oracle evidence. |
| R7.7 | 7.2 | Exact authorization decision facts and read sets can be retained and revalidated at a later snapshot without rebuilding policy meaning or granting reusable authority. | **OPEN** | Compare-and-commit relevant drift denies, unrelated drift remains lawful, and revalidation performs no canonical preparation, digest derivation, or digest text materialization. Full lane consumption remains R7.14. |
| R7.8 | 7.3 | Internal computation authority is distinct from consumer disclosure authority and both narrow Foundational projection/diagnostic masks. | **BLOCKED BY 7.2** | Compile denials for mask substitution plus paired-world noninterference across every observable surface. |
| R7.9 | 7.3 | Publication accepts only typed disclosed-or-omitted results and cannot inspect protected values or decide policy. | **BLOCKED BY 7.2** | Public consumer compilation and serialization-residue denial. |
| R7.10 | 7.4 | Delegation is a proof-carrying narrowing transition with exact lineage and current parent re-admission. | **BLOCKED BY 7.3** | Every dimension widening, copied-parent, depth/width, expiry, and revocation attacks. |
| R7.11 | 7.5 | Allow, deny, conflict, separation-of-duty, and distinct-actor rules form one installed decision over exact actor and touched-graph facts. | **BLOCKED BY 7.4** | Contract-derived interaction matrix and hostile drift sequences without Cartesian expansion. |
| R7.12 | 7.6 | Emergency elevation is a linear installed typestate with a bounded upper authority and mandatory distinct-actor review. | **BLOCKED BY 7.5** | Lawful lifecycle plus self-approval, conflict, widening, expiry, revocation, copying, and repeated-review denial. |
| R7.13 | 7.7 | Bank estate operations use the public Query progression with no bank-local authority executor. | **BLOCKED BY 7.6** | Complete estate courtroom, consumer transcript, dependency direction, and legacy residue. |
| R7.14 | 7.7 | Every supported query lane re-admits identical capability, purpose, disclosure, and conflict meaning without changing query identity or result meaning. | **BLOCKED BY 7.6** | One-shot, continuation, history, preview, and live consumer evidence. |
| R7.15 | 7.7 | Warm authorization work depends only on declared touched evidence, not unrelated grants, relationships, fields, cases, rows, or consumers. | **BLOCKED BY 7.6** | Growth measurements with exact-zero canonical preparation, hashing, digest text, and fallback. |
| R7.16 | 7.2 | Relational owns current graph observation, Signal owns installed boolean composition, Runtime Bridge owns correspondence, and Query alone combines those proofs into attempt authority; no layer silently recreates another layer's decision. | **OPEN** | Neutral Relational evidence, nested Signal rules, Bridge correspondence, prohibited-path precedence, and decision-owner residue are proved. Installed capability compilation, retained Query authority, and compiler-inaccessible capability admission remain. |
| R7.17 | 7.2 | Capability plans compile at the cold application-runtime publication seam; warm admission performs typed lookup plus touched-graph work and cannot recompile or rehash installed meaning as unrelated graph population grows. | **OPEN** | Relational observation, operation-scope binding, and empty-precondition admission now prove zero warm canonical/SHA/text work. Cold capability-plan compilation counters and capability-scale probes remain. |
| R7.L | all | The ledger maps every normative Phase 7 requirement, causal dependency, relevant risk category, consumer boundary, and performance claim without duplicating tests into a Cartesian matrix. | **OPEN** | Skeptical ledger audit at every gate closure; discoveries append findings and reopen causal rows. |

## Finding ledger

| ID | Impact | Finding | Status | Closure evidence |
|---|---|---|---|---|
| Q7.1 | Critical | Query's existing application authorization grammar installed only allow/deny graph paths for named abilities. Bank estate capability, purpose, disclosure, delegation, conflict, and elevation meaning therefore remained bank-owned descriptive structs, policy-name placeholders, and independent oracles rather than generic installed Query meaning. | **CLOSED** | R7.1-R7.5 now install the full generic meaning; Bank declarations consume it, while the older courtroom remains an independent oracle and opens no Query authority. |
| Q7.2 | High | A capability declaration could become an unvalidated schema bag unless every typed reference participates in identifier validation, member closure, canonical identity, installed authority, and public compiler hostility. | **CLOSED** | Every member family is identifier-validated, schema-closed, canonical, installed through a keyed authority seal, and covered by public category and authority denials. |
| Q7.3 | High | Treating an omitted dimension as an implicit wildcard would silently create global authority. | **CLOSED** | Contract typestate requires all dimension families; optional dimensions require explicit `NotApplicable`; structured identity records that posture; narrowing rejects `Bound`/`NotApplicable` substitution in both directions. |
| Q7.4 | High | Capability identity or installation that hashes per grant or admission would make the warm authorization lane population-dependent. | **CLOSED** | Canonical preparation and SHA derivation occur once per installed contract; retained typed lookups prove exact-zero preparation, digest, and digest-text work over 4,096 repetitions. |
| Q7.5 | High | Capability context and provenance initially existed only as unchecked names, and exact action actors were represented by broader estate-level paths. | **CLOSED** | Contexts, entity slots, and provenance are declared typed schema members; member closure rejects aliases and foreign anchors; Bank request, approval, authority, and review rules anchor the exact action entity. |
| Q7.6 | High | Exact actor conjunction initially expanded into a budget-hostile disjunctive normal form, then two different conjunction grammars could encode the same meaning with different identities. | **CLOSED** | The sole grammar is clause = one path, requirement = OR, rule = AND; same-leaf OR/AND twins prove distinct identity, and all 17 Bank contracts remain inside the unchanged 256-entry/64-KiB budget. |
| Q7.7 | Medium | Installed authority seals and registry keys accepted ordered groups of same-typed strings, leaving identity-field swaps compiler-representable. | **CLOSED** | Seal transcripts derive all semantic fields from one erased contract; registry keys derive from either that contract or typed capability and operation references. |
| Q7.8 | High | The new public context-slot macro changed the facade while the governed facade snapshot and generated `worth-query-decl` context still described the old surface. | **CLOSED** | The official boundary and agent-context generators updated the exact artifacts; both constitutional checks pass on the repaired tree. |
| Q7.9 | Critical | Phase 7.2 boundary tracing showed that installed capability meaning did not identify the active grant-status predicate or the resource-side workflow field. A runtime admission could therefore prove revocation and current workflow only by inventing application policy or trusting caller convention. | **CLOSED** | Typed currentness meaning now owns active status, grant workflow, resource workflow, and validity. Closure, canonical identity, authority, budget, Bank installation, Clippy, boundary, generated-context, and composition evidence were rerun before execution work resumed. |
| Q7.10 | Critical | Validity named two fields but no installed timeline interpretation or trusted current-time owner. A caller-authored `now` value could make an expired or not-yet-valid grant appear current. | **OPEN** | Install a canonical timeline contract, sample time through Query-owned authority, retain the sample in decision facts, and reject caller-time substitution. |
| Q7.11 | Critical | A capability request can describe an amount, field, relation, purpose, or context separately from the operation input or query parameters that later execute. Without exact binding, a caller could understate the authorization request beside a wider consumer action. | **OPEN** | Carry the exact input/parameter binding through admitted authority and prove one-axis underreport attacks fail before the consumer opens. |
| Q7.12 | Critical | At the Phase 7.2 audit boundary, Relational emitted its own allow/deny decision and application-query admission dropped the resulting evidence. Signal and Runtime Bridge were subsequently given the installed nested grammar and Relational was neutralized, but the installed capability grammar still lacks one retained Query admission authority. | **OPEN** | Compile installed capability meaning into the runtime, retain exact Relational/Bridge evidence plus purpose/time/request binding in Query authority, and remove the remaining discarded application-query evidence path. |
| Q7.13 | High | R7.7 originally required every query lane during gate 7.2, duplicating the explicit Phase 7.7 cutover and obscuring whether 7.2 owned proof capability or consumer adoption. | **CLOSED** | R7.7 now owns retained revalidation semantics; R7.14 remains the sole full-lane cutover guarantee. |
| Q7.14 | Critical | Relational authorization rehashed plan and observation meaning on every warm read and collapsed each traversal frontier to entity IDs, so future cross-ordinal constraints could synthesize one witness from unrelated graph branches. | **CLOSED** | Relational now emits policy-neutral evidence with a constant-work non-authority correlation token, retains complete path witnesses, proves cross-branch field values cannot join, and contains no authorization SHA/canonical-basis residue. |
| Q7.15 | High | Warm operation authorization canonically hashed a redundant scope fingerprint, and even an empty mutation-precondition request derived a digest. | **CLOSED** | The scope fingerprint now carries typed runtime/schema/operation/principal/scope identity directly; empty preconditions carry `None` plus exact-zero canonical-work evidence, while nonempty preconditions retain the bounded digest lane. |

## Phase 7.1 closure evidence

- `cargo test -p worth-query-declaration -p worth-query-installation`
- `cargo test -p bank-domain`
- `cargo test -p worth-query-certification --test application_schema_compile_fail`
- strict Clippy over the Query declaration, installation, facade, and
  certification packages and over `bank-domain`
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`
- all Phase 7.1 Rust files remain within the 400-line limit; every advisory
  function was inspected against its semantic responsibility

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
