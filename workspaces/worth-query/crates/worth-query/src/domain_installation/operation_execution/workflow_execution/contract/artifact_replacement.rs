use super::WorthQueryWorkflowStageWorkspace;

impl WorthQueryWorkflowStageWorkspace<'_> {
    pub fn replace_artifact<R: crate::domain_installation::WorthQueryArtifactProviderResource>(
        &mut self,
        mut current: crate::domain_installation::WorthQueryMoveOnlyArtifactHandle,
        admission: crate::domain_installation::WorthQueryArtifactProductionAdmission,
        resource: R,
    ) -> Result<
        crate::domain_installation::WorthQueryReplacedArtifact,
        crate::domain_installation::WorthQueryArtifactReplacementStop,
    > {
        if let Err(denial) = current.validate_replacement(&admission) {
            return Err(
                crate::domain_installation::WorthQueryArtifactReplacementStop::new(denial, current),
            );
        }
        let replacement = match self.register_artifact_resource(admission, resource) {
            Ok(replacement) => replacement,
            Err(denial) => {
                return Err(
                    crate::domain_installation::WorthQueryArtifactReplacementStop::new(
                        denial, current,
                    ),
                );
            }
        };
        match current.retire_as_replaced() {
            Ok(prior) => Ok(crate::domain_installation::WorthQueryReplacedArtifact::new(
                prior,
                replacement,
            )),
            Err(denial) => {
                let _disposed_replacement = replacement.cancel();
                Err(
                    crate::domain_installation::WorthQueryArtifactReplacementStop::new(
                        denial, current,
                    ),
                )
            }
        }
    }
}
