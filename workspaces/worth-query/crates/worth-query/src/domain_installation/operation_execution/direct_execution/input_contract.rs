use std::collections::BTreeSet;

use worth_query_installation::facade::{
    WorthQueryOperationParameterContract, WorthQueryOperationParameterField,
    WorthQueryOperationValueFamily,
};

#[derive(Clone, Debug, PartialEq)]
pub enum WorthQueryOperationParameterValue<'a> {
    Bool(bool),
    I64(i64),
    U64(u64),
    Text(&'a str),
    EntityIdentity(&'a str),
    NativeAspect {
        key: &'a worth_foundational::facade::AspectKey,
        identity: worth_foundational::facade::AspectIdentity,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryOperationParameter<'a> {
    name: &'a str,
    value: WorthQueryOperationParameterValue<'a>,
}

impl<'a> WorthQueryOperationParameter<'a> {
    pub fn new(name: &'a str, value: WorthQueryOperationParameterValue<'a>) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn value(&self) -> &WorthQueryOperationParameterValue<'a> {
        &self.value
    }
}

pub trait WorthQueryOperationInput: 'static {
    fn parameters(&self) -> Vec<WorthQueryOperationParameter<'_>>;
}

impl WorthQueryOperationInput for () {
    fn parameters(&self) -> Vec<WorthQueryOperationParameter<'_>> {
        Vec::new()
    }
}

pub(super) fn input_satisfies_contract(
    input: &impl WorthQueryOperationInput,
    contract: &WorthQueryOperationParameterContract,
) -> bool {
    let parameters = input.parameters();
    let WorthQueryOperationParameterContract::Declared { fields } = contract else {
        return parameters.is_empty();
    };
    let mut names = BTreeSet::new();
    if parameters.iter().any(|parameter| {
        !names.insert(parameter.name)
            || fields
                .iter()
                .find(|field| field.name == parameter.name)
                .is_none_or(|field| !value_matches_field(&parameter.value, field))
    }) {
        return false;
    }
    fields
        .iter()
        .filter(|field| field.required)
        .all(|field| names.contains(field.name.as_str()))
}

fn value_matches_field(
    value: &WorthQueryOperationParameterValue<'_>,
    field: &WorthQueryOperationParameterField,
) -> bool {
    matches!(
        (value, &field.value_family),
        (
            WorthQueryOperationParameterValue::Bool(_),
            WorthQueryOperationValueFamily::Bool
        ) | (
            WorthQueryOperationParameterValue::I64(_),
            WorthQueryOperationValueFamily::I64
        ) | (
            WorthQueryOperationParameterValue::U64(_),
            WorthQueryOperationValueFamily::U64
        ) | (
            WorthQueryOperationParameterValue::Text(_),
            WorthQueryOperationValueFamily::Text
        ) | (
            WorthQueryOperationParameterValue::EntityIdentity(_),
            WorthQueryOperationValueFamily::EntityIdentity
        )
    ) || matches!(
        (value, &field.value_family),
        (
            WorthQueryOperationParameterValue::NativeAspect { key, identity },
            WorthQueryOperationValueFamily::NativeAspect {
                key: expected_key,
                identity: expected_identity,
            }
        ) if *key == expected_key && identity == expected_identity
    )
}
