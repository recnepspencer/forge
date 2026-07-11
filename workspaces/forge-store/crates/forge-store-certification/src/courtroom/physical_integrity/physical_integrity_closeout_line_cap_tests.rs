use crate::{
    courtroom::harness::test_support::physical_integrity_closeout_line_cap_test_support::{
        line_cap_module_evidence, s3_owned_closeout_file_evidence,
    },
    PhysicalIntegrityCloseoutDenial, S3CloseoutModuleKind, S3LineCapCompositionEvidence,
    S3LineCapModuleEvidence,
};

#[test]
fn closeout_rejects_line_cap_labels_without_checked_composition() {
    let denial =
        S3LineCapModuleEvidence::checked(S3CloseoutModuleKind::CloseoutTest, 401, 400).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::LineCapModuleOverBudget(
            S3CloseoutModuleKind::CloseoutTest
        )
    );

    let mut modules = line_cap_module_evidence();
    modules.push(S3LineCapModuleEvidence::checked(S3CloseoutModuleKind::Checksum, 1, 400).unwrap());
    let denial = S3LineCapCompositionEvidence::from_checked_modules(modules).unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::CollapsedCloseoutResponsibility(
            S3CloseoutModuleKind::Checksum
        )
    );

    let mut owned_files = s3_owned_closeout_file_evidence();
    owned_files.retain(|file| file.file_name() != "physical_integrity_closeout_suite.rs");
    let denial = S3LineCapCompositionEvidence::from_checked_modules_and_owned_files(
        line_cap_module_evidence(),
        owned_files,
    )
    .unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityCloseoutDenial::OmittedS3OwnedCloseoutFile(
            "physical_integrity_closeout_suite.rs".to_string()
        )
    );
}
