use worth_store_formal_models::{
    OperationalRecoveryModelFamily as Family, OperationalRecoveryMutationSensitivitySuite,
};

use super::{S10OperationalScenarioKind, S10ScenarioCertificationDenial};

pub(super) fn require_mutation_families(
    kind: S10OperationalScenarioKind,
    suite: &OperationalRecoveryMutationSensitivitySuite,
) -> Result<(), S10ScenarioCertificationDenial> {
    let mut required = vec![
        Family::BackupMaterialization,
        Family::BackupReachability,
        Family::Authorization,
        Family::RecoveryStaging,
        Family::Publication,
    ];
    match kind {
        S10OperationalScenarioKind::BurningPrimary => required.extend([
            Family::Promotion,
            Family::ReplicaBootstrap,
            Family::PromotionPublication,
        ]),
        S10OperationalScenarioKind::SplitBrainPromotion => {
            required.extend([
                Family::OwnerExecution,
                Family::Promotion,
                Family::ReplicaBootstrap,
                Family::PromotionPublication,
                Family::OldPrimaryRejoin,
            ]);
        }
        S10OperationalScenarioKind::AuthorityRepairRollback => {
            required.push(Family::OwnerExecution);
        }
    }
    for family in required {
        if !suite
            .receipts()
            .iter()
            .any(|receipt| receipt.family() == family)
        {
            return Err(S10ScenarioCertificationDenial::MissingMutationFamily(
                family,
            ));
        }
    }
    Ok(())
}
