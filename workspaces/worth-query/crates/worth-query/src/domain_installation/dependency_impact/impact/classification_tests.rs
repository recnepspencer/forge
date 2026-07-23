use super::*;

#[test]
fn every_dependency_role_has_one_explicit_narrowest_impact() {
    use WorthQueryImpactClass as Impact;
    use WorthQuerySemanticDependencyRole as Role;
    let matrix = [
        (Role::OperationalIdentity, Impact::Replacement),
        (Role::SelectionOrMembership, Impact::MembershipSplice),
        (Role::Ordering, Impact::ReorderOrRegroup),
        (Role::ProjectedValue, Impact::ValuePatch),
        (Role::Grouping, Impact::ReorderOrRegroup),
        (Role::WindowBoundary, Impact::WindowShift),
        (Role::SupportAndLifecycle, Impact::ExplicitRebind),
        (
            Role::ConditionalEligibilityOrSemanticCleanliness,
            Impact::ValuePatch,
        ),
        (Role::InstalledDomainInvariant, Impact::Reexecute),
        (Role::AdvisoryOnlyContext, Impact::UnaffectedOrSuppressed),
    ];
    for (role, expected) in matrix {
        assert_eq!(class_for_role(role), expected, "role {role:?}");
    }
}

#[test]
fn only_a_changed_computation_may_enter_output_impact() {
    use WorthQueryConditionalOutcomeClass as Outcome;
    assert!(!conditional_suppresses_output(Outcome::ComputedChanged));
    for outcome in [
        Outcome::ComputedRevertedClean,
        Outcome::DependencyUnchanged,
        Outcome::Suppressed,
        Outcome::DeferredByCondition,
        Outcome::DeferredTemporal,
        Outcome::DeferredOnDemand,
    ] {
        assert!(
            conditional_suppresses_output(outcome),
            "outcome {outcome:?}"
        );
    }
}

#[test]
fn impact_classes_have_a_conservative_total_widening_order() {
    use WorthQueryImpactClass as Impact;
    let classes = [
        Impact::UnaffectedOrSuppressed,
        Impact::ValuePatch,
        Impact::MembershipSplice,
        Impact::ReorderOrRegroup,
        Impact::WindowShift,
        Impact::Reexecute,
        Impact::ExplicitRebind,
        Impact::Replacement,
        Impact::Retirement,
        Impact::UnsupportedEscalation,
    ];
    for left in classes {
        for right in classes {
            let widened = widen_impact(left, right);
            assert_eq!(widened, widen_impact(right, left));
            assert_eq!(
                impact_widening_rank(widened),
                impact_widening_rank(left).max(impact_widening_rank(right))
            );
        }
        assert_eq!(widen_impact(left, left), left);
    }
}
