use worth_foundational::facade::{
    CanonicalDigestDerivationDenial, CanonicalDigestId, CanonicalDigestWorkBudget,
};

use crate::canonical_digest_derivation::InstallationCanonicalIdentityBasis;
use crate::canonical_work::WorthQueryCanonicalWorkEvidence;
use crate::package::WorthQueryValidatedPortableDomainPackage;

use super::WorthQueryInstallationAdmissionProfile;

const ADMISSION_BUDGET: CanonicalDigestWorkBudget =
    match CanonicalDigestWorkBudget::new(32_768, 4 * 1_024 * 1_024) {
        Some(budget) => budget,
        None => panic!("fixed installation-admission canonical-work budget is valid"),
    };

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstallationAdmissionIdentity(CanonicalDigestId);

impl WorthQueryInstallationAdmissionIdentity {
    pub const fn digest(&self) -> &CanonicalDigestId {
        &self.0
    }

    pub const fn bytes(&self) -> &[u8; 32] {
        self.0.bytes()
    }

    pub fn render_support_hex(&self) -> String {
        self.0.render_hex()
    }
}

pub(super) fn admission_identity(
    package: &WorthQueryValidatedPortableDomainPackage,
    profile: &WorthQueryInstallationAdmissionProfile,
) -> Result<
    (
        WorthQueryInstallationAdmissionIdentity,
        WorthQueryCanonicalWorkEvidence,
    ),
    CanonicalDigestDerivationDenial,
> {
    let mut basis = InstallationCanonicalIdentityBasis::new(
        "worth-query.installation-admission",
        "worth-query-installation-admission-v2",
        ADMISSION_BUDGET,
    );
    basis.digest("package", *package.identity().digest())?;
    basis.text("support", &profile.support_identity)?;
    basis.text("configuration-profile", &profile.configuration_identity)?;
    append_required_support(&mut basis, package, profile)?;
    append_artifact_support(&mut basis, package, profile)?;
    let (digest, work) = basis.derive()?;
    Ok((WorthQueryInstallationAdmissionIdentity(digest), work))
}

fn append_required_support(
    basis: &mut InstallationCanonicalIdentityBasis,
    package: &WorthQueryValidatedPortableDomainPackage,
    profile: &WorthQueryInstallationAdmissionProfile,
) -> Result<(), CanonicalDigestDerivationDenial> {
    for (index, family) in package.capabilities().iter().enumerate() {
        let prefix = format!("capability[{index}]");
        basis.text(format!("{prefix}.subject"), family.as_str())?;
        basis.text(
            format!("{prefix}.status"),
            profile.capability_statuses[family.as_str()].canonical_part(),
        )?;
    }
    for (index, section) in package.configuration().iter().enumerate() {
        let prefix = format!("configuration[{index}]");
        basis.text(format!("{prefix}.subject"), section.as_str())?;
        basis.text(
            format!("{prefix}.status"),
            if profile.configuration_statuses[section.as_str()] {
                "enabled"
            } else {
                "disabled"
            },
        )?;
    }
    for (index, requirement) in package.operating_requirements().iter().enumerate() {
        let prefix = format!("operating[{index}]");
        basis.text(format!("{prefix}.subject"), requirement.as_str())?;
        basis.text(
            format!("{prefix}.status"),
            profile.operating_statuses[requirement.as_str()].canonical_part(),
        )?;
    }
    Ok(())
}

fn append_artifact_support(
    basis: &mut InstallationCanonicalIdentityBasis,
    package: &WorthQueryValidatedPortableDomainPackage,
    profile: &WorthQueryInstallationAdmissionProfile,
) -> Result<(), CanonicalDigestDerivationDenial> {
    for (index, contract) in package.artifact_contracts().iter().enumerate() {
        let prefix = format!("artifact[{index}]");
        let key = (
            contract.family().as_str().to_string(),
            contract.schema_version().get(),
            contract.protocol_version().get(),
        );
        basis.text(format!("{prefix}.family"), &key.0)?;
        basis.unsigned_u32(format!("{prefix}.schema-version"), key.1)?;
        basis.unsigned_u32(format!("{prefix}.protocol-version"), key.2)?;
        basis.text(
            format!("{prefix}.status"),
            profile.artifact_version_statuses[&key].canonical_part(),
        )?;
        if let Some(comparator) = contract.reproducibility().comparison().registered_family() {
            basis.text(format!("{prefix}.comparator"), comparator)?;
            basis.text(
                format!("{prefix}.comparator-status"),
                profile.artifact_comparator_statuses[comparator].canonical_part(),
            )?;
        }
    }
    Ok(())
}
