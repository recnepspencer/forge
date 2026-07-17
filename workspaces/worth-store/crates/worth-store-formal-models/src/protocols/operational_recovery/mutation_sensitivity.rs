use sha2::{Digest, Sha256};

use super::{
    OperationalRecoveryAction, OperationalRecoveryActionKind as Action,
    OperationalRecoveryControlledDefect, OperationalRecoveryInvariant, OperationalRecoveryModel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationalRecoveryModelFamily {
    Authorization,
    OwnerExecution,
    Publication,
    Promotion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationalRecoveryMutationSensitivityDenial {
    BaselineAcceptedDefect(OperationalRecoveryModelFamily),
    WrongInvariant {
        family: OperationalRecoveryModelFamily,
        observed: OperationalRecoveryInvariant,
    },
    MutatedModelStillRejected(OperationalRecoveryModelFamily),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryMutationSensitivityReceipt {
    family: OperationalRecoveryModelFamily,
    defect: OperationalRecoveryControlledDefect,
    localized_invariant: OperationalRecoveryInvariant,
    localized_transition: String,
    receipt_identity: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryMutationSensitivitySuite {
    receipts: [OperationalRecoveryMutationSensitivityReceipt; 4],
    suite_identity: [u8; 32],
}

pub fn check_operational_recovery_mutation_sensitivity(
) -> Result<OperationalRecoveryMutationSensitivitySuite, OperationalRecoveryMutationSensitivityDenial>
{
    let receipts = [
        check_case(MutationCase::authorization())?,
        check_case(MutationCase::owner_execution())?,
        check_case(MutationCase::publication())?,
        check_case(MutationCase::promotion())?,
    ];
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-recovery-mutation-suite-v1");
    for receipt in &receipts {
        digest.update(receipt.receipt_identity);
    }
    Ok(OperationalRecoveryMutationSensitivitySuite {
        receipts,
        suite_identity: digest.finalize().into(),
    })
}

impl OperationalRecoveryMutationSensitivityReceipt {
    pub const fn family(&self) -> OperationalRecoveryModelFamily {
        self.family
    }
    pub const fn defect(&self) -> OperationalRecoveryControlledDefect {
        self.defect
    }
    pub const fn localized_invariant(&self) -> OperationalRecoveryInvariant {
        self.localized_invariant
    }
    pub fn localized_transition(&self) -> &str {
        &self.localized_transition
    }
    pub const fn receipt_identity(&self) -> [u8; 32] {
        self.receipt_identity
    }
}

impl OperationalRecoveryMutationSensitivitySuite {
    pub fn receipts(&self) -> &[OperationalRecoveryMutationSensitivityReceipt; 4] {
        &self.receipts
    }
    pub const fn suite_identity(&self) -> [u8; 32] {
        self.suite_identity
    }
}

struct MutationCase {
    family: OperationalRecoveryModelFamily,
    defect: OperationalRecoveryControlledDefect,
    expected: OperationalRecoveryInvariant,
    prefix: Vec<OperationalRecoveryAction>,
    defective: OperationalRecoveryAction,
}

impl MutationCase {
    fn authorization() -> Self {
        Self::new(
            OperationalRecoveryModelFamily::Authorization,
            OperationalRecoveryControlledDefect::ExecutionWithoutAuthorization,
            OperationalRecoveryInvariant::AuthorizationBeforeExecution,
            [],
            Action::OwnerExecutionOpened,
        )
    }

    fn owner_execution() -> Self {
        Self::new(
            OperationalRecoveryModelFamily::OwnerExecution,
            OperationalRecoveryControlledDefect::OwnerReceiptWithoutEffectStart,
            OperationalRecoveryInvariant::OwnerEffectBeforeReceipt,
            [Action::AuthorizationConsumed, Action::OwnerExecutionOpened],
            Action::OwnerReceiptPersisted,
        )
    }

    fn publication() -> Self {
        Self::new(
            OperationalRecoveryModelFamily::Publication,
            OperationalRecoveryControlledDefect::PublicationWithoutPreparation,
            OperationalRecoveryInvariant::PreparationBeforePublication,
            [],
            Action::PublicationPending,
        )
    }

    fn promotion() -> Self {
        Self::new(
            OperationalRecoveryModelFamily::Promotion,
            OperationalRecoveryControlledDefect::PromotionWithoutExternalFence,
            OperationalRecoveryInvariant::ExternalFenceBeforePromotion,
            [Action::AuthorizationConsumed],
            Action::ReplicaPromotionRecorded,
        )
    }

    fn new(
        family: OperationalRecoveryModelFamily,
        defect: OperationalRecoveryControlledDefect,
        expected: OperationalRecoveryInvariant,
        prefix: impl IntoIterator<Item = Action>,
        defective: Action,
    ) -> Self {
        let operation = format!("controlled-{family:?}");
        let prefix = prefix
            .into_iter()
            .enumerate()
            .map(|(index, kind)| {
                OperationalRecoveryAction::controlled_defect_probe(
                    &operation,
                    &format!("prefix-{index}"),
                    kind,
                )
            })
            .collect();
        Self {
            family,
            defect,
            expected,
            prefix,
            defective: OperationalRecoveryAction::controlled_defect_probe(
                &operation,
                "defective-edge",
                defective,
            ),
        }
    }
}

fn check_case(
    case: MutationCase,
) -> Result<
    OperationalRecoveryMutationSensitivityReceipt,
    OperationalRecoveryMutationSensitivityDenial,
> {
    let baseline = apply_case(&case, None);
    let counterexample = baseline
        .err()
        .ok_or(OperationalRecoveryMutationSensitivityDenial::BaselineAcceptedDefect(case.family))?;
    if counterexample.invariant() != case.expected {
        return Err(
            OperationalRecoveryMutationSensitivityDenial::WrongInvariant {
                family: case.family,
                observed: counterexample.invariant(),
            },
        );
    }
    if apply_case(&case, Some(case.defect)).is_err() {
        return Err(
            OperationalRecoveryMutationSensitivityDenial::MutatedModelStillRejected(case.family),
        );
    }
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-recovery-mutation-receipt-v1");
    digest.update([case.family as u8, case.defect as u8, case.expected as u8]);
    digest.update(counterexample.transition_identity().as_bytes());
    Ok(OperationalRecoveryMutationSensitivityReceipt {
        family: case.family,
        defect: case.defect,
        localized_invariant: case.expected,
        localized_transition: counterexample.transition_identity().to_owned(),
        receipt_identity: digest.finalize().into(),
    })
}

fn apply_case(
    case: &MutationCase,
    defect: Option<OperationalRecoveryControlledDefect>,
) -> Result<(), super::OperationalRecoveryCounterexample> {
    let mut model = OperationalRecoveryModel::default();
    for action in &case.prefix {
        model.apply(action, defect)?;
    }
    model.apply(&case.defective, defect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_operational_model_family_localizes_its_controlled_defect() {
        let suite = check_operational_recovery_mutation_sensitivity().unwrap();
        assert_eq!(suite.receipts().len(), 4);
        assert!(suite
            .receipts()
            .iter()
            .all(|receipt| receipt.localized_transition() == "defective-edge"));
    }
}
