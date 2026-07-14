pub(crate) use std::cell::RefCell;
pub(crate) use std::rc::Rc;
pub(crate) use std::sync::{Arc, Mutex};

pub(crate) use serde_json::Value as JsonValue;

pub(crate) use crate::boundary::errors::WorthSignalJsError;
pub(crate) use crate::boundary::signals_model;
pub(crate) use crate::boundary::signals_model::{ComputedSpec, OutputSpec};
pub(crate) use crate::boundary::types::{
    DisposableHandle, InputSignal, Signals, SignalsTransaction,
};
pub(crate) use crate::expression::model::{Expr, SignalValue};
pub(crate) use crate::recipe::model::RecipeReadSpec;
pub(crate) use crate::runtime::compute_callbacks::ComputeCallbackInvocationResult;
#[allow(unused_imports)]
pub(crate) use crate::runtime::core::{new_shared_core, WebSignalKind};
pub(crate) use crate::runtime::policy::RuntimePolicySpec;
pub(crate) use crate::runtime::web_callbacks::WebObservationNotice;

pub(crate) fn build_signals() -> Signals {
    Signals {
        core: new_shared_core(RuntimePolicySpec::default()).unwrap(),
    }
}

pub(crate) fn build_phase3_graph(signals: &Signals) {
    let _count = signals
        .input_for_test("count", SignalValue::Number(1.0))
        .unwrap();
    let _double = signals
        .computed_for_test(
            "double",
            ComputedSpec {
                reads: vec![RecipeReadSpec::LegacyId("count".to_owned())],
                expr: Expr::Multiply {
                    args: vec![
                        Expr::Read {
                            id: "count".to_owned(),
                        },
                        Expr::Value {
                            value: SignalValue::Number(2.0),
                        },
                    ],
                },
                when: None,
                identity: None,
                produces_aspects: None,
            },
        )
        .unwrap();
    let _panel = signals
        .output_for_test(
            "panel",
            OutputSpec {
                reads: vec![
                    RecipeReadSpec::LegacyId("count".to_owned()),
                    RecipeReadSpec::LegacyId("double".to_owned()),
                ],
                expr: Expr::Object {
                    fields: vec![
                        (
                            "count".to_owned(),
                            Expr::Read {
                                id: "count".to_owned(),
                            },
                        ),
                        (
                            "double".to_owned(),
                            Expr::Read {
                                id: "double".to_owned(),
                            },
                        ),
                    ],
                },
                when: None,
                identity: None,
                produces_aspects: None,
            },
        )
        .unwrap();
}

pub(crate) fn set_signal_value(signals: &Signals, id: &str, value: f64) {
    let builder = SignalsTransaction {
        core: signals.core.clone(),
        ops: Rc::new(RefCell::new(Vec::new())),
    };
    builder
        .ops
        .borrow_mut()
        .push(crate::recipe::model::TransactionOp::Set {
            id: id.to_owned(),
            value: SignalValue::Number(value),
            aspect: None,
            aspects: None,
        });
    signals.apply_transaction_for_test(&builder).unwrap();
}
