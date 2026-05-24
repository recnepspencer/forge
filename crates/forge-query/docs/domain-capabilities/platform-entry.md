# Platform Entry

## What This Feature Is

Platform entry is the public `forge-query` front door for downstream domains.
It lets your domain enter Query through a typed marker instead of starting from
raw strings, ad hoc setup glue, or a local pseudo-Query layer.

`forge-query` stays generic here. It does not ship concrete domain types. Your
crate defines the domain marker, and Query provides the entry capability,
support posture, and lane structure around it.

## Why You Use It

- start domain work through Query from the beginning instead of wrapping Query
  later
- make the required Query capability families explicit at the call site
- choose between an ordinary lane, a checked lane, and a proof-oriented lane
  without changing the domain meaning
- inspect support posture before you commit to deeper declaration or runtime
  work

## Stable Entry Points

- `ForgeQueryApplicationFacade::domain_entry_support_snapshot()`
- `ForgeQueryApplicationFacade::domain(marker)`
- `ForgeQueryApplicationFacade::domain_checked(marker)`
- `ForgeQueryApplicationFacade::domain_proof_root(marker)`
- `ForgeQueryDomainEntryMarker`

Good to know:
- this is the preferred front door when a downstream domain needs to enter
  Query as a typed capability
- the older `forge_query_domain("...")` path belongs to string-authored domain
  contribution surfaces, not this typed platform-entry path

## Core Mental Model

There are two different ownership boundaries here:

- your downstream crate owns domain identity
- Query owns the generic entry capability

The marker you hand Query is intentionally small. It tells Query:

- which domain is asking to enter
- how that domain should be named in public artifacts
- which Query capability families must be available for checked entry to admit

The returned value is an entry artifact, not a fully configured domain handle.
Its job is to preserve domain identity plus current support posture so later
authoring and routing stages can build on a Query-owned start.

## How It Works

1. define a downstream-owned marker type that implements
   `ForgeQueryDomainEntryMarker`
2. build a `ForgeQueryApplicationFacade`
3. optionally inspect `domain_entry_support_snapshot()` to see the current
   support posture
4. pass your marker to one of the public entry lanes
5. Query compares the marker's required capability families against the current
   support report
6. the checked lane classifies the result as `Admitted`, `Deferred`, or
   `Unsupported`

The ordinary lane still matters even when checked entry would defer or deny,
because it preserves the same domain identity and attaches the same support
snapshot for inspection.

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker,
};

const GEOMETRY_ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::QueryComposition,
    ForgeQueryCapabilityFamily::QueryContext,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        GEOMETRY_ENTRY_CAPABILITIES
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let geometry = query.domain(GeometryDomainEntry);
```

This is the smallest honest example because it shows the real contract:
downstream code owns the marker, and Query contributes the entry surface.

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryChecked,
    ForgeQueryDomainEntryMarker,
};

const GEOMETRY_ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::QueryComposition,
    ForgeQueryCapabilityFamily::QueryContext,
    ForgeQueryCapabilityFamily::IdentityEvolution,
    ForgeQueryCapabilityFamily::PreviewSession,
    ForgeQueryCapabilityFamily::WorkflowOrchestration,
    ForgeQueryCapabilityFamily::HistoricalEvaluation,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        GEOMETRY_ENTRY_CAPABILITIES
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let support = query.domain_entry_support_snapshot();

match query.domain_checked(GeometryDomainEntry) {
    ForgeQueryDomainEntryChecked::Admitted(root) => {
        let _ = root.domain_key();
        let _ = root.support_snapshot().snapshot_digest();
        let _ = support.section_postures();
    }
    ForgeQueryDomainEntryChecked::Deferred(deferred) => {
        let _ = deferred.blocking_capability_families();
    }
    ForgeQueryDomainEntryChecked::Unsupported(unsupported) => {
        let _ = unsupported.blocking_capability_families();
    }
}
```

What is authoritative here:

- the downstream marker is authoritative for domain identity and required
  capability families
- Query's support report is authoritative for admission posture

What gets retained:

- the ordinary, checked, and proof-oriented entry artifacts retain the support
  snapshot
- later inspection can explain the same entry posture without recomputing the
  meaning of the marker

## Choosing A Lane

Use `domain(...)` when you want the standard typed entry artifact and will
inspect support posture yourself.

Use `domain_checked(...)` when you want Query to classify the current posture
up front:

- `Admitted` means every required capability family is currently supported
- `Deferred` means the current build exposes the family but intentionally fails
  closed for now
- `Unsupported` means the family is not available for this build

Use `domain_proof_root(...)` when you need the proof-oriented sibling of the
same entry meaning and support posture.

## Inspection And Debugging

The main inspection surface is `domain_entry_support_snapshot()`, or the
support snapshot attached to a returned entry artifact.

Use it to inspect:

- admitted capability families
- deferred capability families
- unsupported capability families
- section posture for `Query`, `Relational`, `Signal`, `RuntimeBridge`, and
  `Store`
- the validated config digest and snapshot digest

When checked entry is not admitted, start with
`blocking_capability_families()`.

## Anti-Patterns

- expecting Query to export your domain type for you
- starting entry from raw strings like `"worth.spatial"`
- treating the marker as a full domain configuration object
- treating platform entry as if it already performs declaration authoring,
  runtime routing, or continuation

## Current Limits

Platform entry gives you a typed front door and support posture. It does not
yet give you:

- full domain configuration or validated domain handles
- declaration canonicalization or legality
- relational, bridge, or signal routing
- runtime continuation

## Related Docs

- [Configured Domain Handles](./configured-domain-handles.md)
- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Domain Capabilities Index](./README.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
- [Workflow Lanes: Common, Checked, Proof, And Raw](./workflow/workflow-lanes-common-checked-proof-raw.md)
