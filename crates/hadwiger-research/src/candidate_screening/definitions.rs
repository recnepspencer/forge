use super::{
    CandidateScreeningApplicability, CandidateScreeningInvariantAuthority,
    CandidateScreeningInvariantFamily,
};

#[derive(Clone, Copy)]
pub(crate) struct CandidateScreeningInvariantDefinition {
    pub(crate) family: CandidateScreeningInvariantFamily,
    pub(crate) key: &'static str,
    pub(crate) title: &'static str,
    pub(crate) authority: CandidateScreeningInvariantAuthority,
    pub(crate) applicability: CandidateScreeningApplicability,
    pub(crate) statement: &'static str,
    pub(crate) rejection_condition: &'static str,
    pub(crate) promotion_requirement: &'static str,
}
