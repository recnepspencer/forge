# Configured Domain Handles

## What This Feature Is

Configured domain handles are the next step after platform entry. They bind a
downstream domain marker to a typed operating context and turn that pair into a
Query-owned configured handle.

The important boundary is:

- your downstream crate owns domain identity and operating-context values
- Query owns the configured-handle lifecycle around them

That lifecycle gives you draft, validated, admitted, and checked forms without
falling back to raw IDs, ambient builder state, or host-local policy glue.

## Why You Use It

- freeze the stable operating regime your domain is working inside
- make capability and config-section requirements explicit before declaration
  work begins
- get one canonical handle identity digest for the configured domain context
- fail early when the operating regime is deferred, unsupported, or invalid for
  the current Query build

## Stable Entry Points

- `ForgeQueryDomainOperatingContext`
- `ForgeQueryDomainEntryRoot::with_operating_context(...)`
- `ForgeQueryDomainEntryProofRoot::with_operating_context(...)`
- `ForgeQueryDomainEntryChecked::with_operating_context(...)`
- `ForgeQueryConfiguredDomainHandleDraft`
- `ForgeQueryValidatedConfiguredDomainHandle`
- `ForgeQueryAdmittedConfiguredDomainHandle`
- `ForgeQueryConfiguredDomainHandleChecked`

## API Reference

Operating-context contract:

- `required_capability_families() -> &'static [ForgeQueryCapabilityFamily]`
- `required_config_sections() -> &'static [ForgeQueryConfigSectionFamily]`
- `context_identity_digest() -> String`

Configured-handle entry points:

- `with_operating_context(context) -> ForgeQueryConfiguredDomainHandleDraft<D, C>`
- `validate() -> Result<ForgeQueryValidatedConfiguredDomainHandle<D, C>, ForgeQueryConfiguredDomainHandleInvalidContext<D, C>>`
- `admit() -> Result<ForgeQueryAdmittedConfiguredDomainHandle<D, C>, ForgeQueryConfiguredDomainHandleAdmissionError<D, C>>`

Validated and admitted handle inspection:

- `domain_key() -> &'static str`
- `display_name() -> &'static str`
- `operating_context() -> &C`
- `support_snapshot() -> &ForgeQueryDomainEntrySupportSnapshot`
- `required_capability_families() -> &[ForgeQueryCapabilityFamily]`
- `required_config_sections() -> &[ForgeQueryConfigSectionFamily]`
- `operating_context_identity_digest() -> &str`
- `handle_identity_digest() -> &str`

Admitted-handle declaration evidence entry points:

- `describe_foundational(subject) -> Result<ForgeQueryDeclarationFoundationalEvidence<D, I>, ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>>`
- `describe_foundational_checked(subject) -> ForgeQueryDeclarationFoundationalEvidenceChecked<D, I>`
- `describe_foundational_with_profile(subject, profile) -> Result<ForgeQueryDeclarationFoundationalEvidence<D, I>, ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>>`

Admitted-handle route-planning entry points:

- `plan_routes(subject) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_checked(subject) -> ForgeQueryDeclarationRoutePlanChecked<D, I>`
- `plan_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `plan_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationRoutePlanTerminalError<D, I>>`
- `declare_review_progress_describe_and_plan(input) -> Result<ForgeQueryDeclarationRoutePlan<D, I>, ForgeQueryDeclarationEntryRoutePlanError<D, I>>`

Admitted-handle receipt entry points:

- `receipt_routes(subject) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `receipt_routes_checked(subject) -> ForgeQueryDeclarationReceiptChecked<D, I>`
- `receipt_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `receipt_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationReceiptTerminalError<D, I>>`
- `declare_review_progress_describe_plan_and_receipt(input) -> Result<ForgeQueryDeclarationReceipt<D, I>, ForgeQueryDeclarationEntryReceiptError<D, I>>`

Admitted-handle envelope entry points:

- `envelope_routes(subject) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `envelope_routes_checked(subject) -> ForgeQueryDeclarationEnvelopeChecked<D, I>`
- `envelope_routes_from_progressed(progressed) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `envelope_routes_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEnvelopeTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_and_envelope(input) -> Result<ForgeQueryDeclarationEnvelope<D, I>, ForgeQueryDeclarationEntryEnvelopeError<D, I>>`

Checked admission outcomes:

- `ForgeQueryConfiguredDomainHandleChecked::Admitted(ForgeQueryAdmittedConfiguredDomainHandle<D, C>)`
- `ForgeQueryConfiguredDomainHandleChecked::Deferred(ForgeQueryConfiguredDomainHandleDeferred<D, C>)`
- `ForgeQueryConfiguredDomainHandleChecked::Unsupported(ForgeQueryConfiguredDomainHandleUnsupported<D, C>)`
- `ForgeQueryConfiguredDomainHandleChecked::InvalidContext(ForgeQueryConfiguredDomainHandleInvalidContext<D, C>)`

Checked denial inspection:

- `blocking_capability_families() -> &[ForgeQueryCapabilityFamily]`
- `blocking_config_sections() -> &[ForgeQueryConfigSectionFamily]`
- `reason() -> &str`

## Core Mental Model

A configured domain handle is not a declaration and not a runtime binding.
It is the stable admitted world that later declaration work is allowed to
depend on.

That means it should carry stable regime facts such as:

- policy or access class
- invariant regime
- assumption or tolerance regime
- collaborator or tenant-like operating class when it changes the admitted
  operating world

It should not carry:

- declaration-specific meaning
- per-operation trigger dependencies
- exact preview, historical, or runtime basis binding
- callback-shaped permission or invariant logic

Query validates and admits the handle structurally. It does not pretend to own
your downstream domain semantics.

## How It Works

1. define a downstream marker type with `ForgeQueryDomainEntryMarker`
2. define a downstream operating-context type with
   `ForgeQueryDomainOperatingContext`
3. enter Query through `domain(...)`, `domain_checked(...)`, or
   `domain_proof_root(...)`
4. bind the operating context with `with_operating_context(...)`
5. validate the configured handle
6. admit it against the current support matrix and validated Query config

Validation checks structural honesty:

- capability-family canonicalization
- config-section canonicalization
- capability-to-section coverage
- stable configured-handle identity

Admission checks current support posture:

- deferred capability families
- unsupported capability families
- disabled required config sections

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

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
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::PreviewSession]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "access:collaborative|invariant:conservative|assumption:tight".to_string()
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();
let handle = query
    .domain(GeometryDomainEntry)
    .with_operating_context(GeometryOperatingContext)
    .validate()?
    .admit()?;
```

## Real Example

```rust
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryConfiguredDomainHandleChecked, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AccessClass {
    CollaborativeEditor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InvariantRegime {
    Conservative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AssumptionRegime {
    TightTolerance,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomainEntry;

impl ForgeQueryDomainEntryMarker for GeometryDomainEntry {
    fn domain_key(&self) -> &'static str {
        "worth.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
            ForgeQueryCapabilityFamily::IdentityEvolution,
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext {
    access_class: AccessClass,
    invariant_regime: InvariantRegime,
    assumption_regime: AssumptionRegime,
}

impl GeometryOperatingContext {
    fn collaborative() -> Self {
        Self {
            access_class: AccessClass::CollaborativeEditor,
            invariant_regime: InvariantRegime::Conservative,
            assumption_regime: AssumptionRegime::TightTolerance,
        }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomainEntry> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!(
            "access:{:?}|invariant:{:?}|assumption:{:?}",
            self.access_class, self.invariant_regime, self.assumption_regime
        )
    }
}

let query = ForgeQueryApplicationFacade::runtime_backed_default();

match query
    .domain_checked(GeometryDomainEntry)
    .with_operating_context(GeometryOperatingContext::collaborative())
{
    ForgeQueryConfiguredDomainHandleChecked::Admitted(handle) => {
        let _ = handle.operating_context_identity_digest();
        let _ = handle.handle_identity_digest();
        let _ = handle.required_capability_families();
    }
    ForgeQueryConfiguredDomainHandleChecked::Deferred(denial) => {
        let _ = denial.blocking_capability_families();
    }
    ForgeQueryConfiguredDomainHandleChecked::Unsupported(denial) => {
        let _ = denial.blocking_capability_families();
    }
    ForgeQueryConfiguredDomainHandleChecked::InvalidContext(denial) => {
        let _ = denial.blocking_config_sections();
    }
}
```

## Stable Operating Context Vs Dynamic Eligibility

This is the most important boundary to keep straight.

Stable operating context belongs in the configured handle:

- the general policy or access regime
- the general invariant regime
- the general assumption or tolerance regime
- other stable admitted-world posture

Dynamic eligibility belongs later:

- whether a specific operation may trigger now
- whether current truth satisfies a specific precondition
- whether a preview or historical basis makes one declaration legal
- whether a runtime dependency is available at this exact moment

If a value changes the stable admitted world, it belongs here.
If it changes the legality of one specific operation later, it does not.

That retained admitted-world identity is also what later legality,
progression, and foundational evidence surfaces consume. Those later features
should not call back into the operating-context object and rediscover world
identity on their own.

## Inspection And Debugging

The most useful inspection points are:

- `operating_context_identity_digest()`
- `handle_identity_digest()`
- `required_capability_families()`
- `required_config_sections()`
- `support_snapshot()`
- checked-lane denial posture

When a configured handle is denied:

- `Deferred` means the current build exposes the family but keeps it as debt
- `Unsupported` means the family is not available here
- `InvalidContext` means the operating context was structurally or
  configuration-wise incompatible with the current build

## Anti-Patterns

- passing raw collaborator IDs or tenant IDs as Query authority
- using bool shortcuts like `can_edit` or `preview`
- hiding access or invariant logic behind callbacks
- smuggling declaration-specific operation details into operating-context
  identity
- treating the configured handle as if it already proved dynamic eligibility

## Current Limits

Configured domain handles stop at stable admitted context.
They do not yet provide:

- declaration canonicalization
- declaration legality proof by themselves
- declaration progression proof by themselves
- foundational declaration evidence by themselves
- declaration route planning by themselves
- declaration boundary receipts by themselves
- declaration boundary envelopes by themselves
- dynamic operation eligibility
- preview, historical, or runtime basis binding
- lower-authority routing

## Related Docs

- [Canonical Domain Declarations](./canonical-domain-declarations.md)
- [Declaration Legality](./declaration-legality.md)
- [Declaration Progression](./declaration-progression.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Declaration Route Plans](./declaration-route-plan.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Platform Entry](./platform-entry.md)
- [Domain Capabilities Index](./README.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
