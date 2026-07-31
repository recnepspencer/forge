use worth_foundational::facade::{
    canonical_basis_value_for_aspect_value, canonicalization, prepare_canonical_basis_sequence,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial,
    CanonicalDigestId, CanonicalDigestWorkBudget, CanonicalIntegerWidth,
    CanonicalizationRuleVersion,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationAuthorizationPath, ApplicationAuthorizationPathEffect,
    ApplicationAuthorizationTraversalDirection,
};

use crate::canonical_work::WorthQueryCanonicalWorkEvidence;

const DOMAIN: &str = "worth-query.authorization-path";
const RULE: &str = "worth-query-authorization-path-v2";
const PATH_BUDGET: CanonicalDigestWorkBudget =
    match CanonicalDigestWorkBudget::new(4_096, 1_024 * 1_024) {
        Some(budget) => budget,
        None => panic!("fixed authorization path canonical-work budget is valid"),
    };

pub(super) struct PreparedAuthorizationPathIdentity {
    pub(super) digest: CanonicalDigestId,
    pub(super) work: WorthQueryCanonicalWorkEvidence,
}

pub(super) fn prepare_authorization_path_identity(
    path: &ApplicationAuthorizationPath,
) -> Result<PreparedAuthorizationPathIdentity, CanonicalDigestDerivationDenial> {
    let version =
        CanonicalizationRuleVersion::new(RULE).expect("fixed authorization path rule is valid");
    let basis = prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Future(DOMAIN),
        path_entries(path),
    )
    .into_result()
    .expect("authorization paths always have a nonempty canonical basis");
    let ready = canonicalization()
        .digest()
        .for_sequence_with_budget(basis, CanonicalDigestAlgorithmId::sha256(), PATH_BUDGET)
        .into_result()?;
    let derived = canonicalization().digest().derive(ready);
    Ok(PreparedAuthorizationPathIdentity {
        digest: CanonicalDigestId::new(*derived.value().bytes()),
        work: WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
    })
}

fn path_entries(path: &ApplicationAuthorizationPath) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text("family", "authorization-path"),
        text("effect", effect_name(path.effect())),
        text("principal", path.principal_entity()),
        text("scope", path.scope_entity()),
        unsigned("traversal-count", path.traversals().len()),
    ];
    for (ordinal, traversal) in path.traversals().iter().enumerate() {
        let prefix = format!("traversal.{ordinal}");
        entries.extend([
            text(format!("{prefix}.relation"), traversal.relation()),
            text(format!("{prefix}.from"), traversal.from()),
            text(format!("{prefix}.to"), traversal.to()),
            text(
                format!("{prefix}.direction"),
                direction_name(traversal.direction()),
            ),
        ]);
    }
    entries.push(unsigned("predicate-count", path.predicates().len()));
    for (ordinal, predicate) in path.predicates().iter().enumerate() {
        let prefix = format!("predicate.{ordinal}");
        entries.extend([
            unsigned(
                format!("{prefix}.traversal-ordinal"),
                predicate.traversal_ordinal(),
            ),
            text(format!("{prefix}.entity"), predicate.entity()),
            text(format!("{prefix}.aspect"), predicate.aspect()),
            text(format!("{prefix}.field"), predicate.field()),
            entry(
                format!("{prefix}.value"),
                canonical_basis_value_for_aspect_value(predicate.value()),
            ),
        ]);
    }
    entries
}

fn text(locus: impl Into<String>, value: impl Into<String>) -> CanonicalBasisEntry {
    entry(locus, CanonicalBasisValue::ExactText(value.into().into()))
}

fn unsigned(locus: impl Into<String>, value: usize) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u64::try_from(value)
                .expect("authorization path structural counts fit in u64")
                .into(),
        },
    )
}

fn entry(locus: impl Into<String>, value: CanonicalBasisValue) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Future(DOMAIN),
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Field,
        value,
    )
}

const fn effect_name(effect: ApplicationAuthorizationPathEffect) -> &'static str {
    match effect {
        ApplicationAuthorizationPathEffect::Allow => "allow",
        ApplicationAuthorizationPathEffect::Deny => "deny",
    }
}

const fn direction_name(direction: ApplicationAuthorizationTraversalDirection) -> &'static str {
    match direction {
        ApplicationAuthorizationTraversalDirection::Forward => "forward",
        ApplicationAuthorizationTraversalDirection::Reverse => "reverse",
    }
}
