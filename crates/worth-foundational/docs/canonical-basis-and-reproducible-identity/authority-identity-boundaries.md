# Authority Identity Boundaries

## What This Feature Is

Authority identity boundaries let a crate carry an identity value only after an
owner has admitted it with a proof witness. Use this when a value such as an ID,
handle, token, label, or digest must not be treated as the same thing just
because it has the same bytes or string.

## Why You Use It

- You need an API to accept only current authoritative identity.
- You need to send an identity across a trust boundary and force readmission.
- You need labels for logs or UI without letting those labels become identity.
- You need digest evidence without letting the digest replace the thing it
  summarizes.
- You need host-provided tokens to stay external until an owner admits them.

## Stable Entry Points

- `FoundationalAuthorityIdentity<Value, Authority, Kind>`
- `FoundationalAdmittedIdentityValue<Value, Authority, Kind>`
- `FoundationalBoundaryBridgedIdentity<Value, Authority, Kind>`
- `FoundationalRevalidatedIdentityValue<Value, Authority, Kind>`
- `FoundationalIdentityProjectionEvidence<Label, Authority, Kind>`
- `FoundationalProjectionIdentity<Label, Kind>`
- `FoundationalIdentityDigestDerivationEvidence<Basis, Authority, Kind>`
- `FoundationalDigestIdentityEvidence<Basis, Authority, Kind>`
- `FoundationalExternalIdentityToken<Value, Kind>`
- `FoundationalIdentityKind`
- `FoundationalIdentityBasis`
- `admit_foundational_authority_identity(...)`
- `admit_foundational_external_identity_token(...)`
- `readmit_foundational_authority_identity(...)`
- `readmit_revalidated_foundational_authority_identity(...)`
- `project_foundational_identity(...)`
- `derive_foundational_digest_identity_evidence(...)`
- `admitted_foundational_identity_value(...)`
- `revalidated_foundational_identity_value(...)`

These types are available through the `worth_foundational` facade. They reuse
`worth_proof::AuthorityWitness` for admission instead of adding a second proof
system.

The helpers are the preferred ergonomic front doors. They remove repeated
generic ceremony from standard lifecycle steps, but they do not hide authority:
each helper still requires the relevant `AuthorityWitness` at the call site.

## Core Mental Model

An authoritative identity is a value plus proof that the owning crate admitted
it. `Authority` answers who can admit the value. `Kind` answers what category of
identity this is. `Value` is only the representation.

`worth-foundational` does not know runtime semantics. It does not know what a
query commit, bridge lowering, signal output, or kernel handle means. Those
crates define marker types and plug them into the shared boundary shape.

Projection, digest evidence, external tokens, and bridged identities are useful
objects, but they are not authority:

- A projection identity is for display, logs, diagnostics, and wire-friendly
  labels.
- Digest identity evidence proves what was summarized by canonicalization.
- An external identity token is a host or runtime value before WORTH admission.
- A bridged identity crossed a trust boundary and must be readmitted.

## How It Executes

1. A downstream crate defines a `Kind` marker for one identity category.
2. The owner defines an `Authority` marker and controls how witnesses are made.
3. The owner admits a value with `admit_foundational_authority_identity`, or
   uses the lower-level `FoundationalAdmittedIdentityValue` when it needs to
   expose the intermediate proof step.
4. The lower-level path promotes admitted value with
   `FoundationalAuthorityIdentity::from_admitted`.
5. Consumers carry the authority identity, not a `String` or digest.
6. If the identity crosses a boundary, call `bridge_trust_boundary`.
7. After revalidation, use `readmit_foundational_authority_identity` for the
   common path, or explicitly produce `FoundationalRevalidatedIdentityValue`
   before calling `FoundationalAuthorityIdentity::readmit`.
8. If you need display or digest output, use the projection or digest helpers,
   or explicitly carry the evidence objects when the intermediate proof matters.

## DX Contract

Use the helper layer when an API wants the normal authority identity lifecycle:
admit, readmit, project, derive digest evidence, or admit an external token.
This keeps downstream call sites small enough to read while preserving the
security shape in the signature.

Use the intermediate types directly when an API boundary needs to name the
partial state itself. For example, use `FoundationalAdmittedIdentityValue` when
one step admits a value and a later step promotes it, or
`FoundationalRevalidatedIdentityValue` when revalidation must be reviewed before
readmission.

Do not build crate-local "nicer" shortcuts that remove the witness argument,
infer authority from ambient state, or reclassify a projection as authority.
Downstream crates may wrap these helpers with domain-specific names, but the
wrapper must still take and spend the explicit authority witness.

## Small Example

```rust
use worth_foundational::{
    FoundationalAuthorityIdentity, FoundationalIdentityKind,
    admit_foundational_authority_identity,
};
use worth_proof::{AuthorityMarker, AuthorityWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueryIdentityAuthority(());
impl AuthorityMarker for QueryIdentityAuthority {}

struct QueryCommitIdentityKind;
impl FoundationalIdentityKind for QueryCommitIdentityKind {}

type QueryCommitIdentity =
    FoundationalAuthorityIdentity<u64, QueryIdentityAuthority, QueryCommitIdentityKind>;

fn authority() -> AuthorityWitness<QueryIdentityAuthority> {
    AuthorityWitness::from_authority_marker(QueryIdentityAuthority(()))
}

let identity: QueryCommitIdentity =
    admit_foundational_authority_identity(42, authority());
assert_eq!(identity.value(), &42);
```

This is the smallest honest example because the value `42` is not enough. The
type also records who admitted it and which identity kind it belongs to.
The helper removes generic boilerplate, but the authority witness remains
visible at the call site.
The raw value is borrowable for inspection, but the authoritative wrapper does
not expose public owned extraction. If a consumer needs a portable value, create
a projection label or digest evidence instead of stripping authority.

## Real Example

```rust
use worth_foundational::{
    FoundationalAuthorityIdentity, FoundationalExternalIdentityToken,
    FoundationalIdentityKind, FoundationalProjectionIdentity,
    admit_foundational_external_identity_token, project_foundational_identity,
    readmit_foundational_authority_identity,
};
use worth_proof::{AuthorityMarker, AuthorityWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RuntimeAdmissionAuthority(());
impl AuthorityMarker for RuntimeAdmissionAuthority {}

struct SignalOutputIdentityKind;
impl FoundationalIdentityKind for SignalOutputIdentityKind {}

type SignalOutputIdentity =
    FoundationalAuthorityIdentity<String, RuntimeAdmissionAuthority, SignalOutputIdentityKind>;

fn authority() -> AuthorityWitness<RuntimeAdmissionAuthority> {
    AuthorityWitness::from_authority_marker(RuntimeAdmissionAuthority(()))
}

let host_token =
    FoundationalExternalIdentityToken::<String, SignalOutputIdentityKind>::new(
        "host-output-17".to_string(),
    );

let identity: SignalOutputIdentity =
    admit_foundational_external_identity_token(host_token, authority());
let label: FoundationalProjectionIdentity<String, SignalOutputIdentityKind> =
    project_foundational_identity(&identity, "output #17".to_string(), authority());
let bridged = identity.bridge_trust_boundary();
let readmitted: SignalOutputIdentity =
    readmit_foundational_authority_identity(bridged, authority());

assert_eq!(label.label(), "output #17");
assert_eq!(readmitted.value(), "host-output-17");
```

The host token starts as external. The owner admits it before APIs can use it as
WORTH authority. The label can be logged or displayed, but it cannot be passed
to a function that requires `SignalOutputIdentity`.

Use the explicit intermediate types when you want to expose or test the proof
step itself:

```rust
use worth_foundational::{
    FoundationalAdmittedIdentityValue, FoundationalAuthorityIdentity,
};

let admitted = FoundationalAdmittedIdentityValue::admit(42, authority());
let identity = QueryCommitIdentity::from_admitted(admitted);
```

There are helper forms for this lower lane too:

```rust
use worth_foundational::{
    admitted_foundational_identity_value, revalidated_foundational_identity_value,
};

let admitted = admitted_foundational_identity_value(42, authority());
let identity = QueryCommitIdentity::from_admitted(admitted);
let bridged = identity.bridge_trust_boundary();
let revalidated = revalidated_foundational_identity_value(bridged, authority());
let readmitted = QueryCommitIdentity::readmit(revalidated);
```

## How It Relates To Other Features

- Pair this with canonical basis and digest derivation when you need stable
  evidence for what an identity-related surface contained.
- Use boundary artifacts when the whole artifact needs role and materialization
  posture. Use authority identity boundaries when the problem is specifically
  identity substitution.
- Use `worth-proof::AuthorityWitness` for admission. Do not create local proof
  tokens that bypass the shared witness lane.
- Use the helper functions for the common path. Drop to the intermediate
  proof-carrying types when an API boundary needs to expose exactly what has
  been proven so far.

## Inspection And Debugging

When an identity does not type-check, check which category you are holding:

- `FoundationalAuthorityIdentity` means current authority.
- `FoundationalAdmittedIdentityValue` means admitted but not yet promoted into
  the authority identity wrapper.
- `FoundationalBoundaryBridgedIdentity` means readmission is required.
- `FoundationalRevalidatedIdentityValue` means a bridged value was checked and
  can be readmitted.
- `FoundationalIdentityProjectionEvidence` means a projection was derived from
  authority.
- `FoundationalProjectionIdentity` means label only.
- `FoundationalIdentityDigestDerivationEvidence` means a digest was derived from
  authority.
- `FoundationalDigestIdentityEvidence` means digest evidence only.
- `FoundationalExternalIdentityToken` means not admitted yet.

The generic `Kind` is usually the fastest clue. If two identities have the same
payload type but different `Kind` markers, they are intentionally incompatible.

## Anti-Patterns

- Do not accept `String` where authority identity is required.
- Do not call authority identity construction directly from a raw value.
- Do not pass admitted or revalidated identity values where current authority is
  required.
- Do not parse a projection label back into authority.
- Do not use digest bytes as an ID.
- Do not construct projection or digest evidence from raw labels or digests
  without tying them to authority.
- Do not write local shortcuts that hide the authority witness; prefer the
  foundational helpers if the lifecycle step is standard.
- Do not add raw-value extraction to authority or bridged identity wrappers.
- Do not pass a bridged identity to current-authority APIs.
- Do not add runtime-specific marker names to `worth-foundational`.
- Do not use the same `Kind` marker for two identity meanings just because they
  share a representation.

## Current Limits

- This feature provides the reusable boundary shape; downstream crates still
  need to migrate their string-heavy APIs to require these types.
- The foundational crate does not decide which runtime values are valid. Owning
  crates must perform admission and revalidation.
- Digest evidence wraps existing canonical digest output. It is not a second
  digest algorithm or a new canonicalization lane.

## Related Docs

- [Canonical Basis Sequences And Entry Grammar](./canonical-basis-sequences-and-entry-grammar.md)
- [Digest Derivation And Slot Semantics](./digest-derivation-and-slot-semantics.md)
- [Grouped Public Lanes And Front-Door Usage](./grouped-public-lanes-and-front-door-usage.md)
