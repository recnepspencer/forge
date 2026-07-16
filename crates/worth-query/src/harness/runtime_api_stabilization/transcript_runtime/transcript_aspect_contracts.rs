use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType, StructAspectShape,
};

pub(super) fn transcript_aspect_contracts(paths: &[&str]) -> Vec<AspectContract> {
    let mut fields_by_aspect = BTreeMap::<&str, BTreeSet<&str>>::new();
    fields_by_aspect.entry("identity").or_default().insert("id");
    for path in paths {
        let (aspect, field) = path
            .split_once('.')
            .expect("transcript produced aspect paths must name an aspect and field");
        fields_by_aspect.entry(aspect).or_default().insert(field);
    }

    fields_by_aspect
        .into_iter()
        .map(|(aspect, fields)| transcript_aspect_contract(aspect, fields))
        .collect()
}

fn transcript_aspect_contract<'a>(
    aspect: &str,
    fields: impl IntoIterator<Item = &'a str>,
) -> AspectContract {
    let fields = fields.into_iter().map(|field| {
        let required = aspect == "identity" && field == "id";
        FieldDeclaration::new(
            FieldKey::new(field).expect("transcript field must admit"),
            ScalarAspectType::String,
            if required {
                FieldRequirement::Required
            } else {
                FieldRequirement::Optional
            },
            if required {
                AbsenceLaw::Required
            } else {
                AbsenceLaw::Optional
            },
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("transcript field law must be coherent")
    });

    AspectContract::struct_aspect(
        AspectKey::new(aspect).expect("transcript aspect must admit"),
        AspectIdentity(stable_aspect_identity(aspect)),
        AspectContractRevision(1),
        StructAspectShape::new(fields).expect("transcript fields must be unique"),
    )
}

fn stable_aspect_identity(aspect: &str) -> u64 {
    aspect
        .bytes()
        .fold(14_695_981_039_346_656_037, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(1_099_511_628_211)
        })
}
