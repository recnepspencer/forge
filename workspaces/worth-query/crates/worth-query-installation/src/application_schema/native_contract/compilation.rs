use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::{
    aspects, AspectBinding, AspectContract, AspectContractRevision, AspectIdentity, AspectKey,
    FieldKey, ScalarAspectType,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationFieldPresence, ApplicationSchemaBindingIdentity, ApplicationSchemaMember,
    ErasedApplicationSchemaDeclaration,
};

use crate::canonical_work::WorthQueryCanonicalWorkEvidence;

use super::canonical_basis::CatalogCanonicalBudget;
use super::denial::{
    WorthQueryApplicationSchemaContractCatalogDenial,
    WorthQueryApplicationSchemaContractCatalogDenialKind as DenialKind,
};
use super::{
    WorthQueryInstalledApplicationAspectContract, WorthQueryInstalledApplicationAspectLocus,
    WorthQueryInstalledApplicationSchemaContractCatalog,
    WorthQueryInstalledApplicationSchemaContractCatalogCounters,
};

struct DeclaredAspectSeed {
    identity: AspectIdentity,
    revision: AspectContractRevision,
    fields: BTreeMap<FieldKey, (ScalarAspectType, ApplicationFieldPresence)>,
}

pub(crate) fn compile_native_contract_catalog(
    binding: &ApplicationSchemaBindingIdentity,
    schema: &ErasedApplicationSchemaDeclaration,
    schema_work: WorthQueryCanonicalWorkEvidence,
) -> Result<
    WorthQueryInstalledApplicationSchemaContractCatalog,
    WorthQueryApplicationSchemaContractCatalogDenial,
> {
    let mut aspects = collect_declared_aspects(schema)?;
    collect_declared_fields(schema, &mut aspects)?;
    let maximum_aspect_identity = aspects.values().map(|aspect| aspect.identity).max();
    let field_count = aspects.values().map(|aspect| aspect.fields.len()).sum();
    let mut canonical_budget = CatalogCanonicalBudget::from_schema_work(schema_work)?;
    let mut contracts =
        BTreeMap::<String, BTreeMap<String, WorthQueryInstalledApplicationAspectContract>>::new();
    for ((entity, aspect), seed) in aspects {
        let installed = compile_aspect_contract(
            binding,
            entity.clone(),
            aspect.clone(),
            seed,
            &mut canonical_budget,
        )?;
        contracts
            .entry(entity)
            .or_default()
            .insert(aspect, installed);
    }
    let counters = WorthQueryInstalledApplicationSchemaContractCatalogCounters::compiled(
        contracts.values().map(BTreeMap::len).sum(),
        field_count,
    );
    Ok(WorthQueryInstalledApplicationSchemaContractCatalog::new(
        contracts,
        maximum_aspect_identity,
        counters,
        canonical_budget.work(),
    ))
}

fn collect_declared_aspects(
    schema: &ErasedApplicationSchemaDeclaration,
) -> Result<
    BTreeMap<(String, String), DeclaredAspectSeed>,
    WorthQueryApplicationSchemaContractCatalogDenial,
> {
    let mut identities = BTreeMap::<AspectIdentity, (String, String)>::new();
    let mut aspects = BTreeMap::new();
    for member in schema.members() {
        let ApplicationSchemaMember::Aspect {
            entity,
            aspect,
            identity,
            revision,
        } = member
        else {
            continue;
        };
        validate_aspect_identity(&mut identities, entity, aspect, *identity, *revision)?;
        let locus = (entity.clone(), aspect.clone());
        if aspects.contains_key(&locus) {
            return Err(denial(
                DenialKind::DuplicateAspectLocus,
                format!("{entity}:{aspect}"),
            ));
        }
        aspects.insert(
            locus,
            DeclaredAspectSeed {
                identity: *identity,
                revision: *revision,
                fields: BTreeMap::new(),
            },
        );
    }
    Ok(aspects)
}

fn validate_aspect_identity(
    identities: &mut BTreeMap<AspectIdentity, (String, String)>,
    entity: &str,
    aspect: &str,
    identity: AspectIdentity,
    revision: AspectContractRevision,
) -> Result<(), WorthQueryApplicationSchemaContractCatalogDenial> {
    if revision.0 == 0 {
        return Err(denial(DenialKind::RevisionZero, aspect));
    }
    if let Some(existing) = identities.insert(identity, (entity.to_string(), aspect.to_string())) {
        return Err(denial(
            DenialKind::DuplicateAspectIdentity,
            format!(
                "{}:{} conflicts with {entity}:{aspect}",
                existing.0, existing.1
            ),
        ));
    }
    Ok(())
}

fn collect_declared_fields(
    schema: &ErasedApplicationSchemaDeclaration,
    aspects: &mut BTreeMap<(String, String), DeclaredAspectSeed>,
) -> Result<(), WorthQueryApplicationSchemaContractCatalogDenial> {
    for member in schema.members() {
        let ApplicationSchemaMember::Field {
            entity,
            aspect,
            field,
            presence,
            scalar_family,
            ..
        } = member
        else {
            continue;
        };
        let seed = aspects
            .get_mut(&(entity.clone(), aspect.clone()))
            .ok_or_else(|| denial(DenialKind::FieldWithoutAspect, field))?;
        let field_key = FieldKey::new(field.clone())
            .ok_or_else(|| denial(DenialKind::InvalidFieldKey, field))?;
        if seed
            .fields
            .insert(field_key, (*scalar_family, *presence))
            .is_some()
        {
            return Err(denial(
                DenialKind::DuplicateFieldLocus,
                format!("{entity}:{aspect}:{field}"),
            ));
        }
    }
    Ok(())
}

fn compile_aspect_contract(
    binding: &ApplicationSchemaBindingIdentity,
    entity: String,
    aspect_name: String,
    seed: DeclaredAspectSeed,
    canonical_budget: &mut CatalogCanonicalBudget,
) -> Result<
    WorthQueryInstalledApplicationAspectContract,
    WorthQueryApplicationSchemaContractCatalogDenial,
> {
    let aspect_key = AspectKey::new(aspect_name.clone())
        .ok_or_else(|| denial(DenialKind::InvalidAspectKey, &aspect_name))?;
    let contract = build_foundational_contract(&aspect_key, &aspect_name, &seed)?;
    admit_declared_projection_mask(&contract, &aspect_name, seed.fields.keys())?;
    let prepared = canonical_budget.prepare(&aspect_name, &contract)?;
    let fields = seed.fields.into_keys().collect::<BTreeSet<_>>();
    let locus = WorthQueryInstalledApplicationAspectLocus::new(binding.clone(), entity, aspect_key);
    let aspect_field = FieldKey::new(aspect_name.clone())
        .ok_or_else(|| denial(DenialKind::InvalidFieldKey, &aspect_name))?;
    Ok(WorthQueryInstalledApplicationAspectContract::new(
        locus,
        contract,
        fields,
        AspectBinding::EntityField {
            field: aspect_field,
        },
        prepared.basis,
        prepared.material,
    ))
}

fn build_foundational_contract(
    aspect_key: &AspectKey,
    aspect_name: &str,
    seed: &DeclaredAspectSeed,
) -> Result<AspectContract, WorthQueryApplicationSchemaContractCatalogDenial> {
    let mut fields = seed.fields.iter();
    let (first_key, (first_family, first_presence)) = fields
        .next()
        .ok_or_else(|| denial(DenialKind::MissingAspectFieldClosure, aspect_name))?;
    let mut shape = match first_presence {
        ApplicationFieldPresence::Required => aspects()
            .struct_fields()
            .required(first_key.as_str(), *first_family),
        ApplicationFieldPresence::Optional => aspects()
            .struct_fields()
            .optional(first_key.as_str(), *first_family),
    };
    for (field, (family, presence)) in fields {
        shape = match presence {
            ApplicationFieldPresence::Required => shape.required(field.as_str(), *family),
            ApplicationFieldPresence::Optional => shape.optional(field.as_str(), *family),
        };
    }
    let shape = shape
        .finish()
        .map_err(|_| denial(DenialKind::InvalidAspectShape, aspect_name))?;
    Ok(aspects()
        .contract()
        .for_key(aspect_key.clone())
        .identified_by(seed.identity)
        .at_revision(seed.revision)
        .struct_aspect(shape))
}

fn admit_declared_projection_mask<'a>(
    contract: &AspectContract,
    aspect: &str,
    fields: impl Iterator<Item = &'a FieldKey>,
) -> Result<(), WorthQueryApplicationSchemaContractCatalogDenial> {
    let mask = aspects()
        .projection_mask()
        .fields(fields.map(FieldKey::as_str))
        .map_err(|_| denial(DenialKind::InvalidFieldKey, aspect))?;
    contract
        .admits_projection_mask(&mask)
        .map_err(|_| denial(DenialKind::ProjectionMaskRejected, aspect))
}

fn denial(
    kind: DenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationSchemaContractCatalogDenial {
    WorthQueryApplicationSchemaContractCatalogDenial::new(kind, subject)
}
