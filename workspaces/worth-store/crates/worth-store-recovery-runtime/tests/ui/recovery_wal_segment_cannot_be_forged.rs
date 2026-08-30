use worth_store::physical_runtime::{
    IntegrityAdmittedRecoveryWalSegment, ObservedWalArtifact,
};

fn forge(observed: &ObservedWalArtifact) -> IntegrityAdmittedRecoveryWalSegment {
    IntegrityAdmittedRecoveryWalSegment::from_complete_frames(
        observed,
        todo!(),
        Vec::new(),
    )
    .unwrap()
}

fn main() {}
