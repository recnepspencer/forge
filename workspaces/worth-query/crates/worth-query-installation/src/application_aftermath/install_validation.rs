//! Axis-pair, external-effect, and pre-image coverage checks for aftermath install.

use worth_query_declaration::facade::application_aftermath::{
    DeclaredApplicationAftermathContract, DeclaredCorrectionAuthority, DeclaredCorrectionMechanism,
};

use super::denial::{
    WorthQueryAftermathInstallationDenial, WorthQueryAftermathInstallationDenialKind,
};
use super::external_effect_contract::InstalledExternalEffectContract;

/// Field slots taken from an operation's own decision-read targets.
///
/// Built only at the operation-compile resolution site (and crate-internal
/// tests of that derivation). There is no public constructor: a caller cannot
/// author a coverage list independent of the operation being installed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OperationDeclaredReadFields {
    field_slots: Vec<String>,
}

impl OperationDeclaredReadFields {
    pub(crate) fn from_field_slots(
        field_slots: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            field_slots: field_slots.into_iter().map(Into::into).collect(),
        }
    }

    pub(crate) fn field_slots(&self) -> &[String] {
        &self.field_slots
    }
}

pub(super) fn validate_axis_pair(
    declared: &DeclaredApplicationAftermathContract,
) -> Result<(), WorthQueryAftermathInstallationDenial> {
    match declared.authority() {
        DeclaredCorrectionAuthority::NotCorrectable => {
            if declared.mechanism().is_some() {
                return Err(WorthQueryAftermathInstallationDenial::new(
                    WorthQueryAftermathInstallationDenialKind::MechanismPresentForNotCorrectable,
                    "not-correctable-rejects-mechanism",
                ));
            }
            if declared.reconciliation().is_some() {
                return Err(WorthQueryAftermathInstallationDenial::new(
                    WorthQueryAftermathInstallationDenialKind::ReconciliationForbidden,
                    "not-correctable-rejects-reconciliation",
                ));
            }
            Ok(())
        }
        DeclaredCorrectionAuthority::RuntimeAlone => {
            if declared.mechanism().is_none() {
                return Err(WorthQueryAftermathInstallationDenial::new(
                    WorthQueryAftermathInstallationDenialKind::MechanismRequired,
                    "runtime-alone-requires-mechanism",
                ));
            }
            if declared.reconciliation().is_some() {
                return Err(WorthQueryAftermathInstallationDenial::new(
                    WorthQueryAftermathInstallationDenialKind::ReconciliationForbidden,
                    "runtime-alone-rejects-reconciliation",
                ));
            }
            Ok(())
        }
        DeclaredCorrectionAuthority::RuntimeWithExternalOwner => {
            if declared.mechanism().is_none() {
                return Err(WorthQueryAftermathInstallationDenial::new(
                    WorthQueryAftermathInstallationDenialKind::MechanismRequired,
                    "external-owner-requires-mechanism",
                ));
            }
            if declared.reconciliation().is_none() {
                return Err(WorthQueryAftermathInstallationDenial::new(
                    WorthQueryAftermathInstallationDenialKind::ReconciliationRequired,
                    "external-owner-requires-reconciliation",
                ));
            }
            Ok(())
        }
    }
}

pub(super) fn validate_preimage_coverage(
    declared: &DeclaredApplicationAftermathContract,
    declared_reads: &OperationDeclaredReadFields,
) -> Result<(), WorthQueryAftermathInstallationDenial> {
    let Some(DeclaredCorrectionMechanism::RecordedInverse(inverse)) = declared.mechanism() else {
        return Ok(());
    };
    let demand = inverse.preimage_demand();
    if demand.maximum_encoded_bytes() == 0 {
        return Err(WorthQueryAftermathInstallationDenial::new(
            WorthQueryAftermathInstallationDenialKind::PreImageDemandExceedsBound,
            "preimage-zero-bound",
        ));
    }
    // Ordered so each denial can actually fire. The whole-coverage case is
    // tested first: an operation that declares no reads at all is a different
    // installation mistake from one that declares reads and misses a slot, and
    // the installer needs to be told which. Behind the per-slot loop this arm
    // was unreachable, because the loop returns on the first uncovered slot.
    if declared_reads.field_slots().is_empty() && !demand.field_slots().is_empty() {
        return Err(WorthQueryAftermathInstallationDenial::new(
            WorthQueryAftermathInstallationDenialKind::MissingDeclaredReadsCoverage,
            "no-declared-reads",
        ));
    }
    for slot in demand.field_slots() {
        if !declared_reads.field_slots().iter().any(|read| read == slot) {
            return Err(WorthQueryAftermathInstallationDenial::new(
                WorthQueryAftermathInstallationDenialKind::PreImageDemandNotCoveredByDeclaredReads,
                slot.clone(),
            ));
        }
    }
    Ok(())
}

pub(super) fn escaping_effect_subject(external_effect: &InstalledExternalEffectContract) -> String {
    external_effect
        .correlation_family()
        .map_or_else(|| "unspecified-escaping-effect".into(), ToOwned::to_owned)
}
