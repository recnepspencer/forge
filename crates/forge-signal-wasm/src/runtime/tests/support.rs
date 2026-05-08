pub(crate) use crate::expression::model::{Expr, IdentitySpec, SignalValue};
pub(crate) use crate::recipe::model::{
    KeyedRecipeFamilySpec, KeyedSetValue, KeyedSourceFamilySpec, RecipeFamilyReadSpec,
    RecipeReadSpec, RecipeSpec, SourceSpec, TransactionOp,
};
pub(crate) use crate::runtime::adapters::RuntimeEnvelope;
pub(crate) use crate::runtime::compute_callbacks;
pub(crate) use crate::runtime::compute_callbacks::{
    ComputeCallbackFailure, ComputeCallbackFailureClass,
};
pub(crate) use crate::runtime::core::RuntimeCore;
pub(crate) use crate::runtime::policy::{RuntimePolicyPreset, RuntimePolicySpec};
pub(crate) use std::cell::RefCell;
pub(crate) use std::rc::Rc;

pub(crate) fn number(value: f64) -> Expr {
    Expr::Value {
        value: SignalValue::Number(value),
    }
}

pub(crate) fn read(id: &str) -> Expr {
    Expr::Read { id: id.to_owned() }
}

pub(crate) fn assert_digest_shape(digest: &str) {
    assert_eq!(digest.len(), 64);
    assert!(digest
        .chars()
        .all(|character| character.is_ascii_hexdigit()));
}

pub(crate) fn build_adversarial_merge_runtime(
    policy: RuntimePolicySpec,
) -> (RuntimeCore, u64, u64, String) {
    let mut runtime = RuntimeCore::new(policy).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearTeeth".to_owned(),
            initial: SignalValue::Number(16.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearThickness".to_owned(),
            initial: SignalValue::Number(0.22),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearInnerRadius".to_owned(),
            initial: SignalValue::Number(0.28),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "lightIntensity".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "gearTopologyModel".to_owned(),
            reads: vec![
                RecipeReadSpec::LegacyId("gearTeeth".to_owned()),
                RecipeReadSpec::LegacyId("gearThickness".to_owned()),
                RecipeReadSpec::LegacyId("gearInnerRadius".to_owned()),
            ],
            expr: Expr::Object {
                fields: vec![
                    (
                        "teeth".to_owned(),
                        Expr::Read {
                            id: "gearTeeth".to_owned(),
                        },
                    ),
                    (
                        "thickness".to_owned(),
                        Expr::Read {
                            id: "gearThickness".to_owned(),
                        },
                    ),
                    (
                        "innerRadius".to_owned(),
                        Expr::Read {
                            id: "gearInnerRadius".to_owned(),
                        },
                    ),
                ],
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "hudModel".to_owned(),
            reads: vec![
                RecipeReadSpec::LegacyId("gearTopologyModel".to_owned()),
                RecipeReadSpec::LegacyId("lightIntensity".to_owned()),
            ],
            expr: Expr::Object {
                fields: vec![
                    (
                        "gear".to_owned(),
                        Expr::Read {
                            id: "gearTopologyModel".to_owned(),
                        },
                    ),
                    (
                        "light".to_owned(),
                        Expr::Read {
                            id: "lightIntensity".to_owned(),
                        },
                    ),
                ],
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        })
        .unwrap();

    let _ = runtime.read_value("hudModel").unwrap();
    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("what-if".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![
            TransactionOp::Set {
                id: "gearTeeth".to_owned(),
                value: SignalValue::Number(22.0),
                aspect: None,
                aspects: None,
            },
            TransactionOp::Set {
                id: "lightIntensity".to_owned(),
                value: SignalValue::Number(1.78),
                aspect: None,
                aspects: None,
            },
        ])
        .unwrap();

    runtime.switch_branch(main_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![
            TransactionOp::Set {
                id: "gearThickness".to_owned(),
                value: SignalValue::Number(0.42),
                aspect: None,
                aspects: None,
            },
            TransactionOp::Set {
                id: "gearInnerRadius".to_owned(),
                value: SignalValue::Number(0.36),
                aspect: None,
                aspects: None,
            },
        ])
        .unwrap();

    (
        runtime,
        main_branch.id.0,
        feature_branch.id.0,
        main_branch.name,
    )
}
