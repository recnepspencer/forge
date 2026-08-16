use std::collections::BTreeSet;

use quote::ToTokens;
use syn::{Attribute, ImplItem, Item, Visibility};

const ADMISSION: &str =
    include_str!("../../../../../data/proof/invalidation/frontier_admission.rs");
const PLAN: &str = include_str!("../../../../../data/proof/invalidation/plan.rs");
const EXECUTION: &str = include_str!("../../../../../data/proof/invalidation/execution.rs");

const EXPECTED_CONSTRUCTORS: &[&str] = &[
    "InvalidationPlanningEstimate::fn default () -> Self [derive(Default)]",
    "InvalidationSeed::fn new (source_node : NodeId , aspect : Aspect , changed_scopes : impl Into < PartitionScopeSet > , cause : FrontierSeedCause ,) -> Self",
    "InvalidationSeedBatch::fn default () -> Self [derive(Default)]",
    "InvalidationSeedBatch::fn new (seeds : impl IntoIterator < Item = InvalidationSeed >) -> Self",
    "InvalidationTraceRecord::fn new (node : NodeId , aspect : Aspect , wave_index : u32 , classification : FrontierEntryClassification , inclusion_basis : FrontierInclusionBasis ,) -> Self",
];

#[test]
fn phase_1_inventory_freezes_every_public_constructor_signature() {
    assert_eq!(
        public_constructor_signatures([ADMISSION, PLAN, EXECUTION]),
        EXPECTED_CONSTRUCTORS
            .iter()
            .map(|signature| (*signature).to_owned())
            .collect()
    );
}

#[test]
fn phase_1_inventory_rejects_constructor_signature_drift() {
    let drifted = EXECUTION.replacen("pub fn new(", "pub fn drifted(", 1);
    assert_ne!(
        public_constructor_signatures([ADMISSION, PLAN, &drifted]),
        expected_constructors()
    );
}

#[test]
fn phase_1_inventory_rejects_an_extra_public_constructor() {
    let expanded = format!(
        "{ADMISSION}\nimpl InvalidationSeed {{ pub fn from_parts() -> Self {{ todo!() }} }}"
    );
    assert_ne!(
        public_constructor_signatures([expanded.as_str(), PLAN, EXECUTION]),
        expected_constructors()
    );
}

#[test]
fn phase_1_inventory_rejects_concrete_and_wrapped_constructor_returns() {
    let concrete = format!(
        "{ADMISSION}\nimpl InvalidationSeed {{ pub fn concrete() -> InvalidationSeed {{ todo!() }} }}"
    );
    assert_ne!(
        public_constructor_signatures([concrete.as_str(), PLAN, EXECUTION]),
        expected_constructors()
    );

    let wrapped = format!(
        "{ADMISSION}\nimpl InvalidationSeed {{ pub fn fallible() -> Result<Self, ()> {{ todo!() }} }}"
    );
    assert_ne!(
        public_constructor_signatures([wrapped.as_str(), PLAN, EXECUTION]),
        expected_constructors()
    );
}

fn expected_constructors() -> BTreeSet<String> {
    EXPECTED_CONSTRUCTORS
        .iter()
        .map(|signature| (*signature).to_owned())
        .collect()
}

fn public_constructor_signatures<'a>(
    sources: impl IntoIterator<Item = &'a str>,
) -> BTreeSet<String> {
    let mut constructors = BTreeSet::new();
    for source in sources {
        let file = syn::parse_file(source).expect("constructor owner must parse");
        for item in file.items {
            match item {
                Item::Impl(owner) => {
                    let owner_name = owner.self_ty.to_token_stream().to_string();
                    for member in owner.items {
                        let ImplItem::Fn(method) = member else {
                            continue;
                        };
                        if matches!(method.vis, Visibility::Public(_))
                            && method.sig.receiver().is_none()
                        {
                            constructors
                                .insert(format!("{owner_name}::{}", method.sig.to_token_stream()));
                        }
                    }
                }
                Item::Struct(item)
                    if matches!(
                        item.ident.to_string().as_str(),
                        "InvalidationSeedBatch" | "InvalidationPlanningEstimate"
                    ) && derives_default(&item.attrs) =>
                {
                    constructors.insert(format!(
                        "{}::fn default () -> Self [derive(Default)]",
                        item.ident
                    ));
                }
                _ => {}
            }
        }
    }
    constructors
}

fn derives_default(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("derive"))
        .any(|attribute| attribute.to_token_stream().to_string().contains("Default"))
}
