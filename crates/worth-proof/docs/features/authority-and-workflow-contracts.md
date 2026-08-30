# Authority And Workflow Contracts

worth-proof supplies proof-bearing values and progression vocabulary. The
operation that owns a sensitive decision must still choose the exact authority,
capability, or proof type that can open it.

That division is deliberate:

- worth-proof owns the typed carriers and progression laws;
- the domain workflow owns the concrete marker types and their issuers;
- the protected operation accepts those concrete types, not a generic marker;
- downstream contracts prove both the legitimate and counterfeit caller paths.

## Place the owner seal before designing the facade

For a governed public operation, place the concrete marker and issuer in the
domain owner that decides whether the operation is legal. The owner may use
`worth-proof` witness, proof, binding, and transition carriers because Proof is
the progression substrate. Proof must not define a Relational, Signal, Bridge,
or Query owner marker on that domain's behalf: doing so would move the trust
decision beneath the component that owns its live identity and policy.

The protected signature names the concrete owner type:

~~~rust
fn publish(candidate: Candidate<RelationalPublicationAuthority>) {
    // only the Relational owner can have produced this exact candidate
}
~~~

This generic alternative is not a governed boundary:

~~~rust,compile_fail
fn publish<A: AuthorityMarker>(candidate: Candidate<A>) {
    // any caller can define A, so the signature has delegated its authority
}
~~~

Open marker traits remain useful substrate for caller-owned workflows. They are
not a seal shared by every domain. If a marker requires a clock, live table,
counter, retention obligation, or `Drop` behavior to mean anything, that state
belongs in the owning runtime artifact rather than in `worth-proof`.

## The Authority Boundary

An authority-bearing operation should name its concrete requirements:

~~~rust
pub fn admit(
    authority: AuthorityWitness<EntryAdmission>,
    capability: CapabilityWitness<EntryExecution>,
    eligibility: Proof<AdmissionEligible, EntryAdmission>,
) {
    // governed work
}
~~~

The owning workflow issues those values:

~~~rust
let authority = issue_entry_admission();
let capability = issue_entry_execution();
let eligibility = issue_eligibility(&authority);

admit(authority, capability, eligibility);
~~~

A caller-defined marker can legitimately use the open worth-proof substrate. It
cannot satisfy this operation because its resulting witness or proof has a
different concrete type:

~~~rust,compile_fail
struct CounterfeitAdmission;
impl AuthorityMarker for CounterfeitAdmission {}

let counterfeit =
    AuthorityWitness::from_authority_marker(CounterfeitAdmission);
admit_authority_only(counterfeit);
~~~

This type mismatch is the main authority guarantee. Private marker construction
is an additional guarantee where the workflow relies on possession of the
marker value to mint the exact witness.

## Authority Boundary Catalog

The maintained catalog is intentionally small and operation-centered.

| Contract | Legitimate caller | Counterfeit caller | Direct construction |
| --- | --- | --- | --- |
| concrete authority, capability, and proof ceremony | owner issuers compile and call admit | local structurally similar markers fail at the protected calls | value-gated marker literal fails |
| sealed worth-proof witness and proof values | owner macros and checked transitions compile | wrong authority scope and unproven proof kinds fail | witness, proof, and sealed marker minting fail |

The concrete ceremony lives in
tools/boundary-check/tests/authority_sealing_contracts/forgery.rs. Its
construction pressure lives beside it in value_gate_forgery.rs. The substrate
construction and authority-scope cases live under tests/ui/milestone1/ and
tests/ui/milestone2/.

The catalog is not an inventory of every public Rust type. Public data types
may be introduced, inspected, or composed in different ways without becoming
authority boundaries. What matters here is whether a caller can open a
protected operation without the exact owner-issued values it requires.

## Supported Workflow Catalog

The public workflow contract is bounded to representative caller goals:

1. checked DisjointPair construction;
2. scoped brand usage through with_brand;
3. proof and capability progression;
4. recipe resolution through execution readiness;
5. trust-boundary bridging and readmission;
6. the primary scoped transition workflow.

One downstream executable contract exercises all six in
tests/supported_public_workflows.rs. A scenario is expected to exercise many
intermediate types. That is useful evidence: it proves the workflow at the
caller's altitude instead of maintaining a constructor witness for every
internal stage type.

## Checked Values And Scoped Values

Use the checked public door for structural facts:

~~~rust
let pair = DisjointPair::try_from_disjoint("left", "right")?;
assert_eq!(pair.left(), &"left");
~~~

Use a lexical scope when a value must not escape its brand:

~~~rust
let value = with_brand(|brand| brand.bind("scoped").into_value());
~~~

These workflows are supported because callers can complete a meaningful task,
not because every intermediate type has an individually registered producer.

## Progression And Boundaries

The common progression remains:

~~~rust
let executed = recipe("payload")
    .resolve_with(resolution_authority, 7_u8)
    .lower_with(lowering_capability)
    .ready_with(readiness_authority, "runtime admission")
    .execute();
~~~

When trust changes, bridge and readmit explicitly:

~~~rust
let executed = recipe("payload")
    .resolve_with(resolution_authority, 7_u8)
    .lower_with(lowering_capability)
    .bridge_trust_boundary()
    .readmit_with(readmission_authority, 11_u16)
    .ready_with(readiness_authority, "runtime admission")
    .execute();
~~~

The scoped default lane remains the primary workflow when one caller owns the
whole progression:

~~~rust
let executed = proof_flow()
    .resolution_authority(resolution_authority)
    .lowering_capability(lowering_capability)
    .readiness_authority(readiness_authority)
    .recipe("payload")
    .resolve(7_u8)
    .lower()
    .ready("runtime admission")
    .execute();
~~~

## Design Guidance

For sensitive operations:

- accept concrete owner types;
- issue them only from the owning workflow;
- define the domain marker and minting path in that owner, not in Proof;
- use named production imports and reexports; authority-governed globs are denied;
- test the real protected call with both owner-issued and counterfeit values;
- add direct-construction compile-fail evidence when privacy is load-bearing.

Avoid relying only on "private fields cannot be named." That proves one
constructor is closed; it does not prove the protected operation rejects a
structurally similar counterfeit value.

Also avoid generic `A: AuthorityMarker` or `C: CapabilityMarker` parameters on
governed facades. They describe an open substrate extension point, not the one
owner whose decision the operation requires.

## DX Posture

Start with use worth_proof::prelude::* and the supported workflows above. Use
worth_proof::raw::* when authoring a new progression surface or when the
pleasant lane would hide an important phase transition. Both lanes use the same
authority and proof values.
