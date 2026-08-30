use worth_query_installation::facade::{
    WorthQueryDecisionFactKind, WorthQueryOperationEffectFamily,
};

use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

pub(super) fn decision_kind_tag(value: &WorthQueryDecisionFactKind) -> u16 {
    match value {
        WorthQueryDecisionFactKind::ObservedValue => 1,
        WorthQueryDecisionFactKind::AbsenceOrNonMembership => 2,
        WorthQueryDecisionFactKind::PredicateOrComparison => 3,
        WorthQueryDecisionFactKind::OrderingOrExtremum => 4,
        WorthQueryDecisionFactKind::CardinalityUniquenessOrOwnership => 5,
        WorthQueryDecisionFactKind::TraversalFrontierOrPath => 6,
        WorthQueryDecisionFactKind::AccessProductCoverageOrMembership => 7,
        WorthQueryDecisionFactKind::ArtifactSemanticProjection => 8,
        WorthQueryDecisionFactKind::DomainStructuralProof => 9,
    }
}

pub(super) fn decision_kind(tag: u16) -> Result<WorthQueryDecisionFactKind, Denial> {
    Ok(match tag {
        1 => WorthQueryDecisionFactKind::ObservedValue,
        2 => WorthQueryDecisionFactKind::AbsenceOrNonMembership,
        3 => WorthQueryDecisionFactKind::PredicateOrComparison,
        4 => WorthQueryDecisionFactKind::OrderingOrExtremum,
        5 => WorthQueryDecisionFactKind::CardinalityUniquenessOrOwnership,
        6 => WorthQueryDecisionFactKind::TraversalFrontierOrPath,
        7 => WorthQueryDecisionFactKind::AccessProductCoverageOrMembership,
        8 => WorthQueryDecisionFactKind::ArtifactSemanticProjection,
        9 => WorthQueryDecisionFactKind::DomainStructuralProof,
        _ => return unsupported(),
    })
}

pub(super) fn effect_tag(value: WorthQueryOperationEffectFamily) -> u16 {
    match value {
        WorthQueryOperationEffectFamily::Mutation => 1,
        WorthQueryOperationEffectFamily::Merge => 2,
        WorthQueryOperationEffectFamily::Writeback => 3,
    }
}

pub(super) fn effect(tag: u16) -> Result<WorthQueryOperationEffectFamily, Denial> {
    match tag {
        1 => Ok(WorthQueryOperationEffectFamily::Mutation),
        2 => Ok(WorthQueryOperationEffectFamily::Merge),
        3 => Ok(WorthQueryOperationEffectFamily::Writeback),
        _ => unsupported(),
    }
}

fn unsupported<T>() -> Result<T, Denial> {
    Err(Denial::new(Kind::UnsupportedRecordVariant))
}
