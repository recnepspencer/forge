use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;
use worth_query_installation::facade::{
    WorthQueryInstalledGraphAdmissionAuthority, WorthQueryInstalledGraphObligationKind as Kind,
    WorthQueryInstalledGraphObligationOwner as Owner,
    WorthQueryInstalledGraphObligationSetIdentity,
};

use super::{WorthQueryGraphWorkIntent, WorthQuerySelectedGraphObligations};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryGraphWorkRequirementCounters {
    selected_rows_consumed: usize,
    owner_progressions_checked: usize,
    requirement_rows: usize,
    canonical_preparations: usize,
    digest_derivations: usize,
}

impl WorthQueryGraphWorkRequirementCounters {
    pub const fn selected_rows_consumed(self) -> usize {
        self.selected_rows_consumed
    }

    pub const fn owner_progressions_checked(self) -> usize {
        self.owner_progressions_checked
    }

    pub const fn requirement_rows(self) -> usize {
        self.requirement_rows
    }

    pub const fn canonical_preparations(self) -> usize {
        self.canonical_preparations
    }

    pub const fn digest_derivations(self) -> usize {
        self.digest_derivations
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphWorkRequirementDenialKind {
    ForeignAdmissionAuthority,
    UnsupportedOwnerProgression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphWorkRequirementDenial {
    kind: WorthQueryGraphWorkRequirementDenialKind,
    subject: String,
}

impl WorthQueryGraphWorkRequirementDenial {
    fn unsupported(subject: impl Into<String>) -> Self {
        Self {
            kind: WorthQueryGraphWorkRequirementDenialKind::UnsupportedOwnerProgression,
            subject: subject.into(),
        }
    }

    fn foreign_authority(subject: impl Into<String>) -> Self {
        Self {
            kind: WorthQueryGraphWorkRequirementDenialKind::ForeignAdmissionAuthority,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryGraphWorkRequirementDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

/// Sealed requirement proof produced only from selected installed authority.
///
/// ```compile_fail
/// use worth_query_admission::facade::graph_obligation::WorthQueryRequiredGraphWork;
///
/// let forged = WorthQueryRequiredGraphWork {
///     selected: todo!(),
///     counters: todo!(),
/// };
/// ```
#[derive(Debug)]
pub struct WorthQueryRequiredGraphWork {
    selected: WorthQuerySelectedGraphObligations,
    counters: WorthQueryGraphWorkRequirementCounters,
}

pub fn require_selected_graph_work(
    selected: WorthQuerySelectedGraphObligations,
    authority: &WorthQueryInstalledGraphAdmissionAuthority,
) -> Result<WorthQueryRequiredGraphWork, WorthQueryGraphWorkRequirementDenial> {
    if !authority.admits(selected.binding_identity()) {
        return Err(WorthQueryGraphWorkRequirementDenial::foreign_authority(
            selected.subject_name(),
        ));
    }
    let mut counters = WorthQueryGraphWorkRequirementCounters::default();
    for row in selected.rows() {
        counters.selected_rows_consumed += 1;
        counters.owner_progressions_checked += 1;
        if !owner_progression_is_exact(row.kind(), row.owner_progression()) {
            return Err(WorthQueryGraphWorkRequirementDenial::unsupported(
                selected.subject_name(),
            ));
        }
        counters.requirement_rows += 1;
    }
    Ok(WorthQueryRequiredGraphWork { selected, counters })
}

impl WorthQueryRequiredGraphWork {
    pub const fn identity(&self) -> &WorthQueryInstalledGraphObligationSetIdentity {
        self.selected.identity()
    }

    pub const fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        self.selected.binding_identity()
    }

    pub fn subject_name(&self) -> &str {
        self.selected.subject_name()
    }

    pub const fn intent(&self) -> WorthQueryGraphWorkIntent {
        self.selected.intent()
    }

    pub const fn counters(&self) -> WorthQueryGraphWorkRequirementCounters {
        self.counters
    }

    pub fn inspect(&self) -> WorthQueryRequiredGraphWorkInspection<'_> {
        WorthQueryRequiredGraphWorkInspection { required: self }
    }

    pub(super) fn selected(&self) -> &WorthQuerySelectedGraphObligations {
        &self.selected
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WorthQueryRequiredGraphWorkInspection<'a> {
    required: &'a WorthQueryRequiredGraphWork,
}

impl WorthQueryRequiredGraphWorkInspection<'_> {
    pub fn identity(&self) -> &WorthQueryInstalledGraphObligationSetIdentity {
        self.required.identity()
    }

    pub const fn counters(&self) -> WorthQueryGraphWorkRequirementCounters {
        self.required.counters()
    }
}

fn owner_progression_is_exact(kind: Kind, owners: &[Owner]) -> bool {
    match kind {
        Kind::GraphRead => owners == [Owner::Relational, Owner::QueryExecution],
        Kind::AuthorizationObservation => {
            owners == [Owner::Relational, Owner::QueryAdmission]
                || owners
                    == [
                        Owner::Relational,
                        Owner::RuntimeBridge,
                        Owner::Signal,
                        Owner::QueryAdmission,
                    ]
        }
        Kind::MutationTouch => owners == [Owner::Relational, Owner::QueryAdmission],
        Kind::EffectApplication | Kind::InvariantExecution => {
            owners == [Owner::QueryExecution, Owner::Relational]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{owner_progression_is_exact, Kind, Owner};

    #[test]
    fn every_obligation_kind_rejects_incomplete_or_reordered_owner_authority() {
        let hostile = [
            (Kind::GraphRead, &[Owner::Relational][..]),
            (
                Kind::AuthorizationObservation,
                &[Owner::RuntimeBridge, Owner::Signal, Owner::QueryAdmission][..],
            ),
            (
                Kind::MutationTouch,
                &[Owner::QueryAdmission, Owner::Relational][..],
            ),
            (
                Kind::EffectApplication,
                &[Owner::Relational, Owner::QueryExecution][..],
            ),
            (Kind::InvariantExecution, &[Owner::QueryExecution][..]),
        ];

        for (kind, owners) in hostile {
            assert!(!owner_progression_is_exact(kind, owners));
        }
    }

    #[test]
    fn exact_installed_owner_routes_remain_admissible() {
        let exact = [
            (
                Kind::GraphRead,
                &[Owner::Relational, Owner::QueryExecution][..],
            ),
            (
                Kind::AuthorizationObservation,
                &[Owner::Relational, Owner::QueryAdmission][..],
            ),
            (
                Kind::AuthorizationObservation,
                &[
                    Owner::Relational,
                    Owner::RuntimeBridge,
                    Owner::Signal,
                    Owner::QueryAdmission,
                ][..],
            ),
            (
                Kind::MutationTouch,
                &[Owner::Relational, Owner::QueryAdmission][..],
            ),
            (
                Kind::EffectApplication,
                &[Owner::QueryExecution, Owner::Relational][..],
            ),
            (
                Kind::InvariantExecution,
                &[Owner::QueryExecution, Owner::Relational][..],
            ),
        ];

        for (kind, owners) in exact {
            assert!(owner_progression_is_exact(kind, owners));
        }
    }
}
