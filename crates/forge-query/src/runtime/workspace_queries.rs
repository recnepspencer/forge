use serde_json::Value;

use super::read_composition_hooks::{
    ForgeQueryReadInvariantPackContext, ForgeQueryReadInvariantPackViolation,
};
use super::{
    ForgeQueryDerivedViewHandle, ForgeQueryGenericInspectionIntentTarget, ForgeQueryInspection,
    ForgeQueryInspectionTarget, ForgeQueryInstalledProgram, ForgeQueryPatchBatch,
    ForgeQueryProgram, ForgeQueryReadBuilder, ForgeQueryReadDomainInvariantDenial,
    ForgeQueryReadFamily, ForgeQueryReadFamilyInvariantEvidence, ForgeQueryReadGraph,
    ForgeQueryReadResult, ForgeQueryRuntimeError, ForgeQueryWorkspace,
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
