use super::scope_narrowing::{
    ApplicationCapabilityAmountScope, ApplicationCapabilityAmountValue,
    ApplicationCapabilityContextScope, ApplicationCapabilityDelegationScope,
    ApplicationCapabilityLimitScope, ApplicationCapabilityOptionalValueSet,
    ApplicationCapabilityScope, ApplicationCapabilityTargetScope,
    ApplicationCapabilityValidityWindow, ApplicationCapabilityValue,
};

#[derive(Clone, Copy)]
enum Axis {
    Action,
    Resource,
    Relation,
    Field,
    Purpose,
    Amount,
    Cardinality,
    Workflow,
    Validity,
    Delegation,
    Provenance,
    Context,
}

#[test]
fn every_scope_dimension_participates_in_narrowing() {
    let parent = scope(None, false);
    let lawful_child = scope(None, true);
    assert!(lawful_child.is_within(&parent));

    for axis in [
        Axis::Action,
        Axis::Resource,
        Axis::Relation,
        Axis::Field,
        Axis::Purpose,
        Axis::Amount,
        Axis::Cardinality,
        Axis::Workflow,
        Axis::Validity,
        Axis::Delegation,
        Axis::Provenance,
        Axis::Context,
    ] {
        assert!(
            !scope(Some(axis), true).is_within(&parent),
            "one widened dimension must deny the whole scope"
        );
    }
}

#[test]
fn not_applicable_is_exact_posture_not_global_authority() {
    let values = ApplicationCapabilityOptionalValueSet::from_typed([1_u64]).unwrap();
    let not_applicable = ApplicationCapabilityOptionalValueSet::not_applicable();
    let parent = scope_with_relation(not_applicable.clone());
    let child = scope_with_relation(values);
    assert!(!child.is_within(&parent));

    let parent =
        scope_with_relation(ApplicationCapabilityOptionalValueSet::from_typed([1_u64]).unwrap());
    let child = scope_with_relation(not_applicable);
    assert!(!child.is_within(&parent));
}

fn scope(axis: Option<Axis>, child: bool) -> ApplicationCapabilityScope {
    let action = value(if matches!(axis, Some(Axis::Action)) {
        2
    } else {
        1
    });
    let resource = value(if matches!(axis, Some(Axis::Resource)) {
        101
    } else {
        100
    });
    let relation = value_set(if matches!(axis, Some(Axis::Relation)) {
        vec![3]
    } else if child {
        vec![1]
    } else {
        vec![1, 2]
    });
    let field = value_set(if matches!(axis, Some(Axis::Field)) {
        vec![30]
    } else if child {
        vec![10]
    } else {
        vec![10, 20]
    });
    let purpose = value(if matches!(axis, Some(Axis::Purpose)) {
        6
    } else {
        5
    });
    let target = ApplicationCapabilityTargetScope::new(action, resource, relation, field, purpose);

    let amount_units = if matches!(axis, Some(Axis::Amount)) {
        15_000
    } else if child {
        5_000
    } else {
        10_000
    };
    let amount = ApplicationCapabilityAmountScope::ceiling(
        ApplicationCapabilityAmountValue::new("USD", 2, amount_units).unwrap(),
    );
    let cardinality = if matches!(axis, Some(Axis::Cardinality)) {
        11
    } else if child {
        5
    } else {
        10
    };
    let workflow = value(if matches!(axis, Some(Axis::Workflow)) {
        4
    } else {
        3
    });
    let (not_before, not_after) = if matches!(axis, Some(Axis::Validity)) {
        (-1, 90)
    } else if child {
        (10, 90)
    } else {
        (0, 100)
    };
    let validity =
        ApplicationCapabilityValidityWindow::new("bank-time", not_before, not_after).unwrap();
    let limits =
        ApplicationCapabilityLimitScope::new(amount, cardinality, workflow, validity).unwrap();

    let remaining = if matches!(axis, Some(Axis::Delegation)) {
        4
    } else if child {
        2
    } else {
        3
    };
    let provenance = if matches!(axis, Some(Axis::Provenance)) {
        vec!["foreign"]
    } else if child {
        vec!["root", "child"]
    } else {
        vec!["root"]
    };
    let delegation = ApplicationCapabilityDelegationScope::new(remaining, provenance).unwrap();
    let context = if matches!(axis, Some(Axis::Context)) {
        ApplicationCapabilityContextScope::new([("case", value(2))]).unwrap()
    } else if child {
        ApplicationCapabilityContextScope::new([("case", value(1)), ("branch", value(2))]).unwrap()
    } else {
        ApplicationCapabilityContextScope::new([("case", value(1))]).unwrap()
    };
    ApplicationCapabilityScope::new(target, limits, delegation, context)
}

fn scope_with_relation(
    relation: ApplicationCapabilityOptionalValueSet,
) -> ApplicationCapabilityScope {
    let target = ApplicationCapabilityTargetScope::new(
        value(1),
        value(100),
        relation,
        ApplicationCapabilityOptionalValueSet::not_applicable(),
        value(5),
    );
    let limits = ApplicationCapabilityLimitScope::new(
        ApplicationCapabilityAmountScope::not_applicable(),
        1,
        value(3),
        ApplicationCapabilityValidityWindow::new("bank-time", 0, 100).unwrap(),
    )
    .unwrap();
    ApplicationCapabilityScope::new(
        target,
        limits,
        ApplicationCapabilityDelegationScope::new(0, ["root"]).unwrap(),
        ApplicationCapabilityContextScope::new(std::iter::empty::<(
            String,
            ApplicationCapabilityValue,
        )>())
        .unwrap(),
    )
}

fn value(value: u64) -> ApplicationCapabilityValue {
    ApplicationCapabilityValue::from_typed(value)
}

fn value_set(values: Vec<u64>) -> ApplicationCapabilityOptionalValueSet {
    ApplicationCapabilityOptionalValueSet::from_typed(values).unwrap()
}
