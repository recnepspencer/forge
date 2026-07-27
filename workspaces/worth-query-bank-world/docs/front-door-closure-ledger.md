# Bank World Front-Door Closure Ledger

## Closure rule

`PROVED` requires a named production owner, a positive witness, a hostile twin
where the claim has a denial boundary, an independent oracle, a consequential
assertion, and the cheapest exact command. `BLOCKED` names the later phase that
owns unavailable behavior. A green test alone does not close a row. A high- or
critical-impact finding reopens every row it can invalidate.

## Bank World Phase 1

| ID | Status | Claim and owner | Positive evidence | Independent or hostile evidence | Consequence and command |
| --- | --- | --- | --- | --- | --- |
| B1.1 | PROVED | `bank-domain` owns the exact entity, aspect, field, relationship, operation, installed operation-program, policy, currency, effect, and value inventory. | `schema/{entities,fields,relations,operations,program_manifest,governance,values,manifest}.rs`. | `schema_inventory::bank_manifest_matches_the_frozen_phase_one_world` compares every member family and each operation's exact action set to independent sets. | Adding, removing, renaming, or retargeting a member fails `cargo test --manifest-path workspaces/worth-query-bank-world/Cargo.toml -p bank-domain --test schema_inventory`. |
| B1.2 | PROVED | `bank-domain` owns customer and employee role meaning; graph relationships, never strings or token claims, are the future power source. | `model/roles.rs`; exact relationships in `banking-product-contract.md`. | The residue oracle denies raw descriptor construction; the courtroom requires role combinations and revocation. | Role/schema drift changes the manifest identity or inventory and fails the bank package command. |
| B1.3 | PROVED | `bank-domain` owns exact minor-unit money, typed USD, immutable-posting rules, operation purposes, and future invariant meaning. | `model/money.rs`, `schema/values.rs`, and the monetary-invariants section of `banking-product-contract.md`. | `wrong_currency_value.rs` denies a USD marker on `Money<EUR>`; declaration tests deny an undeclared currency. | Currency/capability drift changes canonical schema identity and fails declaration/compiler certification. |
| B1.4 | PROVED | The frozen mutation inventory includes account creation, opening funding, deposit, withdrawal, stable-recipient transfer, business initiation, approve, reject, grant, revoke, and reversal. | `schema/operations.rs`; exact mutation list in `banking-product-contract.md`. | The manifest inventory is independent of the declaration chain; `SendMoney` accepts `BankPrincipalId`, not a caller-selected destination account. | Operation drift fails `schema_inventory`; wrong input types fail compiler certification. |
| B1.5 | PROVED | Read and mutation outcome families have one bank-domain meaning owner without transport vocabulary. | `model/outcomes.rs`; outcome section of `banking-product-contract.md`. | HTTP mapping is explicitly downstream and cannot alter the enums. | Outcome implementation is intentionally `BLOCKED` on Runtime Phase 5; Phase 1 proves ownership and vocabulary only. |
| B1.6 | PROVED | Authentik OIDC code flow, issuer/subject identity, dynamic provisioning, independent user processes, and teardown posture are frozen as fixture responsibilities. | `async-identity-courtroom.md`. | The hostile inventory includes malformed/expired tokens, unknown or ambiguous mapping, crashes, response loss, and queue saturation; product source contains no participant identities. | Adapter behavior is `BLOCKED` on Runtime Phase 2 and Bank Phase 2; cross-process execution is `BLOCKED` on Bank Phase 4. |
| B1.7 | PROVED | Every required courtroom actor and combination has a dynamic real-process destination. | Dynamic-participant and process-topology inventories in `async-identity-courtroom.md`. | The fixture forbids locally minted identity, shared user runtimes, and privileged product mutation. | Future command is fixed as `cargo test --manifest-path workspaces/worth-query-bank-world/Cargo.toml -p bank-courtroom --test async_identity_courtroom`; crate creation waits for its first real Phase 2 responsibility. |
| B1.8 | PROVED | Real read, traversal, mutation, effect, approval, grant, and revoke authoring use only audience facades. | `public_consumer_transcript.rs` and the exact Phase 1 blocks in `public-consumer-contract.md`. | The transcript binds an installed schema and asserts binding identity, graph changes, and denial kinds. | `cargo test --manifest-path workspaces/worth-query-bank-world/Cargo.toml -p bank-domain --test public_consumer_transcript`. |
| B1.9 | PROVED | Authentication, policy admission, execution, commit, and live delivery are not simulated in Phase 1. | Later-execution block and prohibitions in `public-consumer-contract.md`. | No bank executor, HTTP crate, Authentik adapter, or courtroom crate exists prematurely. | Runnable behavior remains owned by Runtime Phases 2-5 and Bank Phases 2-4. |
| B1.10 | PROVED | Every known workaround is classified by its causal owner and phase. | Gap table below. | The ledger records typed graph mutation and currency association as resolved Phase 1 gaps rather than hiding them downstream. | Any new generic Query gap must add/reopen a Runtime row and phase before bank-local code may proceed. |

## Runtime Hardening Phase 1

| ID | Status | Claim and owner | Positive evidence | Independent or hostile evidence | Consequence and command |
| --- | --- | --- | --- | --- | --- |
| R1.1 | PROVED | Declaration owns one portable manifest covering entity, aspect, field, relation, operation, installed operation-program, policy, currency, and effect members. | `application_schema/{declaration,operation_program}.rs` and schema macros. | Canonical-identity tests vary every member family; bank inventory independently lists every family and exact program edge. | Missing closure or duplicate members deny in `cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-declaration`. |
| R1.2 | PROVED | Manifest identity is explicit, order-convergent, versioned, and includes field family/type/currency/capabilities, operation input, every create/delete/write/link/unlink/emit edge, and effect payload types. | `canonical_identity.rs`. | Identity tests vary every program action and field dimension; member order convergence is tested separately. | Capability-program drift changes package identity rather than sharing a digest. |
| R1.3 | PROVED | Installation alone binds a typed schema to exact runtime, generation, package identity, admission identity, owner, version, and schema meaning. | `worth-query-installation::{application_schema,installed_index/application_schema}.rs`. | Installation tests deny foreign runtime, stale generation, package drift, admission drift, owner/version mismatch, and schema drift. | `cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-installation application_schema`. |
| R1.4 | PROVED | Reads expose typed root entity, direction-typed relation traversal, projection, and equality filtering only where the field posture permits it. | `read_authoring.rs`; bank account and owner-traversal transcript. | `cross_schema_field`, `wrong_relation_direction`, `wrong_field_value`, and `unsupported_equality_operator` compiler cases; forged endpoints deny installed admission. | Illegal next actions do not compile; forged names return a typed authoring denial. |
| R1.5 | PROVED | Application values carry exact scalar family and Rust value type; currency-bearing values additionally prove the domain currency represented by the schema marker. | `values.rs`, currency capability types, and currency field macro. | `wrong_currency_value.rs`, missing-currency declaration test, forged-value and forged-currency consumer attacks. | Wrong type, scalar family, or currency cannot silently bind to the installed field. |
| R1.6 | PROVED | Mutations expose typed create, delete, relation link/unlink, and field write actions gated by operation-specific compile capabilities and the same exact installed operation-program edge. | `mutation_authoring.rs`, `operation_program.rs`, and operation capability macros. | `illegal_write.rs`, `illegal_graph_mutation.rs`, forged field/relation attacks, `compile_capability_without_installed_operation_edge_is_denied`, and dangling-program closure denial. | Grant and revoke carry explicit graph changes; an undeclared or merely compile-local graph effect cannot author successfully. |
| R1.7 | PROVED | Operation and effect authoring bind the exact operation input and installed emit edge plus declared effect payload. | `effect_authoring.rs`; bank mutation/effect transcript. | Wrong-operation input compiler case; forged operation, input, effect, and payload denials; program omission denial. | `effects(operation).emit(effect, payload)` requires both `OperationEmits` and the identical installed program edge. |
| R1.8 | PROVED | Descriptors and copied binding identity are descriptive only; the opaque installed-schema handle remains the authority root. | `ApplicationSchemaBindingIdentity` documentation and non-`Clone` `WorthQueryInstalledApplicationSchema`. | `installed_handle_constructor_is_private.rs`; index validation consumes exact package authority fields. | A forged descriptor/binding can author no executable authority; later execution must validate against the opaque handle. |
| R1.9 | PROVED | Dynamic application keys are absent; generated bank code has no accidental dynamic lane. | No dynamic reference type is exported. | `schema_inventory::bank_schema_source_has_no_raw_query_descriptor_or_dynamic_key_lane` scans production source. | Dynamic extension requires an explicit future phase/spec change rather than a string escape hatch. |
| R1.10 | PROVED | Bank application source has no raw application descriptor constructors or non-audience Query dependency. | Macro-authored schema modules and `bank-domain/Cargo.toml`. | Residue/dependency oracle plus boundary checker. Hostile tests may construct raw descriptors only to prove denial. | Production residue or a dependency bypass fails the bank package or constitutional gate. |
| R1.11 | PROVED | Ordinary declaration, installation, and bank consumer loops are isolated from compiler certification. | Dedicated `application_schema_compile_fail` target contains ten orthogonal cases. | Final warm times: declaration 2.26s, installation 0.44s, bank workspace 0.52s, focused trybuild 1.70s. Cold trybuild rebuilds remain deliberate and isolated. | Ordinary edits do not invoke the historical compiler matrix; compiler certification runs only on its exact command. |
| R1.12 | PROVED | Dependency compilation distinguishes primary-logical-graph evidence from separate-authority provider calls. | `dependency_impact/compilation/graph_calls.rs` filters both read and touch provider receipts by graph participation; local mutation retains runtime effect plus primary-read evidence. | `workflow_effect_execution` proves the local success and partial-effect paths; `dependency_impact::workflow_closure` names the declared touch/read edges and realized primary read; the complete 319-test installed-world matrix preserves hostile remote call and commit checks. | Primary graph work cannot be forced to invent provider receipts, while missing, duplicate, contaminated, or wrong-authority remote receipts still deny. |

## Gap classification

| Gap | Classification | Owning phase | Posture |
| --- | --- | --- | --- |
| Typed markers were self-authored and implied authority. | Query | Runtime Phase 1 | Resolved: markers are descriptive; exact installed handle owns binding authority. |
| Installation lacked a complete application-schema inventory. | Query | Runtime Phase 1 | Resolved by package/index manifest identity and binding validation. |
| Typed relations existed without direction-typed traversal. | Query | Runtime Phase 1 | Resolved with installed endpoint admission and compile-negative direction proof. |
| Currency markers were not tied to currency-bearing values. | Query | Runtime Phase 1 | Resolved with domain-currency-linked markers, manifest identity, closure, and hostile proof. |
| Mutation authoring could not express relationship grant/revoke or deletion. | Query | Runtime Phase 1 | Resolved with typed operation delete/link/unlink capabilities and installed admission. |
| Operation capability traits were absent from canonical installed schema identity. | Query | Runtime Phase 1 | Resolved with first-class operation-program members, closure validation, canonical identity, exact installed admission, and omission proof. |
| Dependency compilation treated primary graph touches as separate-provider calls. | Query | Runtime Phase 1 | Resolved by participation-symmetric receipt matching and explicit local mutation/read dependency evidence. |
| Ordinary execution of authored declarations is not composed. | Query | Runtime Phase 5 | Intentionally blocked; no bank-local executor. |
| Authentik proof admission does not exist. | Query and adapter | Runtime Phase 2 / Bank Phase 2 | Intentionally blocked; no token shortcut. |
| Scoped touched-graph authorization is not composed. | Query | Runtime Phase 3 | Intentionally blocked; descriptors and route checks grant no power. |
| Exact accounting behavior and invariants are not installed. | Bank domain | Bank Phase 3 | Meaning is frozen now; behavior waits for the owning phase. |
| HTTP and independent user-node processes do not exist. | Adapter and fixture | Bank Phase 4 | Intentionally blocked; no same-process substitute. |

No newly discovered generic capability requires a new phase. Currency
association, direction-typed traversal, typed graph mutation, and
primary-versus-provider dependency evidence were incomplete guarantees inside
Runtime Hardening Phase 1 and are closed there.

## Exact verification commands

```text
# Declaration
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-declaration

# Installation
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-installation

# Existing execution baseline (new application declarations remain blocked)
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-execution

# Installed-world regression, including primary and separate graph evidence
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query --test installed_operating_world

# Bank consumer and independent inventory/residue oracle
cargo test --manifest-path workspaces/worth-query-bank-world/Cargo.toml -p bank-domain

# Consolidated hostile compiler certification
cargo test --manifest-path workspaces/worth-query/Cargo.toml -p worth-query-certification --test application_schema_compile_fail

# Future cross-process courtroom, available when Bank Phase 2 creates its first real crate
cargo test --manifest-path workspaces/worth-query-bank-world/Cargo.toml -p bank-courtroom --test async_identity_courtroom
```
