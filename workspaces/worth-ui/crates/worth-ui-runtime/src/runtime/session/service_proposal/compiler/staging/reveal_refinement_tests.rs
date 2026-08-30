//! Admission of the Focus owner's reveal refinement during coordination stage 4.

use super::tests::{family, family_receipt, reserved_staging, reserved_two_family_staging};
use super::{
    UiServiceProposalStageIssuer, UiServiceProposalStageReceipt, UiServiceProposalStagingDenial,
};

use super::super::super::UiServiceProposalOccupancyScopeIdentity;

#[test]
fn a_reveal_refinement_is_admitted_only_for_a_participating_scroll_owner_scope() {
    let (mut compiler, mut staging, proposal) =
        reserved_staging(46, &[family::Portal, family::Focus, family::Scroll]);
    let scroll_scope = UiServiceProposalOccupancyScopeIdentity::for_test(3);
    for (index, family) in [family::Portal, family::Focus, family::Scroll]
        .into_iter()
        .enumerate()
    {
        compiler
            .advance_staging(
                &mut staging,
                family_receipt(proposal, family, index as u64 + 1, 21, 31),
            )
            .unwrap();
    }
    compiler
        .advance_staging(
            &mut staging,
            UiServiceProposalStageReceipt::recorded_stage_fixture(
                proposal,
                super::super::UiServiceProposalStage::AssembleSuccessor,
                UiServiceProposalStageIssuer::ExistingPreparation,
            ),
        )
        .unwrap();

    // A refinement naming a scope no Scroll owner staged is not backed by a replan.
    let foreign = UiServiceProposalOccupancyScopeIdentity::for_test(9);
    assert_eq!(
        compiler.advance_staging(
            &mut staging,
            UiServiceProposalStageReceipt::focus_resolution(proposal, Some(foreign)),
        ),
        Err(UiServiceProposalStagingDenial::UnbackedRevealRefinement)
    );

    compiler
        .advance_staging(
            &mut staging,
            UiServiceProposalStageReceipt::focus_resolution(proposal, Some(scroll_scope)),
        )
        .unwrap();
    let batch = compiler.finish_staging(staging).unwrap();

    assert_eq!(batch.reveal_refinement(), Some(scroll_scope));
}

#[test]
fn a_reveal_refinement_without_a_scroll_owner_cannot_stage() {
    let (mut compiler, mut staging, proposal) = reserved_two_family_staging(47);
    compiler
        .advance_staging(
            &mut staging,
            family_receipt(proposal, family::Portal, 1, 21, 31),
        )
        .unwrap();
    compiler
        .advance_staging(
            &mut staging,
            family_receipt(proposal, family::Focus, 2, 22, 32),
        )
        .unwrap();
    compiler
        .advance_staging(
            &mut staging,
            UiServiceProposalStageReceipt::recorded_stage_fixture(
                proposal,
                super::super::UiServiceProposalStage::AssembleSuccessor,
                UiServiceProposalStageIssuer::ExistingPreparation,
            ),
        )
        .unwrap();

    let focus_scope = UiServiceProposalOccupancyScopeIdentity::for_test(2);
    assert_eq!(
        compiler.advance_staging(
            &mut staging,
            UiServiceProposalStageReceipt::focus_resolution(proposal, Some(focus_scope)),
        ),
        Err(UiServiceProposalStagingDenial::UnbackedRevealRefinement)
    );
}
