use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::read_composition_hooks::{
    ForgeQueryReadInvariantPackContext, ForgeQueryReadInvariantPackViolation,
};
#[cfg(test)]
use super::{record_forbidden_fallback_seam_invocation, ForgeQueryForbiddenFallbackSeam};
use super::{
    ForgeQueryDerivedViewHandle, ForgeQueryGenericInspectionIntentTarget, ForgeQueryInspection,
    ForgeQueryInspectionTarget, ForgeQueryInstalledProgram, ForgeQueryLiveArtifactBinding,
    ForgeQueryLiveArtifactBundle, ForgeQueryLiveArtifactTarget, ForgeQueryLiveReadResult,
    ForgeQueryPatchBatch, ForgeQueryProgram, ForgeQueryReadBuilder,
    ForgeQueryReadDomainInvariantDenial, ForgeQueryReadFamily,
    ForgeQueryReadFamilyInvariantEvidence, ForgeQueryReadGraph, ForgeQueryReadResult,
    ForgeQueryRuntimeError, ForgeQueryWorkspace,
};

impl ForgeQueryWorkspace {
    pub fn compose_read(
        &mut self,
        declaration: impl FnOnce(
            ForgeQueryReadBuilder,
        ) -> Result<ForgeQueryReadGraph, super::ForgeQueryReadDenial>,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.compose_read_with_invariant_pack(declaration, |_context| Ok(()))
    }

    pub fn compose_read_with_invariant_pack(
        &mut self,
        declaration: impl FnOnce(
            ForgeQueryReadBuilder,
        ) -> Result<ForgeQueryReadGraph, super::ForgeQueryReadDenial>,
        invariant_pack: impl FnOnce(
            &ForgeQueryReadInvariantPackContext<'_>,
        ) -> Result<(), ForgeQueryReadInvariantPackViolation>,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        let read_graph = declaration(ForgeQueryReadBuilder::new())
            .map_err(ForgeQueryRuntimeError::ReadCompositionDenied)?;
        let invariant_context = ForgeQueryReadInvariantPackContext::new(&read_graph);
        invariant_pack(&invariant_context).map_err(|violation| {
            ForgeQueryRuntimeError::ReadCompositionDomainInvariantDenied(
                ForgeQueryReadDomainInvariantDenial::from_violation(violation, &invariant_context),
            )
        })?;
        let family = ForgeQueryReadFamily::new_kernel_only("composed_read", read_graph);
        self.read_family_intent(&family).execute()
    }

    pub fn define_read_family(
        &mut self,
        family_name: impl Into<String>,
        declaration: impl FnOnce(
            ForgeQueryReadBuilder,
        ) -> Result<ForgeQueryReadGraph, super::ForgeQueryReadDenial>,
    ) -> Result<ForgeQueryReadFamily, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        let read_graph = declaration(ForgeQueryReadBuilder::new())
            .map_err(ForgeQueryRuntimeError::ReadCompositionDenied)?;
        Ok(ForgeQueryReadFamily::new_kernel_only(
            family_name,
            read_graph,
        ))
    }

    pub fn define_read_family_with_invariant_pack(
        &mut self,
        family_name: impl Into<String>,
        invariant_family: impl Into<String>,
        declaration: impl FnOnce(
            ForgeQueryReadBuilder,
        ) -> Result<ForgeQueryReadGraph, super::ForgeQueryReadDenial>,
        invariant_pack: impl FnOnce(
            &ForgeQueryReadInvariantPackContext<'_>,
        ) -> Result<(), ForgeQueryReadInvariantPackViolation>,
    ) -> Result<ForgeQueryReadFamily, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        let read_graph = declaration(ForgeQueryReadBuilder::new())
            .map_err(ForgeQueryRuntimeError::ReadCompositionDenied)?;
        let invariant_context = ForgeQueryReadInvariantPackContext::new(&read_graph);
        invariant_pack(&invariant_context).map_err(|violation| {
            ForgeQueryRuntimeError::ReadCompositionDomainInvariantDenied(
                ForgeQueryReadDomainInvariantDenial::from_violation(violation, &invariant_context),
            )
        })?;
        let invariant_evidence = ForgeQueryReadFamilyInvariantEvidence::new(
            invariant_family,
            invariant_context.read_domain_invariant_summary(),
        );
        Ok(ForgeQueryReadFamily::new_domain_invariant_admitted(
            family_name,
            invariant_evidence,
            read_graph,
        ))
    }

    pub fn execute_read_family(
        &mut self,
        family: &ForgeQueryReadFamily,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        self.read_family_intent(family).execute()
    }

    pub fn execute_read_family_in_basis_context(
        &mut self,
        family: &ForgeQueryReadFamily,
        context: &crate::query_context::AdmittedQueryBasisContext,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        self.read_family_in_basis_context_intent(family, context)
            .execute()
    }

    pub fn read<T>(
        &mut self,
        view: &super::ForgeQueryLiveView<T>,
    ) -> Vec<crate::memory_workspace::ForgeQueryEntity> {
        self.read_live_intent(view)
            .execute()
            .expect("live view declaration admitted before workspace.read execution")
            .rows()
            .to_vec()
    }

    pub fn read_live_artifact_binding(
        &mut self,
        artifact_name: impl Into<String>,
        targets: impl IntoIterator<Item = ForgeQueryLiveArtifactTarget>,
    ) -> Result<ForgeQueryLiveArtifactBinding, ForgeQueryRuntimeError> {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(
            ForgeQueryForbiddenFallbackSeam::ReadLiveArtifactBinding,
        );
        let retained_targets = targets.into_iter().collect::<Vec<_>>();
        self.read_live_artifact_bundle(retained_targets.clone())?
            .bind_live_artifact(artifact_name, retained_targets)
    }

    pub fn read_live_artifact_bundle(
        &mut self,
        targets: impl IntoIterator<Item = ForgeQueryLiveArtifactTarget>,
    ) -> Result<ForgeQueryLiveArtifactBundle, ForgeQueryRuntimeError> {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(
            ForgeQueryForbiddenFallbackSeam::ReadLiveArtifactBundle,
        );
        let mut retained_targets = targets.into_iter().collect::<Vec<_>>();
        retained_targets.sort();
        retained_targets.dedup();

        let mut reads = BTreeMap::new();
        for target in retained_targets {
            let result = self.runtime.execute_live_read_by_name(target.view_name())?;
            reads.insert(target.view_name().to_string(), result);
        }
        let snapshot_token = live_bundle_snapshot_token(&reads)?;
        Ok(ForgeQueryLiveArtifactBundle::new(snapshot_token, reads))
    }

    pub fn observe<T>(&mut self, view: &super::ForgeQueryLiveView<T>) -> ForgeQueryPatchBatch {
        self.runtime.drain_patches(view)
    }

    pub fn materialize<T>(&self, view: &ForgeQueryDerivedViewHandle<T>) -> Vec<Value> {
        self.runtime.read_derived(view)
    }

    pub fn inspect_intent<'a, T>(
        &'a self,
        target: T,
    ) -> crate::intent_admission::ForgeQueryRuntimeInspectionIntentAuthoring<'a>
    where
        T: ForgeQueryGenericInspectionIntentTarget<'a>,
    {
        self.runtime.inspect_intent(target)
    }

    pub fn observe_computed(&mut self, view_name: &str) -> ForgeQueryPatchBatch {
        self.runtime.drain_derived_patches(view_name)
    }

    pub fn install_program(
        &mut self,
        program: ForgeQueryProgram,
    ) -> Result<ForgeQueryInstalledProgram, ForgeQueryRuntimeError> {
        self.runtime.install_program(program)
    }

    pub fn inspect<'a, T>(
        &'a self,
        target: T,
    ) -> Result<ForgeQueryInspection, ForgeQueryRuntimeError>
    where
        T: Into<ForgeQueryInspectionTarget<'a>>,
    {
        self.runtime.inspect(target)
    }
}

fn live_bundle_snapshot_token(
    reads: &BTreeMap<String, ForgeQueryLiveReadResult>,
) -> Result<String, ForgeQueryRuntimeError> {
    let snapshot_tokens = reads
        .iter()
        .map(|(view_name, result)| {
            (
                view_name.clone(),
                result.receipt().snapshot_token().to_string(),
            )
        })
        .collect::<Vec<_>>();
    let distinct_snapshot_tokens = snapshot_tokens
        .iter()
        .map(|(_, snapshot_token)| snapshot_token.clone())
        .collect::<BTreeSet<_>>();
    match snapshot_tokens.as_slice() {
        [] => Ok(String::new()),
        [(_, snapshot_token)] if distinct_snapshot_tokens.len() == 1 => Ok(snapshot_token.clone()),
        _ if distinct_snapshot_tokens.len() == 1 => Ok(snapshot_tokens[0].1.clone()),
        _ => Err(ForgeQueryRuntimeError::ReadCompositionDenied(
            super::ForgeQueryReadDenial::new(
                super::ForgeQueryReadDenialKind::ExecutionDenied,
                format!(
                    "live artifact bundle materialized multiple snapshot tokens: {}",
                    snapshot_tokens
                        .iter()
                        .map(|(view_name, snapshot_token)| format!("{view_name}:{snapshot_token}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ),
        )),
    }
}
