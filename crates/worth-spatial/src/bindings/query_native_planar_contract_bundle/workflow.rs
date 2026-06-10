use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_contract_bundle::authoring::{
    planar_contract_bundle_validation_entry, PlanarContractBundleValidationCase,
    PlanarContractBundleValidationEntry,
};
use crate::bindings::query_native_planar_contract_bundle::domain::PlanarContractBundleValidationQueryDomain;
use crate::bindings::query_native_planar_contract_bundle::facts::{
    planar_contract_bundle_validation_facts, PlanarContractBundleValidationFactError,
};
use crate::planar_contracts::contract_bundle::{
    m7_readiness_family_rows, PlanarBooleanReadinessBundle, PlanarContractBundleDenial,
    PlanarContractBundleDenialKind, PlanarContractBundleFamily,
    PlanarContractBundleValidationReceipt, PlanarM7ReadinessBasis, PlanarM7ReadinessBundle,
    PlanarM7ReadinessDenial, PlanarM7ReadinessDenialKind, PlanarM7ReadinessReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarContractBundleValidator {
    bundle: PlanarBooleanReadinessBundle,
    planar_neighborhood_identity: String,
}

impl PlanarContractBundleValidator {
    pub fn for_boolean_readiness(bundle: PlanarBooleanReadinessBundle) -> Self {
        Self {
            bundle,
            planar_neighborhood_identity: String::new(),
        }
    }

    pub fn within_planar_neighborhood(mut self, identity: impl Into<String>) -> Self {
        self.planar_neighborhood_identity = identity.into();
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a PlanarContractBundleValidationContracts<WC>,
    ) -> Result<PlanarContractBundleValidationPlan<'a, WC>, PlanarContractBundleValidationFactError>
    where
        WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
    {
        validate_planar_neighborhood_context(&self.bundle, &self.planar_neighborhood_identity)?;
        let entry = planar_contract_bundle_validation_entry(
            PlanarContractBundleValidationCase::from_boolean_readiness_bundle(
                self.bundle,
                self.planar_neighborhood_identity,
            ),
        );
        Ok(PlanarContractBundleValidationPlan {
            query_validation_entry: entry,
            validation_contracts: contracts,
        })
    }
}

fn validate_planar_neighborhood_context(
    bundle: &PlanarBooleanReadinessBundle,
    planar_neighborhood_identity: &str,
) -> Result<(), PlanarContractBundleValidationFactError> {
    if planar_neighborhood_identity.is_empty() {
        return Err(PlanarContractBundleValidationFactError::BundleBasis {
            denial: PlanarContractBundleDenial::new(
                PlanarContractBundleDenialKind::MissingTopologyBasis,
                None,
                "planar contract bundle validation requires a planar neighborhood identity",
            ),
        });
    }
    if bundle
        .topology_contract_receipt()
        .basis()
        .planar_neighborhood_identity()
        != planar_neighborhood_identity
    {
        return Err(PlanarContractBundleValidationFactError::BundleBasis {
            denial: PlanarContractBundleDenial::new(
                PlanarContractBundleDenialKind::MismatchedCertificateFamily,
                Some(PlanarContractBundleFamily::TopologyContractCompleteness),
                "bundle validation neighborhood must match topology completeness receipt",
            ),
        });
    }
    if bundle
        .overlap_receipt()
        .basis()
        .planar_neighborhood_identity()
        != planar_neighborhood_identity
    {
        return Err(PlanarContractBundleValidationFactError::BundleBasis {
            denial: PlanarContractBundleDenial::new(
                PlanarContractBundleDenialKind::MismatchedCertificateFamily,
                Some(PlanarContractBundleFamily::CoplanarOverlap),
                "bundle validation neighborhood must match the consumed overlap contract",
            ),
        });
    }
    if bundle
        .signed_area_receipt()
        .basis()
        .planar_neighborhood_identity()
        != planar_neighborhood_identity
        || bundle
            .winding_receipt()
            .basis()
            .planar_neighborhood_identity()
            != planar_neighborhood_identity
    {
        return Err(PlanarContractBundleValidationFactError::BundleBasis {
            denial: PlanarContractBundleDenial::new(
                PlanarContractBundleDenialKind::MismatchedCertificateFamily,
                Some(PlanarContractBundleFamily::SignedArea),
                "bundle validation neighborhood must match retained winding and signed-area facts",
            ),
        });
    }
    Ok(())
}

pub struct PlanarContractBundleValidationContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
{
    bundle_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PlanarContractBundleValidationQueryDomain, WC>,
}

impl<WC> PlanarContractBundleValidationContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
{
    pub fn new(
        bundle_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarContractBundleValidationQueryDomain,
            WC,
        >,
    ) -> Self {
        Self { bundle_handle }
    }
}

pub struct PlanarContractBundleValidationPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
{
    query_validation_entry: PlanarContractBundleValidationEntry,
    validation_contracts: &'a PlanarContractBundleValidationContracts<WC>,
}

impl<WC> PlanarContractBundleValidationPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
{
    pub fn inspected_bundle_rows(&self) -> usize {
        self.query_validation_entry
            .case()
            .basis()
            .family_rows()
            .len()
    }

    pub fn certify(
        self,
    ) -> Result<PlanarContractBundleValidationReceipt, PlanarContractBundleValidationFactError>
    {
        planar_contract_bundle_validation_facts(
            &self.query_validation_entry,
            &self.validation_contracts.bundle_handle,
        )
    }
}

impl PlanarM7ReadinessBundle {
    pub fn compile<'a, WC>(
        self,
        contracts: &'a PlanarContractBundleValidationContracts<WC>,
    ) -> Result<PlanarM7ReadinessPlan<'a, WC>, PlanarM7ReadinessDenial>
    where
        WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
    {
        let basis = self.build()?;
        let planar_neighborhood_identity = basis
            .boolean_readiness()
            .basis()
            .topology_contract_receipt()
            .basis()
            .planar_neighborhood_identity()
            .to_string();
        let entry = planar_contract_bundle_validation_entry(
            PlanarContractBundleValidationCase::from_boolean_readiness_bundle(
                basis.boolean_readiness().basis().clone(),
                planar_neighborhood_identity,
            ),
        );
        Ok(PlanarM7ReadinessPlan {
            basis,
            query_validation_entry: entry,
            validation_contracts: contracts,
        })
    }
}

pub struct PlanarM7ReadinessPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
{
    basis: PlanarM7ReadinessBasis,
    query_validation_entry: PlanarContractBundleValidationEntry,
    validation_contracts: &'a PlanarContractBundleValidationContracts<WC>,
}

impl<WC> PlanarM7ReadinessPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarContractBundleValidationQueryDomain>,
{
    pub fn inspected_closeout_rows(&self) -> usize {
        m7_readiness_family_rows(&self.basis).len()
    }

    pub fn certify(self) -> Result<PlanarM7ReadinessReceipt, PlanarM7ReadinessDenial> {
        let checked_root = planar_contract_bundle_validation_facts(
            &self.query_validation_entry,
            &self.validation_contracts.bundle_handle,
        )
        .map_err(|_| {
            PlanarM7ReadinessDenial::new(
                PlanarM7ReadinessDenialKind::QueryBoundaryMismatch,
                "M7 readiness must certify through the Query-native bundle boundary",
            )
        })?;
        if checked_root.fact_digest() != self.basis.boolean_readiness().fact_digest() {
            return Err(PlanarM7ReadinessDenial::new(
                PlanarM7ReadinessDenialKind::QueryBoundaryMismatch,
                "Query-checked bundle root must match the frozen boolean-readiness receipt",
            ));
        }
        Ok(PlanarM7ReadinessReceipt::new(
            self.basis,
            checked_root.declaration_digest(),
            checked_root.envelope_digest(),
        ))
    }
}
