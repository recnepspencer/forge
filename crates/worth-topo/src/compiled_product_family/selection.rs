use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::validation::DerivedTopologyValidationReport;

use super::admitted_input::TopologyCompiledProductFamilyAdmittedInput;
use super::catalog::TopologyCompiledProductFamilyCatalog;
use super::compiled_product::{
    lower_topology_compiled_product_identity, TopologyCompiledProductLoweredIdentity,
};
use super::declaration::TopologyCompiledProductFamilyDeclaration;
use super::error::{TopologyCompiledProductFamilyError, TopologyCompiledProductFamilyErrorKind};

#[derive(Debug, Clone)]
pub struct SelectedTopologyCompiledProductFamily {
    declaration: TopologyCompiledProductFamilyDeclaration,
    admitted_input: TopologyCompiledProductFamilyAdmittedInput,
}

impl SelectedTopologyCompiledProductFamily {
    pub fn declaration(&self) -> &TopologyCompiledProductFamilyDeclaration {
        &self.declaration
    }

    pub fn admitted_input(&self) -> &TopologyCompiledProductFamilyAdmittedInput {
        &self.admitted_input
    }

    pub fn compile_product_identity(
        &self,
        materialized: &MaterializedTopologyView,
        interpreted: &InterpretedTopologyView,
        validation: &DerivedTopologyValidationReport,
    ) -> Result<TopologyCompiledProductLoweredIdentity, TopologyCompiledProductFamilyError> {
        lower_topology_compiled_product_identity(
            &self.declaration,
            &self.admitted_input,
            materialized,
            interpreted,
            validation,
        )
    }
}

pub fn select_topology_compiled_product_family(
    catalog: &TopologyCompiledProductFamilyCatalog,
    admitted_input: TopologyCompiledProductFamilyAdmittedInput,
) -> Result<SelectedTopologyCompiledProductFamily, TopologyCompiledProductFamilyError> {
    let declaration = catalog
        .family(admitted_input.family_identity())
        .ok_or_else(|| {
            TopologyCompiledProductFamilyError::new(
                TopologyCompiledProductFamilyErrorKind::NoDeclaredFamilyForConsumer,
                "admitted topology compiled-product family input referenced a missing declaration",
            )
        })?;
    if !declaration.supports(admitted_input.consumer()) {
        return Err(TopologyCompiledProductFamilyError::new(
            TopologyCompiledProductFamilyErrorKind::NoDeclaredFamilyForConsumer,
            "admitted topology compiled-product family input referenced an undeclared consumer",
        ));
    }
    Ok(SelectedTopologyCompiledProductFamily {
        declaration: declaration.clone(),
        admitted_input,
    })
}
