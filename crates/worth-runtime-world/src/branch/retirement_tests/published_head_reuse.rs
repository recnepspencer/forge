//! Exact reuse resolves the commit the branch actually carries, including
//! after a composite publication has moved that branch's head.
//!
//! The reference cell is the authority for a product head; the registry's
//! basis-to-commit index is derived from it. A publication that moved the cell
//! without re-indexing would leave exact reuse resolving the commit the branch
//! was installed with, so a creation from a head the owner had just published
//! would be denied.

use super::fork_creation::{
    current_root_observation, seed_relational_source, setup_with_relational_source,
};
use super::{create_reused_branch, reuse_intent};

#[test]
fn exact_reuse_after_a_publication_selects_the_published_commit() {
    let (_fixture, owner, published) = setup_with_relational_source(3);
    let root_commit = published.selected_commit().clone();

    let child = create_reused_branch(&owner, &published, reuse_intent("published-head-child"));

    assert_eq!(
        child.selected_commit(),
        &root_commit,
        "exact reuse selects the commit the published head names"
    );
    assert_eq!(
        child.basis(),
        published.basis(),
        "the child carries the published composite basis unchanged"
    );
    assert_ne!(child.branch_identity(), published.branch_identity());
    assert_eq!(owner.state.branches.branch_count(), 2);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    assert_eq!(
        owner.state.history.len(),
        2,
        "reuse publishes nothing: the seeded and bootstrap commits are all there is"
    );
}

/// The index exchanges the head it holds rather than accumulating heads: a
/// second publication on the same occurrence vacates the basis the first one
/// installed, so a stale basis stops resolving at the same moment the
/// published one starts.
#[test]
fn a_second_publication_vacates_the_basis_the_first_one_indexed() {
    let (mut fixture, owner, first) = setup_with_relational_source(3);
    let vacated_basis = first.basis().clone();
    assert_eq!(
        owner.state.branches.commit_for_basis(&vacated_basis),
        Some(first.selected_commit().clone()),
        "the first publication indexed the head it installed"
    );

    seed_relational_source(&owner, &mut fixture, first);
    let second = current_root_observation(&owner);

    assert_ne!(second.basis(), &vacated_basis);
    assert_eq!(
        owner.state.branches.commit_for_basis(&vacated_basis),
        None,
        "the occurrence that moved no longer counts against the basis it left"
    );
    assert_eq!(
        owner.state.branches.commit_for_basis(second.basis()),
        Some(second.selected_commit().clone()),
        "the basis the branch now carries resolves to the commit it now carries"
    );
}
