use std::fmt::Debug;

use crate::data::error::SpecError;
use crate::logic::transaction::SpecDraft;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchedDomain {
    Intent,
    Topology,
    GeometryBinding,
    Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationResult<T> {
    pub value: T,
    pub touched_domains: Vec<TouchedDomain>,
    pub mutation_trace: Vec<String>,
}

#[derive(Debug, Default)]
pub struct SpecLineageRecorder;

pub trait SpecMutation: Debug {
    type Output;

    const NAME: &'static str;
    const SCHEMA_VERSION: u32 = 1;

    fn execute(
        &self,
        draft: &mut SpecDraft,
        recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError>;

    fn semantic_summary(&self) -> String {
        format!("{:?}", self)
    }
}
