use std::collections::{BTreeMap, BTreeSet};

use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_query::{
    ApplicationQueryDisclosurePosture, ApplicationQueryDisclosureRule,
    ApplicationQueryDisclosureSelector, ApplicationQueryInfluenceContract,
    ApplicationQueryObservableInfluence, ApplicationQueryResultSlotKey,
};
use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;

use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout;

#[derive(Clone, Debug)]
pub(in crate::domain_computation::primary_graph::application_query) struct WorthQueryInstalledApplicationDisclosureRule
{
    disclosure_value: AspectValue,
    influence: ApplicationQueryInfluenceContract,
}

impl WorthQueryInstalledApplicationDisclosureRule {
    pub(super) const fn disclosure_value(&self) -> &AspectValue {
        &self.disclosure_value
    }

    pub(super) const fn influence(&self) -> &ApplicationQueryInfluenceContract {
        &self.influence
    }
}

#[derive(Clone, Debug)]
pub(in crate::domain_computation::primary_graph::application_query) enum WorthQueryAdmittedApplicationDisclosureContract
{
    Public,
    Governed {
        classification: String,
        capability_name: String,
        capability_type: String,
        result_rules:
            BTreeMap<ApplicationQueryResultSlotKey, WorthQueryInstalledApplicationDisclosureRule>,
        internal_rules: Vec<WorthQueryInstalledApplicationDisclosureRule>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph::application_query) struct WorthQueryApplicationDisclosureContractDenial
{
    subject: String,
}

impl WorthQueryApplicationDisclosureContractDenial {
    pub(in crate::domain_computation::primary_graph::application_query) fn subject(&self) -> &str {
        &self.subject
    }
}

pub(in crate::domain_computation::primary_graph::application_query) fn compile_disclosure_contract<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Scope,
>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    layout: &WorthQueryPrimaryGraphLayout,
) -> Result<
    WorthQueryAdmittedApplicationDisclosureContract,
    WorthQueryApplicationDisclosureContractDenial,
> {
    let declared = query.disclosure();
    match declared.posture() {
        ApplicationQueryDisclosurePosture::Public => {
            if declared.rules().is_empty() {
                Ok(WorthQueryAdmittedApplicationDisclosureContract::Public)
            } else {
                Err(denial(query.name()))
            }
        }
        ApplicationQueryDisclosurePosture::InstalledPolicyRequired => Err(denial(query.name())),
        ApplicationQueryDisclosurePosture::Governed => compile_governed_contract(query, layout),
    }
}

fn compile_governed_contract<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    layout: &WorthQueryPrimaryGraphLayout,
) -> Result<
    WorthQueryAdmittedApplicationDisclosureContract,
    WorthQueryApplicationDisclosureContractDenial,
> {
    let mut result_rules = BTreeMap::new();
    let mut internal_rules = Vec::new();
    let mut governed_fields = BTreeMap::new();
    for rule in query.disclosure().rules() {
        admit_rule(
            query,
            layout,
            rule,
            &mut result_rules,
            &mut internal_rules,
            &mut governed_fields,
        )?;
    }
    require_complete_result_shape(query, &result_rules)?;
    validate_influence(query, &governed_fields, &result_rules)?;
    let capability_name = query
        .disclosure()
        .capability_name()
        .ok_or_else(|| denial(query.name()))?
        .to_string();
    let capability_type = query
        .disclosure()
        .capability_type()
        .ok_or_else(|| denial(query.name()))?
        .to_string();
    Ok(WorthQueryAdmittedApplicationDisclosureContract::Governed {
        classification: query.disclosure().classification().to_string(),
        capability_name,
        capability_type,
        result_rules,
        internal_rules,
    })
}

type GovernedFieldRules =
    BTreeMap<(String, String, String), Vec<(AspectValue, ApplicationQueryInfluenceContract)>>;

fn admit_rule<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    layout: &WorthQueryPrimaryGraphLayout,
    rule: &ApplicationQueryDisclosureRule,
    result_rules: &mut BTreeMap<
        ApplicationQueryResultSlotKey,
        WorthQueryInstalledApplicationDisclosureRule,
    >,
    internal_rules: &mut Vec<WorthQueryInstalledApplicationDisclosureRule>,
    governed_fields: &mut GovernedFieldRules,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    let installed = WorthQueryInstalledApplicationDisclosureRule {
        disclosure_value: rule.disclosure_value().clone(),
        influence: rule.influence().clone(),
    };
    let selector = rule.selector();
    if let Some((entity, aspect, field)) = selector.field_contract() {
        admit_field_masks(layout, selector, entity, aspect)?;
        governed_fields
            .entry((entity.to_string(), aspect.to_string(), field.to_string()))
            .or_default()
            .push((rule.disclosure_value().clone(), rule.influence().clone()));
    }
    match selector {
        ApplicationQueryDisclosureSelector::InternalField { .. } => {
            internal_rules.push(installed);
        }
        ApplicationQueryDisclosureSelector::Field { .. } => {
            let slot = selector
                .result_slot_key()
                .expect("result fields retain a typed slot identity");
            let exact = query
                .read_graph()
                .projections()
                .iter()
                .any(|projection| projection.slot_key_identity().as_ref() == &slot);
            if !exact || result_rules.insert(slot, installed).is_some() {
                return Err(denial(selector.slot_type()));
            }
        }
        ApplicationQueryDisclosureSelector::Relation { .. } => {
            let slot = selector
                .result_slot_key()
                .expect("result relations retain a typed slot identity");
            let exact = query
                .read_graph()
                .relations()
                .iter()
                .any(|relation| relation.slot_key_identity().as_ref() == &slot);
            if !exact || result_rules.insert(slot, installed).is_some() {
                return Err(denial(selector.slot_type()));
            }
        }
    }
    Ok(())
}

fn admit_field_masks(
    layout: &WorthQueryPrimaryGraphLayout,
    selector: &ApplicationQueryDisclosureSelector,
    entity: &str,
    aspect: &str,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    let aspect_key =
        worth_foundational::facade::AspectKey::new(aspect).ok_or_else(|| denial(aspect))?;
    let contract = layout
        .aspect_contract(entity, &aspect_key)
        .ok_or_else(|| denial(aspect))?;
    contract
        .admits_projection_mask(
            selector
                .projection_mask()
                .expect("governed fields retain projection masks"),
        )
        .map_err(|_| denial(aspect))?;
    contract
        .admits_diagnostic_mask(
            selector
                .diagnostic_mask()
                .expect("governed fields retain diagnostic masks"),
        )
        .map_err(|_| denial(aspect))
}

fn require_complete_result_shape<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    rules: &BTreeMap<ApplicationQueryResultSlotKey, WorthQueryInstalledApplicationDisclosureRule>,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    let expected = query.read_graph().projections().len() + query.read_graph().relations().len();
    let exact = query
        .read_graph()
        .projections()
        .iter()
        .map(|projection| projection.slot_key_identity())
        .chain(
            query
                .read_graph()
                .relations()
                .iter()
                .map(|relation| relation.slot_key_identity()),
        )
        .all(|slot| rules.contains_key(slot.as_ref()));
    if expected == rules.len() && exact {
        Ok(())
    } else {
        Err(denial(query.name()))
    }
}

fn validate_influence<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    fields: &GovernedFieldRules,
    results: &BTreeMap<ApplicationQueryResultSlotKey, WorthQueryInstalledApplicationDisclosureRule>,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    for predicate in query.read_graph().predicates() {
        require_field_influence(fields, predicate.field(), membership_surfaces(query))?;
    }
    for ordering in query.read_graph().ordering() {
        require_field_influence(
            fields,
            ordering.field(),
            [
                ApplicationQueryObservableInfluence::Ordering,
                ApplicationQueryObservableInfluence::Pagination,
            ],
        )?;
    }
    if let Some(continuation) = query.continuation() {
        let relation = query
            .read_graph()
            .relations()
            .iter()
            .find(|relation| relation.slot_type() == continuation.slot_type())
            .ok_or_else(|| denial(continuation.slot_type()))?;
        require_result_influence(
            results,
            relation.slot_key_identity().as_ref(),
            ApplicationQueryObservableInfluence::Pagination,
        )?;
    }
    if let Some(live) = query.live() {
        for projection in [live.scope_identity(), live.target_identity()] {
            require_result_influence(
                results,
                projection.slot_key_identity().as_ref(),
                ApplicationQueryObservableInfluence::LiveMembership,
            )?;
        }
    }
    Ok(())
}

fn membership_surfaces<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
) -> BTreeSet<ApplicationQueryObservableInfluence> {
    let mut surfaces = BTreeSet::from([
        ApplicationQueryObservableInfluence::RowPresence,
        ApplicationQueryObservableInfluence::Pagination,
        ApplicationQueryObservableInfluence::Count,
        ApplicationQueryObservableInfluence::Aggregate,
        ApplicationQueryObservableInfluence::Explanation,
    ]);
    if query.basis_support().pinned() {
        surfaces.insert(ApplicationQueryObservableInfluence::HistoricalMembership);
    }
    if query.basis_support().preview() {
        surfaces.insert(ApplicationQueryObservableInfluence::Preview);
    }
    if query.live().is_some() {
        surfaces.insert(ApplicationQueryObservableInfluence::LiveMembership);
    }
    surfaces
}

fn require_field_influence(
    fields: &GovernedFieldRules,
    field: (&str, &str, &str),
    surfaces: impl IntoIterator<Item = ApplicationQueryObservableInfluence>,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    let key = (
        field.0.to_string(),
        field.1.to_string(),
        field.2.to_string(),
    );
    let rules = fields.get(&key).ok_or_else(|| denial(field.2))?;
    let surfaces = surfaces.into_iter().collect::<Vec<_>>();
    if rules
        .iter()
        .all(|(_, influence)| surfaces.iter().all(|surface| influence.permits(*surface)))
    {
        Ok(())
    } else {
        Err(denial(field.2))
    }
}

fn require_result_influence(
    rules: &BTreeMap<ApplicationQueryResultSlotKey, WorthQueryInstalledApplicationDisclosureRule>,
    slot: &ApplicationQueryResultSlotKey,
    surface: ApplicationQueryObservableInfluence,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    if rules
        .get(slot)
        .is_some_and(|rule| rule.influence().permits(surface))
    {
        Ok(())
    } else {
        Err(denial("observable-influence"))
    }
}

fn denial(subject: impl Into<String>) -> WorthQueryApplicationDisclosureContractDenial {
    WorthQueryApplicationDisclosureContractDenial {
        subject: subject.into(),
    }
}
