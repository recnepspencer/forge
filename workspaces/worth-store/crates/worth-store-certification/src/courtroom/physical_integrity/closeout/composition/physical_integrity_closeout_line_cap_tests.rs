use crate::{
    courtroom::harness::test_support::physical_integrity_closeout_line_cap_test_support::{
        line_cap_module_evidence, physical_integrity_owned_closeout_file_evidence,
    },
    IntegrityCloseoutModuleKind, IntegrityCompositionEvidence, IntegrityModuleCompositionEvidence,
    PhysicalIntegrityCloseoutDenial,
};

#[test]
fn closeout_rejects_line_cap_labels_without_checked_composition() {
    let denial = IntegrityModuleCompositionEvidence::checked(
        IntegrityCloseoutModuleKind::CloseoutTest,
        401,
        400,
    )
    .unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::LineCapModuleOverBudget(
            IntegrityCloseoutModuleKind::CloseoutTest
        )
    );

    let mut modules = line_cap_module_evidence();
    modules.push(
        IntegrityModuleCompositionEvidence::checked(IntegrityCloseoutModuleKind::Checksum, 1, 400)
            .unwrap(),
    );
    let denial = IntegrityCompositionEvidence::from_checked_modules(modules).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::CollapsedCloseoutResponsibility(
            IntegrityCloseoutModuleKind::Checksum
        )
    );

    let mut owned_files = physical_integrity_owned_closeout_file_evidence();
    owned_files.retain(|file| file.file_name() != "physical_integrity_closeout_suite.rs");
    let denial = IntegrityCompositionEvidence::from_checked_modules_and_owned_files(
        line_cap_module_evidence(),
        owned_files,
    )
    .unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::OmittedOwnedCloseoutFile(
            "physical_integrity_closeout_suite.rs".to_string()
        )
    );
}
