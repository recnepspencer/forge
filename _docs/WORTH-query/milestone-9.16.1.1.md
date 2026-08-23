# Milestone 9.16.1.1: Installed Graph Contract Integrity Repair

> **Status:** Proposed — required before Milestone 9.16.2
>
> **Historical posture:** Milestone 9.16.1 remains closed with its recorded
> evidence. This append-only corrective sub-milestone records and repairs a
> later-discovered mismatch between the installed application-operation
> contract promised by that milestone and the public contract currently
> emitted by `worth-query-installation`. It does not rewrite the 9.16.1 ledger
> or revoke evidence that remains true.

## Goal

Make the installed application schema and operation contract the single typed
authority for every declared graph read, declared graph touch, native aspect
contract, and inspectable aftermath fact.

For one exact installed application operation, Query must expose and carry:

- the exact schema-bound entity, aspect, field, and relation loci it may read;
- the exact Foundational native aspect contract and projection mask for every
  declared field or whole-aspect projection;
- the exact typed graph mutation scopes it may perform;
- graph mutation and external-effect emission as distinct effect families;
- the exact installed reconciliation procedure and typed external-effect
  correlation family when those facts are declared; and
- one canonical installed contract consumed unchanged by graph-obligation
  selection, admission, execution lowering, terminal comparison, public host
  inspection, and successor package export.

The repaired progression is:

```text
typed application schema and operation declaration
    -> declaration-owned stable aspect identity and revision
    -> installed native application-schema contract catalog
    -> typed installed operation read and touch scopes
    -> sealed installed graph-obligation set
    -> admitted graph-work plan and provider session
    -> Relational registration from the installed native contracts
    -> actual read facts and touched-scope evidence
    -> terminal and descriptive public inspection
```

No runtime component may recreate, parse, widen, or guess the installed
meaning. No public inspection value grants read, mutation, correction,
dispatch, or recovery authority.

## Roadmap Placement And Append-Only Rule

Milestone 9.16.1.1 consumes:

- the closed canonical graph-obligation and provider-session progression from
  Milestone 9.16.1;
- the application-schema, operation, aftermath, and external-effect meaning
  already delivered by Milestone 9.16;
- Foundational native aspect contracts, identities, revisions, field paths,
  projection masks, canonical basis, and admissibility;
- Relational ownership of authoritative graph truth, exact performed touches,
  and native aspect registration; and
- the existing `worth-query-declaration`, `worth-query-installation`,
  `worth-query-execution`, and `worth-query-host` package direction.

Milestone 9.16.1 remains historically closed. Its closure ledger receives an
append-only post-closure finding that points here; previously proved session,
execution, lifecycle, publication, facade, and warm-cost rows remain inputs.
The finding corrects the narrower claim that installed application-operation
read and touch meaning was completely inspectable and typed.

Milestone 9.16.2 remains proposed and may not begin production implementation
until this milestone closes. Its stable-identity and package-export phases
consume the aspect identities, revisions, correlation-family identity, and
complete installed operation contracts established here. It may extend those
families with other package identities; it may not replace or reinterpret
them. The NCR vertical court in 9.16.2 must use the repaired public facade and
may not carry a local compatibility representation.

This milestone does not reopen Milestone 9.16 Runtime work or authorize a
general Query schema redesign. A newly discovered defect with an independent
telos receives its own append-only milestone.

## Central Claim

For an exact installed application operation, the public installed contract,
the sealed graph-obligation selection basis, and the runtime lowering all
refer to the same typed schema-bound read and mutation meaning.

The claim is false if any of the following is possible:

- an operation declares a decision read while its installed graph-read scopes
  are empty;
- an entity-existence or relation read is silently represented as a native
  aspect projection;
- a one-field read is widened to a whole-aspect projection;
- a same-named field, aspect, relation, or effect from another entity, schema,
  package, or runtime generation can substitute;
- execution assigns a second application aspect identity or rebuilds an
  application `AspectContract` from erased field declarations;
- a graph touch must be parsed from a string such as
  `write:Entity/Aspect/Field` or `link:Relation:From->To`;
- an external-effect emission is published as a graph mutation touch;
- declared touch scope is accepted as evidence that a touch actually occurred;
- canonical identity and public inspection are derived from different
  representations of the operation;
- reconciliation affects canonical identity but cannot be recovered from the
  installed contract as a typed value;
- the aftermath external-effect posture discards or stringly reinterprets the
  operation's correlation family; or
- a facade consumer must import declaration, execution, Relational, or private
  installation modules to inspect the contract.

## Current Boundary

### Preserved foundations

The following existing boundaries are correct and remain authoritative:

- `ApplicationOperationDecisionReadTarget` and
  `ApplicationOperationProgramTarget` retain the operation's typed declaration
  closure through compilation.
- `WorthQueryOperationNativeProjectionContract` already retains a real
  Foundational `AspectContract`, an admitted `AspectMask<ProjectionMask>`, and
  their canonical material.
- every installed application operation already owns one sealed graph-
  obligation set and one managed provider-session progression.
- Relational owns authoritative graph state, actual projection and traversal,
  committed mutation interpretation, and exact performed touched-scope
  evidence.
- installation already validates application schema closure, operation target
  closure, aftermath axis compatibility, pre-image coverage, and external-
  effect posture.
- `WorthQueryInstalledAftermathContract` already exposes correction authority,
  correction mechanism, recovery posture, canonical artifact, next actions,
  and published posture.
- `InstalledExternalEffectContract` already retains the operation's external-
  effect correlation family and protocol.
- `worth_query_host::facade::domain` is the stable inspection audience and
  re-exports installation-owned domain meaning without owning it.

### Defects repaired here

The current application-operation compiler creates the primary graph-read role
with an unconditional empty `semantic_reads` vector. It therefore loses the
typed aspect contract and projection-mask evidence already promised by the
operation's decision reads.

The current touch contract lowers `ApplicationOperationProgramTarget` into
structured `String` values. That representation combines operation kind and
semantic components in a private grammar and includes `Emit` beside actual
graph mutation targets.

The native application `AspectContract`, `AspectIdentity`, and revision are
currently produced later by primary-graph execution lowering. Installation
cannot honestly construct the promised native projection contract without
duplicating that lowering or depending backward on execution.

The installed aftermath canonical basis includes the declared reconciliation
procedure, but `WorthQueryInstalledAftermathContract` does not retain and
expose the installed procedure. Its derived external-effect posture retains a
correlation-family `String` but exposes only `is_declared()`.

### Corrected interpretation of the incoming report

The public aftermath surface is incomplete, not absent. Authority, mechanism,
recovery, next actions, canonical artifact, and published posture already have
typed installed accessors. The operation-level external-effect contract
already exposes its correlation family. This milestone adds only the missing
typed retained meaning and makes the operation and aftermath views share one
identity type; it does not create duplicate getters or a second aftermath
contract.

## Ownership And Truth Lock

| Meaning or product | Authoritative owner | Constructed at | Consumed by | Cannot authorize |
|---|---|---|---|---|
| Application aspect semantic identity and contract revision | Query declaration | Typed aspect declaration | Installation schema compilation and 9.16.2 package export | A graph read, write, or runtime registration by itself |
| Installed native application aspect contract catalog | Query installation | Exact schema installation | Operation compilation, graph obligations, execution schema lowering, Host inspection | Relational mutation or runtime execution by inspection alone |
| Installed operation graph-read scopes | Query installation | Operation compilation against the installed schema catalog | Obligation identity/selection, admission, execution comparison, Host inspection | Additional reads beyond the admitted operation and session |
| Installed operation declared touch scopes | Query installation | Operation compilation against the same catalog | Obligation identity/selection, mutation admission, performed-touch comparison, Host inspection | Proof that a mutation happened |
| Actual reads and performed touched scope | Relational, carried through the Query session | Runtime owner execution | Decision read-set, invalidation, invariant and terminal evidence | Widening the installed operation ceiling |
| External-effect contract and correlation family | Query declaration and installation | Operation contract installation | Outbox/dispatch admission, aftermath projection, Host inspection | External completion or retry authority |
| Installed reconciliation procedure | Query installation from declared aftermath meaning | Aftermath installation | Correction admission, package export, Host inspection | Execution of reconciliation by inspection alone |
| Canonical installed artifacts | Query installation using Foundational canonical basis | Cold installation | Identity comparison, package export, inspection | Reconstruction from digest or rendered text |
| Execution schema layout | Query execution, derived from installed contracts and Relational registration | Runtime installation | Primary-graph execution | A competing application aspect contract |

Foundational continues to own the native contract, mask, field-path,
admissibility, canonical-basis, and digest mechanics. Query must carry those
types directly and must not create mirror representations.

Application aspect identities and revisions are stable declaration meaning.
Provider idempotency, dispatch-outbox, and aftermath-causality aspects remain
execution-owned runtime schema because they describe provider storage and have
a different lifecycle, authority, and replacement fate. They do not move into
the application schema catalog.

## Adversarial Courtroom

### Production boundary

The decisive court is a normal external `worth-query-host` consumer. It
declares and installs an application schema and operations through
`worth_query_decl::facade`, inspects them through
`worth_query_host::facade::domain`, executes through the ordinary host runtime,
and observes authoritative Relational results only through the existing Query
terminal and publication surfaces.

The consumer has no dependency on the `worth-query` monolith,
`worth-query-execution`, `worth-relational`, `worth-runtime-bridge`,
`worth-signal`, or private installation modules.

### Hostile schema and operation world

The schema contains:

- two entity kinds with the same local aspect name and same local field name;
- one aspect containing at least three fields, including two fields with the
  same scalar family;
- a second aspect on the same entity;
- two relations with the same endpoint entity kinds;
- one mutation operation that reads entity existence, exactly one field path,
  and one relation, then writes a sibling field and links the other relation;
- one read-only operation that reads exactly one field;
- one operation that emits an external effect but performs no graph mutation;
  and
- one external-owner aftermath contract with a reconciliation procedure and
  correlation family.

The same logical declaration is installed twice with deliberately permuted
member and operation declaration order. A hostile twin changes exactly one
field read, one touch variant, one aspect revision, one correlation family, or
one reconciliation procedure at a time. A cross-splice twin supplies
same-rendered-name loci from the other entity, schema binding, or runtime
generation.

The scale twin adds 128 unrelated fields and 4,096 unrelated operations while
selecting and executing the same subject operation.

### Required observations

The public Host inspection must show:

- one typed entity read scope;
- one schema-bound native projection whose `AspectContract` is the catalog's
  exact contract and whose mask contains exactly the declared field path;
- one typed relation read scope with exact endpoint kinds;
- one typed sibling-field write scope and one typed link scope;
- no graph touch for the emit-only operation;
- an external-effect contract for the emit-only operation;
- the exact installed reconciliation procedure; and
- the exact typed correlation family shared by the operation external-effect
  and aftermath views.

Independent runtime evidence must show that execution registered the same
application aspect contract identity and revision, observed the declared read
facts, performed only admitted mutations, and produced actual touched-scope
evidence that is a subset of the installed declared scopes.

Equivalent declaration order must produce equal installed schema catalogs,
operation contracts, obligation identities, and canonical artifacts. Every
single-semantic mutant must change the relevant canonical artifact and either
install as a distinct compatible package or deny under the existing
compatibility law. Cross-schema, cross-entity, and cross-generation
substitutions must deny before graph work.

Warm selection and execution in the scale twin must perform no application
contract reconstruction, declaration scan, canonical encoding, SHA work, or
digest-text materialization. Work may scale with selected read scopes, selected
touch scopes, actual traversal, and actual mutation only.

### Mutation sensitivity

The court must turn red if implementation:

- restores the empty read vector;
- maps every field read to a whole-aspect mask;
- drops schema or entity binding from a native projection;
- assigns application aspect identities inside execution;
- compares a touch through formatted text;
- treats `Emit` as a graph mutation;
- accepts declared touch scope as performed evidence;
- canonicalizes a sorted string rendering instead of the typed variants;
- hashes reconciliation without retaining it;
- copies correlation-family text into a second untyped authority lane;
- scans unrelated schema members or operations during warm selection; or
- exposes a public constructor that can insert a forged scope into an
  installed operation or obligation set.

The proof must include direct semantic assertions against independent owner
evidence. Getter-only tests, snapshots produced by the same formatter, and an
in-memory imitation of the Host boundary are insufficient.

## Product Decision Lock

### Stable application aspect identity

Application aspect identity and contract revision move to the declaration
surface because they are stable semantic meaning that crosses runtimes.
`ApplicationAspectMarkerIdentity` gains the exact Foundational
`AspectIdentity` and `AspectContractRevision` associated with the aspect.

The ordinary macro makes those values explicit:

```rust
worth_query_aspect!(
    pub AccountFacts in BankingSchema, Account;
    identity = AspectIdentity(0x9161_1001),
    revision = AspectContractRevision(1),
);
```

The macro remains the only ordinary constructor of a typed application aspect
reference. Installation denies duplicate aspect identities within one schema,
an identity reused for incompatible aspect meaning, revision zero, missing
field closure, or an aspect contract whose declared shape cannot be admitted
by Foundational.

The identity namespace is one exact application-schema binding. Two different
owner/schema bindings may use the same `AspectIdentity`. Within one erased
schema, however, an identity may belong to only one `(entity, aspect)` locus;
reuse at any second locus is incompatible regardless of revision or shape.
Across generations, full schema declaration equality and canonical identity
govern compatibility, so changing identity, revision, field shape, presence,
or scalar family changes installed schema meaning.

The old macro form without identity and revision is retired when covered
consumers migrate. Installation may not synthesize a fallback identity,
derive one from `type_name`, truncate a digest, or allocate one by declaration
or sort order. Source compatibility does not outrank stable semantic identity.

### Installed native schema catalog

Installation creates exactly one
`WorthQueryInstalledApplicationSchemaContractCatalog` for each installed
schema binding. It is immutable, canonical-order independent, and contains one
`WorthQueryInstalledApplicationAspectContract` per declared `(entity, aspect)`
pair.

The installed package index compiles and stores that catalog exactly once per
unique `(owner, schema)` record during each index build, rebuild, or successor
generation. The stored record owns the erased declaration and an immutable
shared catalog. `bind_application_schema` compares the supplied typed
declaration and shares the stored catalog with the typed handle; binding,
lookup, and inspection never compile or canonicalize it again.

Each entry retains:

- the exact application schema binding identity and compatibility generation;
- validated entity and aspect semantic keys;
- the Foundational `AspectContract` built from the declared identity,
  revision, field shape, presence, scalar family, absence, and evolution law;
- exact typed field-key membership;
- canonical contract material prepared once at installation; and
- the Relational semantic binding required to register the contract.

Catalog lookup is bounded indexed lookup by schema-bound entity/aspect locus.
The catalog has no public constructor or mutation method. A canonical digest
is derived compression and cannot substitute for the retained contract or
schema binding.

Catalog compilation is cold installed-index work with explicit catalog,
contract, field, and prepared-basis counters. Aggregate canonical preparation
is admitted under a checked constant-factor entry/byte budget derived from the
already admitted schema canonical-work bound. Overflow or budget exhaustion
denies index construction before graph work.

### Typed graph-read scopes

`WorthQueryOperationGraphReadRole` replaces its native-only
`semantic_reads` collection with a closed collection of
`WorthQueryOperationGraphReadScope` values:

```rust
pub enum WorthQueryOperationGraphReadScope {
    Entity(WorthQueryOperationEntityReadScope),
    NativeProjection(WorthQueryOperationApplicationProjectionScope),
    Relation(WorthQueryOperationRelationReadScope),
}
```

An application projection scope contains the schema-bound entity/aspect locus
and one `WorthQueryOperationNativeProjectionContract` built from the exact
catalog contract. Field decision reads for the same entity/aspect are grouped
into one projection mask containing their exact Foundational field paths.

Entity decision reads remain entity-existence scopes. Relation decision reads
remain relation scopes with exact declared endpoint kinds. Neither is encoded
as an aspect projection. A whole-aspect projection exists only when the typed
declaration explicitly names that posture; an entity read or empty mask does
not imply it. This milestone does not add a new whole-aspect application read
feature: current field-derived masks are explicitly partial and report
`is_whole_aspect() == false`. A later whole-aspect authoring feature must add a
named typed declaration variant before installation may emit that posture.

Read-scope fields are private. Public accessors support exhaustive typed
matching and return borrowed Foundational contracts and masks. Public callers
cannot construct an installed scope or add it to an installed contract.

### Typed declared touch scopes

`WorthQueryOperationTouchContract::Declared` carries
`Vec<WorthQueryOperationTouchScope>` rather than `Vec<String>`:

```rust
pub enum WorthQueryOperationTouchScope {
    CreateEntity(WorthQueryOperationEntityTouchScope),
    DeleteEntity(WorthQueryOperationEntityTouchScope),
    WriteField(WorthQueryOperationFieldTouchScope),
    LinkRelation(WorthQueryOperationRelationTouchScope),
    UnlinkRelation(WorthQueryOperationRelationTouchScope),
    DeclaredDomain(WorthQueryDeclaredDomainTouchScopeIdentity),
}
```

Application operations compile only to the first five structured variants.
The `DeclaredDomain` variant preserves generic installed domain-operation
scope as one validated atomic semantic identity. Its representation contains
no separators or parseable substructure and cannot impersonate an application
entity, field, or relation locus.

`ApplicationOperationProgramTarget::Emit` compiles into the operation's effect
and external-effect contracts, never into a graph touch. `NotRequired` is used
when an operation has no graph mutation targets even if it emits an external
effect. Mutation effect posture is derived from graph mutation targets; an
external-effect family is derived independently from the installed external-
effect contract.

Declared touch scope is the installed legal ceiling. Actual Relational touched
scope is performed evidence. Query compares them through typed schema binding,
entity/aspect identity, field paths, relation identity, and operation/session
affinity. It never promotes the declaration to performed evidence.

### Read/write duality and invalidation

The installed contract index computes overlap without strings:

- a native projection intersects a field write only when schema binding,
  entity locus, aspect contract identity/revision, and field path overlap;
- a whole-aspect projection intersects any field touch within that exact
  bound aspect;
- entity-existence scope intersects the exact entity structural mutation
  family defined by the installed operation contract;
- relation scope intersects link or unlink only for the exact schema-bound
  relation and endpoint contract; and
- external-effect emission has no graph-read intersection.

Installation computes and retains the comparison-ready typed index once.
Admission and execution consume it. They do not rebuild masks, walk schema
members, or parse canonical material.

### Canonical contract basis

Canonical encoding writes a closed variant discriminant followed by each
typed component's canonical basis. Ordering and deduplication operate on typed
values before encoding. Rendered strings are diagnostics only and are never
canonical input when a retained typed value exists.

The installed schema catalog, operation contracts, and graph-obligation set
must agree on the same canonical read and touch material. Equivalent order
produces equal artifacts. Any semantic field, mask posture, identity, revision,
scope variant, endpoint, correlation family, or reconciliation change changes
the appropriate canonical basis.

Canonical work is cold installation/package work. Ordinary selection,
admission, execution, publication, and inspection carry retained fixed-width
identity and typed contract references and perform exact-zero canonical/SHA
work.

### Complete installed aftermath inspection

Declaration introduces a validated
`WorthQueryExternalEffectCorrelationFamily` atomic identity. It may render as
text but exposes no private grammar. The operation external-effect contract,
aftermath external-effect posture, outbox binding, dispatch admission, and
package record carry this exact type rather than independent `String` copies.

Installation creates a
`WorthQueryInstalledReconciliationProcedure` from the declared procedure and
retains it in `WorthQueryInstalledAftermathContract`. The canonical aftermath
basis consumes that retained installed value; it does not hash the portable
declaration through a separate path.

The stable inspection surface includes:

```rust
aftermath.authority();
aftermath.mechanism();
aftermath.recovery();
aftermath.reconciliation();
aftermath.external_effect().correlation_family();
aftermath.canonical();
```

The accessors return borrowed or copyable typed values and grant no correction,
dispatch, recovery, or execution authority. The types are re-exported through
`worth_query_host::facade::domain`; Host contains no duplicate representation
or translation logic.

## Compiler-Visible Progression

Installation progression is compiler-visible:

```text
validated application schema declaration
    -> WorthQueryInstalledApplicationSchemaContractCatalog
    -> operation compilation against &catalog
    -> WorthQueryCompiledApplicationOperationContracts
    -> WorthQueryInstalledGraphObligationSet
```

Execution progression consumes the same authority one way:

```text
&WorthQueryInstalledApplicationSchemaContractCatalog
    -> Relational application-contract registration
    -> derived WorthQueryPrimaryGraphLayout
    -> admitted operation + exact provider session
    -> performed read/touch evidence
    -> typed subset/intersection comparison
    -> terminal
```

No API accepts an unbound scope vector, raw `AspectContract`, caller-selected
mask, scope string, correlation-family string, reconciliation slot, or digest
as a substitute for an installed operation.

The derived primary-graph layout may cache Relational kind IDs, field locators,
and index IDs. Those runtime values are not portable meaning and cannot mint or
modify the installed catalog. Rebuilding the layout from the same installed
catalog must retain exact application aspect contracts.

## Destination Directory And Module Skeleton

Status markers:

- `[E]` existing and retained;
- `[C]` created;
- `[M]` existing responsibility moved to the named destination;
- `[R]` existing file replaced after its responsibilities move;
- `[X]` removed after cutover; and
- `[S]` committed successor destination documented here but not created empty.

```text
workspaces/worth-query/crates/
├── worth-query-declaration/src/
│   ├── application_schema_macro.rs                         [E, narrow]
│   ├── application_aspect_macro.rs                         [C]
│   ├── application_schema/
│   │   ├── aspect_contract_identity.rs                    [C]
│   │   ├── canonical_identity.rs                          [E, narrow]
│   │   ├── canonical_identity/member.rs                   [C]
│   │   ├── declaration.rs                                 [E, modify]
│   │   ├── external_effect_correlation_family.rs          [C]
│   │   ├── field_reference.rs                             [E, narrow]
│   │   ├── references.rs                                  [E, modify]
│   │   └── schema_member.rs                               [E, modify]
├── worth-query-installation/src/
│   ├── application_schema.rs                              [E, facade]
│   ├── application_schema/
│   │   ├── installed.rs                                   [C]
│   │   ├── compilation.rs                                 [E, orchestrator]
│   │   └── native_contract/
│   │       ├── mod.rs                                     [C]
│   │       ├── catalog.rs                                 [C]
│   │       ├── aspect_contract.rs                         [C]
│   │       ├── compilation.rs                             [C]
│   │       ├── locus.rs                                   [C]
│   │       ├── denial.rs                                  [C]
│   │       └── canonical_basis.rs                         [C]
│   ├── installed_index/
│   │   ├── application_schema_record.rs                   [C]
│   │   ├── application_schema.rs                          [E, facade]
│   │   ├── application_schema/binding.rs                  [C]
│   │   ├── application_schema/validation.rs               [C]
│   │   ├── application_schema/denial_mapping.rs           [C]
│   │   └── construction/application_schema_records.rs     [C]
│   ├── application_operation.rs                           [E, facade]
│   ├── application_operation/
│   │   ├── contracts.rs                                   [R]
│   │   └── contracts/
│   │       ├── mod.rs                                     [C]
│   │       ├── compilation.rs                             [M]
│   │       ├── graph_reads.rs                             [C]
│   │       ├── graph_touches.rs                           [C]
│   │       └── effect_posture.rs                          [C]
│   ├── domain_operation.rs                                [E, facade]
│   ├── domain_operation/
│   │   ├── semantic_contracts.rs                          [R]
│   │   ├── graph_read/
│   │   │   ├── mod.rs                                     [C]
│   │   │   ├── contract.rs                                [M]
│   │   │   ├── role.rs                                    [M]
│   │   │   ├── scope.rs                                   [C]
│   │   │   └── native_projection.rs                       [M]
│   │   └── touch/
│   │       ├── mod.rs                                     [C]
│   │       ├── contract.rs                                [M]
│   │       ├── scope.rs                                   [C]
│   │       └── overlap.rs                                 [C]
│   ├── graph_obligation/
│   │   ├── contract.rs                                    [E, modify]
│   │   ├── identity/contract_encoding.rs                  [E, modify]
│   │   └── operation_binding.rs                           [E, modify]
│   ├── application_aftermath/
│   │   ├── install.rs                                     [E, modify]
│   │   ├── reconciliation.rs                              [C]
│   │   ├── external_effect_contract.rs                    [E, modify]
│   │   └── canonical_basis.rs                             [E, modify]
│   └── package/
│       └── portable_records/                              [S, 9.16.2 consumer]
├── worth-query-execution/src/domain_computation/primary_graph/
│   ├── schema_layout.rs                                   [E, modify]
│   └── schema_layout/
│       ├── registry_lowering.rs                           [E, narrow]
│       ├── provider_identity_allocator.rs                 [C]
│       ├── provider_idempotency.rs                        [E, retained]
│       ├── provider_dispatch_outbox.rs                    [E, retained]
│       └── provider_aftermath_causality.rs                [E, retained]
├── worth-query-host/src/facade.rs                         [E, re-export only]
└── worth-query-package-archive/src/                       [S, 9.16.2 consumer]
```

The dominant axes are:

- declaration owns stable application semantic identity and authoring;
- installation schema owns cold compilation of authoritative installed native
  contracts;
- installation domain-operation owns reusable typed read/touch vocabulary;
- installation application-operation owns lowering from application targets
  into that vocabulary;
- graph-obligation owns sealed selection and canonical binding, not scope
  invention;
- aftermath owns installed correction/effect inspection;
- execution schema layout owns volatile Relational mechanics derived from the
  installed catalog; and
- Host owns audience re-export only.

Forbidden placements include adding the catalog to execution, putting
Relational types in declaration or installation, adding application-specific
scope parsing to graph obligation, retaining new logic in a Host facade,
placing effect emission in graph touch, or combining read, touch, aftermath,
and canonical encoding back into one `contracts.rs` bag.

The `contracts.rs` and `semantic_contracts.rs` replacements are required
composition work, not optional cleanup: both files are already near the
400-line cap and currently combine responsibilities that acquire distinct
authority and proof obligations in this milestone.

## Ordered Phase Plan

### Phase 1: Stable Aspect Meaning And Installed Native Catalog

Add declaration-owned `AspectIdentity` and `AspectContractRevision`, migrate
every covered application aspect declaration, and compile the exact installed
native application-schema catalog. Validate identity/revision uniqueness,
shape closure, field membership, and Foundational mask/contract admission.

Move only application aspect-contract construction out of execution. Keep
provider-internal schema aspects with their execution owner. Installation
order-equivalence, semantic-mutant, collision, and compile-fail evidence lets
Phase 2 trust one exact native contract per installed entity/aspect locus.

This phase also performs the minimal atomic execution cutover required to make
that move honest: primary-graph bootstrap passes the sealed catalog into
layout lowering, Relational registers its exact application contracts, and
execution deletes application identity allocation, revision choice, field
shape reconstruction, and application member scans. Phase 3 retains
performed-read/touch comparison and the remaining operation/capability
consumer cutover; it does not defer this registration handoff.

Provider aspects allocate three deterministic runtime-local identities above
the maximum installed application identity. An empty catalog uses maximum
zero and therefore provider identities `1..=3`. Checked exhaustion denies
before Relational installation; `u64::MAX - 3` is the last successful
application maximum and `u64::MAX - 2` or greater denies.

This phase mechanically forbids synthesized application aspect identities,
hard-coded application revisions, `type_name` identity, digest truncation, and
execution-owned application contract construction.

### Phase 2: Typed Read And Touch Contract Compilation

Compile decision reads against the Phase 1 catalog into typed entity, native
projection, and relation scopes. Compile graph mutation program targets into
typed create/delete/write/link/unlink scopes. Split effect emission from graph
touch and derive mutation/effect posture from the correct target families.

Bind the typed values into graph-obligation identity and selection. Add the
comparison-ready overlap index and remove the structured-string scope
function. Exact-mask, same-name-cross-locus, emit-only, canonical-order,
semantic-mutant, and public-construction evidence lets Phase 3 trust that the
sealed obligation exposes the same exact meaning as the installed operation.

This phase mechanically forbids empty installed read scope for a non-empty
decision-read declaration, string parsing, inferred whole-aspect posture, and
declared-scope promotion to performed evidence.

### Phase 3: Execution Consumption And Performed-Evidence Closure

Consume the Phase 1 registered application contracts and compare runtime
read/touch evidence against the Phase 2 typed contract. Cut over every
remaining ordinary operation and capability-operation consumer atomically.

Real owner-bound execution, layout reconstruction, cross-generation
substitution, exact subset/intersection, sibling-field precision, relation
endpoint, cancellation, conflict, and lifecycle evidence lets Phase 4 trust
that inspection and runtime behavior share one installed authority.

Application `AspectIdentity` allocation and application `AspectContract`
construction were already removed from execution in Phase 1. This phase may
not restore either through a private compatibility cache or parallel old
comparison lane.

### Phase 4: Complete Aftermath And Host Inspection Surface

Introduce the typed correlation-family identity, retain the exact installed
reconciliation procedure, derive aftermath external-effect posture from the
operation's installed external-effect contract, and expose the complete typed
surface through `worth_query_host::facade::domain`.

Canonical identity, external-owner, not-correctable, no-effect, wrong-family,
wrong-reconciliation, compile-fail construction, and public Host consumer
evidence lets Phase 5 trust that package and NCR consumers need no private
imports or string interpretation.

This phase does not add correction execution, durable recovery, or external-
effect completion authority.

### Phase 5: Courtroom, Cutover, Documentation, And Residue

Run the complete public Host courtroom, scale twin, full affected consumer
parity, dependency enforcement, line-cap audit, and warm-work evidence. Remove
the old aspect macro form, structured scope rendering as authority, application
contract reconstruction, public untyped scope exposure, and any compatibility
re-export that can preserve the defective contract.

Revise the durable feature documents and append the 9.16.1 closure finding.
Only after all acceptance evidence closes may Milestone 9.16.2 begin and trust
the repaired installed contract for stable package records and NCR adoption.

## Caller DX Target

Application authors declare stable aspect meaning and continue to declare
operation reads and writes through typed schema references:

```rust
worth_query_aspect!(
    pub AccountFacts in BankingSchema, Account;
    identity = AspectIdentity(0x9161_1001),
    revision = AspectContractRevision(1),
);

schema
    .operation_read(operation, AccountStatus::reference())
    .operation_write(operation, AccountBalance::reference());
```

A Host consumer inspects the installed contract without importing owners or
reconstructing meaning:

```rust
use worth_query_host::facade::domain::{
    WorthQueryOperationGraphReadScope,
    WorthQueryOperationTouchScope,
};

let contracts = installed_operation.contracts();

let projection = contracts.graph_reads().roles()[0]
    .read_scopes()
    .iter()
    .find_map(|scope| match scope {
        WorthQueryOperationGraphReadScope::NativeProjection(scope) => Some(scope),
        _ => None,
    })
    .expect("installed native projection");

assert_eq!(
    projection.entity().semantic_key(),
    Account::reference().semantic_key(),
);
assert!(
    projection
        .projection()
        .mask()
        .selects(AccountStatus::reference().field_path()),
);
assert!(!projection.projection().mask().is_whole_aspect());

assert!(contracts
    .touches()
    .scopes()
    .iter()
    .any(|scope| matches!(scope, WorthQueryOperationTouchScope::WriteField(_))));

let aftermath = installed_operation.aftermath().expect("declared aftermath");
let procedure = aftermath.reconciliation().expect("external-owner procedure");
let family = aftermath
    .external_effect()
    .correlation_family()
    .expect("declared external lane");
```

Exact method names may align with the destination types during planning, but
the semantic operations, typed matching, and absence of caller-authored strings
are locked. Convenience predicates may supplement, not replace, access to the
real Foundational contract, mask, and typed scope variants.

## Work And Performance Contract

- Native schema-contract compilation, mask preparation, canonical encoding,
  and obligation identity are bounded cold installation work.
- Catalog construction is proportional to the installed schema's aspects and
  fields and is performed once per installation attempt.
- Operation contract compilation is proportional to that operation's declared
  decision reads and program targets, with indexed schema-locus lookup.
- Equivalent read fields for one aspect are grouped without scanning unrelated
  operations.
- Warm obligation selection is bounded by the selected operation's fixed index
  and selected scope counts, not total installed operations or schema members.
- Execution registration iterates the installed schema catalog once during
  runtime installation; ordinary execution performs no contract reconstruction.
- Typed read/touch intersection is proportional to the selected operation's
  scopes and actual performed evidence, not unrelated graph population.
- Host inspection borrows retained contracts and performs no canonicalization,
  SHA, registry scan, or digest-text materialization.
- Package export in 9.16.2 may encode retained canonical artifacts as cold work
  but may not rebuild them from declaration strings.

## Documentation Deliverables

Implementation closure must revise:

- `workspaces/worth-query/crates/worth-query/docs/domain-capabilities/canonical-graph-obligation-progression.md`
  for the exact installed read/touch contract, declared-versus-performed touch
  distinction, and installation-to-execution catalog flow;
- `workspaces/worth-query/crates/worth-query/docs/authoring/graph-touch-obligation-authority.md`
  for typed scope variants, effect separation, and the prohibition on string
  grammar;
- `workspaces/worth-query/crates/worth-query/docs/modeling/aspects-and-authority-lanes.md`
  for declaration-owned application aspect identity/revision and execution-
  owned provider schema separation;
- `workspaces/worth-query/crates/worth-query/docs/execution/application-aftermath-and-recovery.md`
  for complete installed reconciliation and correlation-family inspection;
- `workspaces/worth-query/crates/worth-query/docs/AI_README.md` for the installed
  schema catalog and public Host inspection path;
- the declaration and Host crate-level documentation for the new aspect macro
  form and read-only inspection examples; and
- Milestone 9.16.2 package and NCR language so it consumes these retained typed
  contracts and does not create package-local mirrors.

All public examples compile or run through their ordinary documentation gates.
No new guide duplicates the canonical graph-obligation document.

## Must Ship

- declaration-owned stable application aspect identity and revision;
- one sealed installed native application-schema contract catalog;
- exact typed entity, native projection, and relation read scopes;
- exact typed create, delete, field-write, relation-link, and relation-unlink
  declared touch scopes;
- exact Foundational projection masks with explicit whole-aspect posture;
- graph mutation separated from external-effect emission;
- one typed read/touch overlap path and one canonical contract basis;
- execution registration from the exact installed application contracts;
- actual Relational touched evidence kept distinct from declared scope;
- typed retained reconciliation and correlation-family inspection;
- complete `worth_query_host::facade::domain` re-export;
- exact-zero warm contract reconstruction and canonical/SHA work;
- public hostile-consumer, independent owner, scale, compile-fail, dependency,
  facade, residue, and mutation-sensitive evidence; and
- the append-only roadmap, closure-ledger, durable-documentation, and 9.16.2
  handoff updates.

## Must Preserve

- Milestone 9.16.1's single obligation, admission, provider-session, owner-
  execution, terminal, publication, and lifecycle progression;
- all recorded historical milestone and phase statuses;
- Relational ownership of graph truth, exact performed touches, and commit;
- Foundational ownership of aspect contract, identity, revision, mask,
  admissibility, canonical basis, and digest mechanics;
- Query declaration ownership of portable application meaning;
- Query installation ownership of installed schema, operation, obligation, and
  aftermath meaning;
- Runtime Bridge and Signal ownership boundaries and Query-independent package
  direction;
- Query execution ownership of runtime sessions, provider-internal schema,
  actual work, and volatile layouts;
- read-only, mutation, capability, conditional, aftermath, external-effect,
  recovery, publication, and cert-only replay behavior not contradicted here;
- typed branch, schema binding, runtime generation, plan, session, and basis
  affinity;
- ordinary public declaration and Host composition roots; and
- exact lifecycle release and existing warm-path bounds.

## Explicit Non-Goals

- PostgreSQL, durability, restart recovery, release archives, or package
  persistence;
- NCR workflow rules, UI, notifications, or acceptance behavior;
- multiple branches, branch-local MVCC, merge, rebase, or durable composite
  history;
- correction, compensation, or reconciliation execution;
- a durable recovery handle or external-effect completion decision;
- changing Relational performed-touch semantics or Signal invalidation
  authority;
- moving provider idempotency, dispatch-outbox, or aftermath-causality schema
  into application declaration or installation;
- replacing Foundational aspect, mask, path, canonical, or denial types with
  Query mirrors;
- adding new whole-aspect application read behavior beyond exposing the exact
  partial-versus-whole posture of the installed Foundational mask;
- generic crate decomposition beyond the files whose responsibilities must
  separate for this repair; or
- preserving source compatibility for the defective aspect macro or structured
  string touch surface after covered consumers migrate.

## Acceptance Evidence

Milestone 9.16.1.1 closes only when:

1. every installed application aspect carries declaration-owned identity and
   revision, and installation denies collision, incompatibility, missing
   closure, and invalid revision;
2. one exact installed native application-schema catalog supplies operation
   projection contracts, graph obligations, execution registration, Host
   inspection, and the 9.16.2 package handoff;
3. every operation decision-read target appears as the correct typed entity,
   native projection, or relation scope with no widening or omission;
4. field reads sharing an aspect produce one exact mask, sibling-field and
   whole-aspect behavior are distinguished, and same-name foreign loci deny;
5. graph mutation targets appear as typed touch variants, emit-only operations
   declare no graph touch, and no structured scope string or parser remains;
6. installed declared touch scope and actual performed touch evidence remain
   distinct and are compared through exact typed binding;
7. application aspect contracts are no longer constructed or assigned identity
   in execution, while provider-internal execution schema remains intact;
8. installed schema, operation, and graph-obligation canonical artifacts agree,
   order permutations remain equal, and every single-semantic mutant changes
   or denies at the correct boundary;
9. reconciliation and typed correlation family are retained, canonical, and
   inspectable through `worth_query_host::facade::domain` without granting
   correction, dispatch, or recovery authority;
10. public construction, raw-scope insertion, cross-schema, cross-entity,
    cross-generation, wrong-revision, wrong-family, and wrong-procedure
    substitutions fail mechanically or with typed denial before owner work;
11. the real Host courtroom executes through the ordinary composition root and
    independently confirms the exact Relational contracts, reads, mutations,
    and touched evidence;
12. the 128-field and 4,096-operation scale twin preserves bounded indexed
    selection and exact-zero warm reconstruction, canonicalization, SHA, and
    digest-text work;
13. deletion or inversion of the disputed read, touch, identity, effect, or
    aftermath mechanism turns targeted evidence red;
14. every covered consumer uses the new aspect declaration and typed public
    contracts, and no compatibility representation, parser, old macro form, or
    private facade import remains;
15. the canonical product docs, AI README, declaration docs, Host docs,
    roadmap, 9.16.1 post-closure finding, and 9.16.2 handoff agree with the real
    implementation and their public examples compile or run; and
16. focused owner tests, affected integration and consumer suites, formatting,
    dirty line-cap enforcement, boundary check, generated context check,
    dependency/residue checks, strict Clippy, and workspace lanes proportional
    to the touched boundary all pass.

## Successor Handoff

Milestone 9.16.2 may trust:

- stable declaration-owned application aspect identity and revision;
- one complete installed native schema catalog;
- complete typed application-operation read, touch, external-effect, and
  aftermath meaning;
- canonical installed artifacts that package export may carry but not
  reconstruct;
- public Host inspection sufficient for the NCR consumer; and
- execution that consumes the same contracts package reconstruction will
  readmit.

Milestone 9.16.2 Phase 1 extends stable identity to every remaining package-
relevant Rust type axis. Phase 2 exports the retained contracts as typed
records. Phase 3 reconstructs candidates and asks Query to rebuild and compare
the same installed catalog and operation contracts under fresh validation.
Neither phase may introduce package-local aspect identities, string touch
scopes, correlation-family text authority, reconciliation summaries, or an NCR
adapter contract.

Milestones 9.17 and 9.18 may add durable owner bases, composite publication,
and reconciliation execution around these contracts. They inherit the typed
installed meaning and must not move its facade, make SQL authoritative, or
reinterpret descriptive aftermath inspection as correction authority.
