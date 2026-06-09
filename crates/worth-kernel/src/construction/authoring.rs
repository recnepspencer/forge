use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryWorkspace,
};

pub(crate) const REQUIRED_QUERY_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 2] = [
    ForgeQueryRuntimeFacadeFamily::Write,
    ForgeQueryRuntimeFacadeFamily::Inspect,
];

pub(crate) fn admit_primitive_construction_query_family(
    workspace: &ForgeQueryWorkspace,
    family: ForgeQueryRuntimeFacadeFamily,
) -> Result<ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryRuntimeError> {
    workspace.admit_public_api_family(family)
}

fn require_primitive_construction_query_entry(
    workspace: &ForgeQueryWorkspace,
) -> Result<(), ForgeQueryRuntimeError> {
    for family in REQUIRED_QUERY_FAMILIES {
        admit_primitive_construction_query_family(workspace, family)?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryEntryError {
    Authority(ForgeQueryRuntimeError),
}

impl From<ForgeQueryRuntimeError> for PrimitiveConstructionQueryEntryError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::Authority(value)
    }
}

impl std::fmt::Display for PrimitiveConstructionQueryEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => write!(f, "{error:?}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryEntryError {}

pub(crate) fn require_primitive_construction_query_authority(
    workspace: &ForgeQueryWorkspace,
) -> Result<(), PrimitiveConstructionQueryEntryError> {
    require_primitive_construction_query_entry(workspace)?;
    Ok(())
}
