//! Exact root-family materialization from a structurally closed candidate.

use crate::package::portable_records::readmit_portable_domain_operation;
use crate::package::{
    WorthQueryPortableDomainPackage, WorthQueryPortablePackageRecord,
    WorthQueryPortablePackageRecordFamily,
};

use super::{
    WorthQueryPortablePackageReconstructionCandidate,
    WorthQueryPortablePackageReconstructionDenial as Denial,
    WorthQueryReconstructedPortablePackageCandidate,
};

impl WorthQueryPortablePackageReconstructionCandidate {
    pub fn materialize(self) -> Result<WorthQueryReconstructedPortablePackageCandidate, Denial> {
        let (manifest, records, limits, mut work) = self.into_parts();
        let domain_count =
            manifest.family_count(WorthQueryPortablePackageRecordFamily::DomainIdentity);
        if domain_count != 1 {
            return Err(Denial::DomainIdentityCardinality {
                observed: domain_count,
            });
        }
        let mut package = None;
        let mut expected_native_aspects = Vec::new();
        let mut expected_application_operations = Vec::new();
        for record in records {
            match record {
                WorthQueryPortablePackageRecord::DomainIdentity(identity) => {
                    package = Some(WorthQueryPortableDomainPackage::new(identity));
                }
                WorthQueryPortablePackageRecord::CapabilityRequirement(value) => {
                    package = Some(take_package(&mut package).requires_capability(value.as_str()));
                }
                WorthQueryPortablePackageRecord::ConfigurationRequirement(value) => {
                    package =
                        Some(take_package(&mut package).requires_configuration(value.as_str()));
                }
                WorthQueryPortablePackageRecord::OperatingRequirement(value) => {
                    package =
                        Some(take_package(&mut package).requires_operating_posture(value.as_str()));
                }
                WorthQueryPortablePackageRecord::Definition(value) => {
                    package = Some(take_package(&mut package).definition(value));
                }
                WorthQueryPortablePackageRecord::DomainOperation(value) => {
                    let remaining = work.remaining_canonical_work_bytes(limits);
                    let configured = limits.canonical_query_limits();
                    let query_limits = worth_query_declaration::facade::canonicalization::WorthQueryPortableCanonicalQueryReadmissionLimits::new(
                        configured.maximum_entries(),
                        configured.maximum_logical_bytes().min(remaining),
                    );
                    let (operation, canonical_work_bytes) =
                        readmit_portable_domain_operation(value, query_limits, remaining)?;
                    work = work.consume_canonical_work(canonical_work_bytes, limits)?;
                    package = Some(take_package(&mut package).domain_operation(operation));
                }
                WorthQueryPortablePackageRecord::ArtifactContract(value) => {
                    let remaining = work.remaining_canonical_work_bytes(limits);
                    let (contract, canonical_work_bytes) = crate::domain_computation::validate_portable_artifact_contract_freshly_with_work(
                            value, remaining,
                        )
                        .map_err(|denial| Denial::ArtifactContractReadmissionDenied { denial })?;
                    work = work.consume_canonical_work(canonical_work_bytes, limits)?;
                    package = Some(take_package(&mut package).artifact_contract(contract));
                }
                WorthQueryPortablePackageRecord::ApplicationSchema(value) => {
                    let maximum_source_bytes = work.remaining_canonical_work_bytes(limits);
                    let (schema, schema_work) = worth_query_declaration::facade::application_schema::validate_portable_application_schema_freshly_with_work(
                        value,
                        maximum_source_bytes,
                        u64::MAX,
                    )
                        .map_err(|denial| Denial::ApplicationSchemaReadmissionDenied { denial })?;
                    work = work.consume_canonical_work(schema_work.source_bytes(), limits)?;
                    package = Some(take_package(&mut package).application_schema_erased(schema));
                }
                WorthQueryPortablePackageRecord::ConditionalApplicationOperation(value) => {
                    package = Some(
                        take_package(&mut package).conditional_application_operation_erased(value),
                    );
                }
                WorthQueryPortablePackageRecord::ContributionPolicy(value) => {
                    package = Some(take_package(&mut package).permits_contribution(value.as_str()));
                }
                WorthQueryPortablePackageRecord::NativeAspectContract(value) => {
                    expected_native_aspects.push(value);
                }
                WorthQueryPortablePackageRecord::ApplicationOperationContract(value) => {
                    expected_application_operations.push(value);
                }
            }
        }
        Ok(WorthQueryReconstructedPortablePackageCandidate {
            manifest,
            package: package.expect("validated family order contains one domain identity"),
            expected_native_aspects,
            expected_application_operations,
            limits,
            work,
        })
    }
}

fn take_package(
    package: &mut Option<WorthQueryPortableDomainPackage>,
) -> WorthQueryPortableDomainPackage {
    package
        .take()
        .expect("validated family order places the domain identity first")
}
