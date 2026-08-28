mod commit_finalization;
mod event_emission;

use crate::runtime::RelationalRuntime;

pub struct LineageAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

pub(crate) struct LineagePreparationAuthority<'runtime> {
    runtime: &'runtime crate::runtime::RelationalPreparationRuntime,
}

impl RelationalRuntime {
    pub(crate) fn lineage_authority(&mut self) -> LineageAuthority<'_> {
        LineageAuthority::new(self)
    }
}

impl crate::runtime::RelationalPreparationRuntime {
    pub(crate) fn lineage_preparation_authority(&self) -> LineagePreparationAuthority<'_> {
        LineagePreparationAuthority::new(self)
    }
}

impl<'runtime> LineagePreparationAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime crate::runtime::RelationalPreparationRuntime) -> Self {
        Self { runtime }
    }
}

impl<'runtime> LineageAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}
