# Portable Query Packages

## What This Feature Is

Portable package export turns one validated Query package into a bounded,
versioned set of typed logical records. Use it when a host needs to retain or
hand off workflow meaning without carrying live installation or runtime
authority with it.

## Why You Use It

- Inspect the complete meaning that a host is about to retain or release.
- Pass store-neutral logical records to a later archive or reconstruction step.
- Keep PostgreSQL, Worth Store, and other physical adapters outside Query's
  schema and operation vocabulary.

## Stable Entry Points

Import the public host audience at `worth_query_host::facade::domain`:

- `WorthQueryValidatedPortableDomainPackage::export_typed_records()`
- `WorthQueryValidatedPortableDomainPackage::export_typed_records_with_limits(...)`
- `WorthQueryPortablePackageRecordSet`
- `WorthQueryPortablePackageManifest`
- `WorthQueryPortablePackageRecord`
- `WorthQueryPortablePackageRecordFamily`

Archive encoding, signatures, reconstruction, and fresh readmission are not
part of this surface yet. The exported package identity is descriptive data;
it is not proof that decoded records are valid.

## Core Mental Model

The validated package remains the source of truth. Export takes a snapshot of
its descriptive meaning and returns records in one canonical family order.
The record set carries no installed package handle, runtime generation,
provider binding, callback, or adapter row.

There are twelve record families:

1. domain identity;
2. capability requirements;
3. configuration requirements;
4. operating requirements;
5. definitions;
6. domain operations;
7. artifact contracts;
8. application schemas;
9. conditional application-operation bindings;
10. contribution policy;
11. retained native aspect contracts; and
12. retained application-operation contracts.

The last two families come from the contract spine retained during validation.
They contain the exact native contracts, typed reads and touches, external
correlation contract, and reconciliation procedure. Export does not rebuild
them from declaration names.

## How It Executes

1. Build and validate a `WorthQueryPortableDomainPackage`.
2. Call `export_typed_records()` on the validated package.
3. Query computes the expected count for every family.
4. Query checks the fixed record-count and logical-export-byte budgets.
5. Query assembles records in canonical family order and verifies them against
   the validated source inventory.
6. The host receives an immutable manifest and record slice for inspection or
   later carriage.

The manifest reports canonical source material separately from logical export
bytes. Logical export bytes cover that source material, stable record framing,
and the owner-defined canonical payload traversals that otherwise enter package
identity only by digest. A large domain operation or artifact contract therefore
cannot hide behind a fixed-width identity. Neither figure claims to be an
archive or wire-byte size.

## Small Example

```rust
use worth_query_host::facade::domain::{
    WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage,
    WorthQueryPortablePackageRecordFamily,
};

let validated = WorthQueryPortableDomainPackage::new(
    WorthQueryPortableDomainIdentity::new("acme.billing", 1, 0),
)
.requires_capability("billing-read")
.validate()
.expect("the package is valid");

let export = validated
    .export_typed_records()
    .expect("the package fits the fixed export budgets");

assert_eq!(export.manifest().package_identity(), validated.identity());
assert_eq!(
    export
        .manifest()
        .family_count(WorthQueryPortablePackageRecordFamily::DomainIdentity),
    1,
);
```

This is the smallest honest example because export is available only after
package validation.

## Real Example

```rust
use worth_query_host::facade::domain::{
    ApplicationSchema,
    WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage,
    WorthQueryPortablePackageRecord,
};

let validated = WorthQueryPortableDomainPackage::new(
    WorthQueryPortableDomainIdentity::new("acme.orders", 3, 1),
)
.application_schema(OrderWorkflow::declaration().expect("schema declaration"))
.validate()
.expect("package validation");

let export = validated.export_typed_records().expect("bounded export");

for view in export.views() {
    match view.record() {
        WorthQueryPortablePackageRecord::NativeAspectContract(contract) => {
            println!("native aspect: {}", contract.aspect().as_str());
        }
        WorthQueryPortablePackageRecord::ApplicationOperationContract(contract) => {
            println!("operation: {}", contract.operation());
        }
        _ => {}
    }
}
```

The application declaration owns workflow meaning. Validation retains exact
typed contracts. Export copies that descriptive meaning. A later physical
store may retain it, but neither the record variant nor the manifest identity
authorizes installation or execution.

## How It Relates To Other Features

- Application-schema declaration defines the typed source vocabulary.
- Package validation proves the in-process candidate before export.
- Installation consumes a validated package and adds runtime bindings and
  authority; export does not.
- Future reconstruction will consume untrusted records, rebuild a candidate,
  and ask Query to validate it again.
- Future archive support will define deterministic bytes and signatures around
  these logical records.

## Inspection And Debugging

Use `manifest()` to inspect:

- manifest version;
- expected package semantic identity;
- total logical record count;
- canonical source-byte count;
- logical export-byte count; and
- the count for each `WorthQueryPortablePackageRecordFamily`.

Use `records()` for direct matching or `views()` when canonical position is
useful. An export denial distinguishes record-count overflow, logical
export-byte overflow, and an internal incomplete-closure failure.

## Anti-Patterns

- Do not treat `manifest().package_identity()` as validation or install
  authority.
- Do not serialize installed handles, runtime generations, provider bindings,
  or adapter rows beside these records and call the result a portable package.
- Domain-operation records project canonical query and result meaning while
  stripping canonicalization proof. Legacy operations carrying installed aftermath fail validation;
  portable aftermath and reconciliation meaning comes from the retained
  application-operation contract record.
- Do not reconstruct native contracts, read scopes, touches, correlation, or
  reconciliation from schema member strings. Use the retained record variants.
- Do not select a package by a friendly name or by "latest"; compare the exact
  semantic identity after future fresh reconstruction.

## Current Limits

- The default ceiling is 65,536 logical records and 64 MiB of logical export
  material. Custom limits can narrow but cannot widen those ceilings.
- This surface does not yet decode records, reconstruct a package, or perform
  fresh validation.
- It does not define archive bytes, signing, release provenance, database
  layout, activation, or recovery.
- Export is a cold package-management operation, not part of warm query or
  mutation execution.

## Related Docs

- [Query orientation for AI agents](./AI_README.md)
- [Query documentation index](./README.md)
