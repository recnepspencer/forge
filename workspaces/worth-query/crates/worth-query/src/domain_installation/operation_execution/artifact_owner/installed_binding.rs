use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct WorthQueryInstalledWorkflowArtifactContracts {
    input:
        Option<Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>>,
    output:
        Option<Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>>,
}

impl WorthQueryInstalledWorkflowArtifactContracts {
    pub(crate) fn input(
        &self,
    ) -> Option<&Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>>
    {
        self.input.as_ref()
    }

    pub(crate) fn output(
        &self,
    ) -> Option<&Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>>
    {
        self.output.as_ref()
    }
}

pub(crate) fn compile_workflow_artifact_contracts(
    owner: &str,
    stages: &[worth_query_installation::facade::WorthQueryPortableWorkflowStage],
    portable_index: &worth_query_installation::facade::WorthQueryInstalledPackageIndex,
) -> BTreeMap<String, WorthQueryInstalledWorkflowArtifactContracts> {
    stages
        .iter()
        .map(|stage| {
            let input = installed_contract(
                owner,
                &stage.semantics().input,
                portable_index,
                "workflow input artifact contract must be installed",
            );
            let output = installed_contract(
                owner,
                &stage.semantics().output,
                portable_index,
                "workflow output artifact contract must be installed",
            );
            (
                stage.identity().to_owned(),
                WorthQueryInstalledWorkflowArtifactContracts { input, output },
            )
        })
        .collect()
}

fn installed_contract(
    owner: &str,
    value: &worth_query_installation::facade::WorthQueryWorkflowValueContract,
    portable_index: &worth_query_installation::facade::WorthQueryInstalledPackageIndex,
    message: &'static str,
) -> Option<Arc<worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority>> {
    let worth_query_installation::facade::WorthQueryWorkflowValueContract::InstalledArtifact(
        reference,
    ) = value
    else {
        return None;
    };
    let authority = portable_index
        .artifact_contract(
            owner,
            reference.family().as_str(),
            reference.schema_version(),
            reference.protocol_version(),
        )
        .expect(message);
    portable_index
        .validate_artifact_contract(&authority)
        .expect("newly minted workflow artifact authority must validate");
    Some(Arc::new(authority))
}
