use super::authorization_policy::{
    ApplicationAuthorizationPathEffect, ApplicationAuthorizationTraversalDirection,
};
use super::canonical_basis::ApplicationSchemaCanonicalBasis;
use super::ApplicationAuthorizationPath;

pub(super) fn append_authorization_path(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    path: &ApplicationAuthorizationPath,
) {
    basis.text(format!("{prefix}.effect"), effect_name(path.effect()));
    basis.text(
        format!("{prefix}.principal-entity"),
        path.principal_entity(),
    );
    basis.text(format!("{prefix}.scope-entity"), path.scope_entity());
    basis.usize(format!("{prefix}.traversal-count"), path.traversals().len());
    for (index, traversal) in path.traversals().iter().enumerate() {
        let traversal_prefix = format!("{prefix}.traversal[{index}]");
        basis.text(format!("{traversal_prefix}.relation"), traversal.relation());
        basis.text(format!("{traversal_prefix}.from"), traversal.from());
        basis.text(format!("{traversal_prefix}.to"), traversal.to());
        basis.text(
            format!("{traversal_prefix}.direction"),
            direction_name(traversal.direction()),
        );
    }
    basis.usize(format!("{prefix}.predicate-count"), path.predicates().len());
    for (index, predicate) in path.predicates().iter().enumerate() {
        let predicate_prefix = format!("{prefix}.predicate[{index}]");
        basis.usize(
            format!("{predicate_prefix}.traversal-ordinal"),
            predicate.traversal_ordinal(),
        );
        basis.text(format!("{predicate_prefix}.entity"), predicate.entity());
        basis.text(format!("{predicate_prefix}.aspect"), predicate.aspect());
        basis.text(format!("{predicate_prefix}.field"), predicate.field());
        basis.aspect_value(format!("{predicate_prefix}.value"), predicate.value());
    }
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
