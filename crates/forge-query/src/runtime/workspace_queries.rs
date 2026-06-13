use std::collections::{BTreeMap, BTreeSet};

use crate::memory_workspace::ForgeQuerySnapshotIdentity;
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
    ForgeQueryRuntimeError, ForgeQueryUnifiedInspectionResult, ForgeQueryWorkspace,
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

    pub fn state_live_by_name(
        &self,
        view_name: &str,
    ) -> Result<super::ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        super::state::snapshot_live_view_name(&self.runtime, view_name)
    }

    pub fn read_live_by_name(
        &mut self,
        view_name: &str,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        let installation = self
            .runtime
            .inspect_live_view_name_installation(view_name)?
            .clone();
        let review = self.review_live_read_execution(super::ForgeQueryLiveView::<Value>::new(
            crate::memory_workspace::ForgeQueryLiveViewHandle::new(view_name),
            installation,
        ))?;
        let handoff = self.resolve_reviewed_admitted_live_read_execution_handoff(review)?;
        let binding = self.into_runtime_live_read_execution_binding(handoff);
        self.execute_bound_live_read_execution(binding)
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
        let snapshot_identity = live_bundle_snapshot_identity(&reads)?;
        Ok(ForgeQueryLiveArtifactBundle::new(snapshot_identity, reads))
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

    pub fn inspect_live_by_name(
        &self,
        view_name: &str,
    ) -> Result<ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
        self.runtime.inspect_live_view_name_result(view_name)
    }

    pub fn subscription_basis_digest_by_name(
        &self,
        view_name: &str,
    ) -> Result<String, ForgeQueryRuntimeError> {
        Ok(self
            .runtime
            .inspect_live_view_name_installation(view_name)?
            .basis_binding_for_reporting()
            .to_string())
    }
}

fn live_bundle_snapshot_identity(
    reads: &BTreeMap<String, ForgeQueryLiveReadResult>,
) -> Result<ForgeQuerySnapshotIdentity, ForgeQueryRuntimeError> {
    let snapshot_identities = reads
        .iter()
        .map(|(view_name, result)| {
            (
                view_name.clone(),
                result.receipt().snapshot_identity().clone(),
            )
        })
        .collect::<Vec<_>>();
    let distinct_snapshot_identities = snapshot_identities
        .iter()
        .map(|(_, snapshot_identity)| snapshot_identity.evidence_identity().to_string())
        .collect::<BTreeSet<_>>();
    match snapshot_identities.as_slice() {
        [] => Ok(ForgeQuerySnapshotIdentity::empty_relational_state()),
        [(_, snapshot_identity)] if distinct_snapshot_identities.len() == 1 => {
            Ok(snapshot_identity.clone())
        }
        _ if distinct_snapshot_identities.len() == 1 => Ok(snapshot_identities[0].1.clone()),
        _ => Err(ForgeQueryRuntimeError::ReadCompositionDenied(
            super::ForgeQueryReadDenial::new(
                super::ForgeQueryReadDenialKind::ExecutionDenied,
                format!(
                    "live artifact bundle materialized multiple snapshot identities: {}",
                    snapshot_identities
                        .iter()
                        .map(|(view_name, snapshot_identity)| {
                            format!("{view_name}:{}", snapshot_identity.evidence_identity())
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ),
        )),
    }
}
