use std::num::NonZeroU32;

use super::{verify_mutants, verify_ordinary_process_set};
use worth_store::physical_runtime::{
    PhysicalWorkEvidenceDigest, PhysicalWorkMutantBinding, PhysicalWorkMutantExecutionContext,
    PhysicalWorkMutantLocalization, PhysicalWorkMutantOutcome, PhysicalWorkMutantSubject,
    PhysicalWorkProcessEvidence, PhysicalWorkSourceBinding,
};

#[test]
fn ordinary_process_accounting_rejects_campaign_extension() {
    let processes = (41..=45)
        .map(|identity| {
            PhysicalWorkProcessEvidence::exited_success(
                format!("ordinary-{identity}"),
                NonZeroU32::new(identity).unwrap(),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let expected = [41, 42, 43, 44].map(|identity| NonZeroU32::new(identity).unwrap());

    if verify_ordinary_process_set(&processes, expected).is_ok() {
        panic!("MUTANT_PREDICATE:c7-ordinary-process-scope-extended");
    }
}

#[test]
fn courtroom_requires_each_bounded_residency_mutant() {
    let required = crate::mutation_campaign::bounded_residency_requirements();
    let complete = required
        .iter()
        .map(|requirement| killed_mutant(requirement.identity(), requirement.predicate()))
        .collect::<Vec<_>>();
    assert!(verify_mutants(&complete).is_ok());

    for missing in required {
        let incomplete = complete
            .iter()
            .filter(|mutant| mutant.identity() != missing.identity())
            .cloned()
            .collect::<Vec<_>>();
        let denial = match verify_mutants(&incomplete) {
            Ok(()) => panic!("MUTANT_PREDICATE:bounded-residency-corpus-truncated"),
            Err(denial) => denial,
        };
        assert!(
            denial.contains(missing.predicate()),
            "wrong omission denial: {denial}"
        );
    }
}

fn killed_mutant(identity: u16, predicate: &str) -> PhysicalWorkMutantLocalization {
    let source_digest =
        PhysicalWorkEvidenceDigest::new([identity as u8; 32]).expect("nonzero fixture digest");
    let mutant_digest = PhysicalWorkEvidenceDigest::new([(identity + 1) as u8; 32])
        .expect("nonzero fixture digest");
    let subject = PhysicalWorkMutantSubject::new(identity, predicate, "current-source.rs").unwrap();
    let execution = PhysicalWorkMutantExecutionContext::new("test", "causal-scenario").unwrap();
    let binary = PhysicalWorkSourceBinding::new("current-test.exe", source_digest).unwrap();
    let binding =
        PhysicalWorkMutantBinding::new(subject, source_digest, mutant_digest, binary, execution);
    PhysicalWorkMutantLocalization::new(
        binding,
        PhysicalWorkMutantOutcome::new(true, "exact causal assertion"),
    )
    .unwrap()
}
