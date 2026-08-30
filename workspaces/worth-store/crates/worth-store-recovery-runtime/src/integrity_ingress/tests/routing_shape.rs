use worth_store_physical_format::integrity_declarations::PhysicalIntegrityArtifactFamily;

use super::super::IntegrityAdmittedRecoveryArtifact;

#[test]
fn routing_is_exhaustive_over_every_current_recovery_family() {
    fn family(artifact: &IntegrityAdmittedRecoveryArtifact<'_>) -> PhysicalIntegrityArtifactFamily {
        match artifact {
            IntegrityAdmittedRecoveryArtifact::BootstrapCatalog(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::CurrentSelector(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::PreviousSelector(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::RootManifest(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::RootRoutingBlock(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::SegmentMembershipBlock(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::PageFrame(value) => value.scope().artifact_family(),
            IntegrityAdmittedRecoveryArtifact::ExtentManifest(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::ExtentChunk(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::WalFrame(value) => value.scope().artifact_family(),
            IntegrityAdmittedRecoveryArtifact::CheckpointStreamHeader(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::CheckpointDirtyBasis(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::CheckpointBindingCompaction(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::CheckpointBinding(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::CheckpointFooter(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::FreeSpaceHeader(value) => {
                value.scope().artifact_family()
            }
            IntegrityAdmittedRecoveryArtifact::FreeSpaceMembershipBlock(value) => {
                value.scope().artifact_family()
            }
        }
    }
    let _ = family;
}
