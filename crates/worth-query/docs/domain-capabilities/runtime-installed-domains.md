# Runtime-Installed Domains

## What This Feature Is

Runtime-installed domains let a domain crate declare its Query capabilities
once, install that declaration into a concrete runtime, and use a handle that
can only operate with that runtime. Use this when your domain owns typed
operations, invariants, declaration families, or contribution vocabulary that
must participate in Query execution and diagnostics.

The public surface is `worth_query::facade::domain`. Domain code supplies typed
meaning. Query validates it, assigns canonical identity, compiles it into
runtime indexes, and retains the authority evidence needed by later work.

## Why You Use It

- Install domain operations and invariants before runtime work begins.
- Prevent a handle, declaration, or contribution from crossing runtime or
  installation generations accidentally.
- Give a domain crate ergonomic extension methods without moving identity,
  admission, execution, or receipt authority out of Query.
- Inspect one linked chain from package installation through execution and
  projection or diagnostic output.

## Stable Entry Points

- `domain::WorthQueryDomainPackage::declare(...)`
- `runtime::WorthQueryRuntimeBuilder::domain_package(...)`
- `runtime::WorthQueryWorkspace::domain(...)`
- `domain::WorthQueryInstalledDomainHandle`
- `domain::WorthQueryDomainOperatingContextIdentityDeclaration`
- `WorthQueryInstalledDomainHandle::{read, mutation, contributions_in, declarations_in}`
- `WorthQueryWorkspace::domain_installation_receipt(...)`

The handle also exposes installed live, workflow, inspection, rebind, and
operation-resolution capabilities. Reach those through the handle rather than
assembling their internal transition artifacts.

## Core Mental Model

A domain package is setup data, not executable authority. It contains typed
domain identity plus declarations such as required capabilities, configuration,
operating posture, invariants, graph obligations, graph-read operations,
declaration families, and permitted contribution categories.

Installing the package into a runtime creates the authority that later work
needs. The returned handle proves three things together:

1. which domain package authorized the work;
2. which runtime installed it;
3. which installation generation is current.

Runtime indexes are derived from the installed package. They make operation
lookup and invariant dispatch efficient, but they are not a second source of
domain authority. Query can rebuild them from installed artifacts.

An operating context describes the stable semantic fields of one domain world,
such as a tenant and an assumption regime. Domain code supplies those fields
through `WorthQueryDomainOperatingContextIdentityDeclaration`. Query orders and
seals them. The domain does not create an identity digest.

## How It Executes

The lifecycle is:

```text
declare package
  -> validate structure
  -> admit against Query support
  -> install atomically into one runtime
  -> obtain a runtime-affine handle
  -> declare a read, workflow, contribution, or domain declaration
  -> execute or inspect through the owning workspace
```

Package installation compiles all declared semantic families before the
runtime is published. If any family conflicts or cannot lower, installation
fails without leaving partial operation, invariant, obligation, contribution,
or declaration-family state.

Execution still performs its own basis, policy, tenant, target, and
lower-runtime checks. Installation proves domain authority; it does not grant
blanket permission for every later operation.

## Small Example

```rust
use worth_query::facade::{domain, runtime};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CatalogDomain;

impl domain::WorthQueryDomainEntryMarker for CatalogDomain {
    fn domain_key(&self) -> &'static str {
        "WORTH.catalog.products"
    }

    fn display_name(&self) -> &'static str {
        "Product Catalog"
    }

    fn required_capability_families(
        &self,
    ) -> &'static [domain::WorthQueryCapabilityFamily] {
        &[domain::WorthQueryCapabilityFamily::QueryRead]
    }
}

fn catalog_package() -> domain::WorthQueryDomainPackage<CatalogDomain> {
    domain::WorthQueryDomainPackage::declare(
        CatalogDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.catalog").unwrap(),
            domain::WorthQueryDomainIdentityName::new("products").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .requires_capability(domain::WorthQueryCapabilityFamily::QueryRead)
    .requires_configuration(domain::WorthQueryConfigSectionFamily::Query)
}

let builder = runtime::WorthQueryRuntimeBuilder::new()
    .domain_package(catalog_package())?;
# Ok::<_, domain::WorthQueryDomainPackageInstallationError>(builder)
```

This is the smallest honest setup example. It declares semantic input and asks
the runtime builder to install it. Query returns the installed authority and
its receipt.

## Real Example

A domain crate normally adds native vocabulary with an extension trait over the
generic installed handle:

```rust
use worth_query::facade::{domain, runtime};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CatalogContext {
    tenant: &'static str,
    pricing_regime: &'static str,
}

impl domain::WorthQueryDomainOperatingContext<CatalogDomain> for CatalogContext {
    fn required_capability_families(
        &self,
    ) -> &'static [domain::WorthQueryCapabilityFamily] {
        &[domain::WorthQueryCapabilityFamily::QueryRead]
    }

    fn required_config_sections(
        &self,
    ) -> &'static [domain::WorthQueryConfigSectionFamily] {
        &[domain::WorthQueryConfigSectionFamily::Query]
    }

    fn context_identity(
        &self,
    ) -> domain::WorthQueryDomainOperatingContextIdentityDeclaration {
        domain::WorthQueryDomainOperatingContextIdentityDeclaration::from_fields([
            ("tenant", self.tenant),
            ("pricing_regime", self.pricing_regime),
        ])
        .expect("static context field names are valid")
    }
}

trait CatalogQueryExt {
    fn catalog_context(
        &self,
        workspace: &runtime::WorthQueryWorkspace,
        context: CatalogContext,
    ) -> Result<
        domain::WorthQueryInstalledDomainDeclarationContext<CatalogDomain, CatalogContext>,
        domain::WorthQueryInstalledDomainDeclarationContextDenial,
    >;
}

impl CatalogQueryExt for domain::WorthQueryInstalledDomainHandle<CatalogDomain> {
    fn catalog_context(
        &self,
        workspace: &runtime::WorthQueryWorkspace,
        context: CatalogContext,
    ) -> Result<
        domain::WorthQueryInstalledDomainDeclarationContext<CatalogDomain, CatalogContext>,
        domain::WorthQueryInstalledDomainDeclarationContextDenial,
    > {
        self.declarations_in(workspace, context)
    }
}

let handle = workspace.domain(CatalogDomain)?;
let catalog = handle.catalog_context(
    &workspace,
    CatalogContext {
        tenant: "north-america",
        pricing_regime: "retail-v2",
    },
)?;
```

`CatalogQueryExt` improves domain ergonomics without becoming a parallel
authority. The extension delegates to the installed handle, so the resulting
context retains the same package, runtime, generation, and world witnesses as
the generic call.

The Hadwiger reference consumer follows this pattern for registered reads,
workflow mutations, installed invariants, contributions, projection facts, and
inspection.

## How It Relates To Other Features

- Use ordinary `facade::read`, `facade::workflow`, and `facade::inspection`
  directly when no domain package contributes semantic vocabulary.
- Use installed contributions when domain meaning must attach to a declaration,
  admitted plan, or lower-runtime boundary. Obtain the contribution surface
  with `handle.contributions_in(&workspace)`.
- Use an installed declaration context when a domain declaration family needs
  a typed operating world. The context identity remains Query-sealed.
- Physical storage, source, signal, and transport adapters remain runtime
  boundaries. They do not belong in the domain package.

## Inspection And Debugging

`workspace.domain_installation_receipt(marker)` reports the installed package
identity, definition counts, derived-index counts, warnings, and construction
counters. Use it to confirm that package lowering happened once.

Operational receipts retain installed authority. A read, workflow,
contribution, projection, live continuation, or inspection result can therefore
be linked back to the package and runtime that authorized it.

Foreign-runtime and stale-generation failures return typed denials or rebind
actions before planning or execution. Rebind asks the target runtime to admit
the domain again; it does not copy a generation or digest into a new handle.

## Anti-Patterns

- Creating domain identity from a raw string at execution time.
- Asking domain code to author a digest for its operating context.
- Calling Query transition materializers from a consumer crate.
- Treating an installation receipt, package digest, or diagnostic artifact as
  executable authority.
- Reconstructing a handle after runtime replacement instead of using rebind.

## Current Limits

- Domain packages are installed before runtime publication. Dynamic
  installation requires a separate quiescence and invalidation contract.
- Installed-domain durability and cross-process reload require a support
  profile that admits durable runtime state.
- Semantic package equivalence does not make handles interchangeable across
  runtimes.
- Rich diagnostics are optional projections; they do not change operational
  results or authorize future work.

## Related Docs

- [Declarative Query Experience](../capabilities/declarative-query-experience.md)
- [Consumer Kit](../foundations/consumer-kit.md)
- [Inspection](../capabilities/inspection.md)
- [Basis Capability Lifecycle](../capabilities/basis-capability-lifecycle.md)
- [Domain Capability Index](./README.md)
