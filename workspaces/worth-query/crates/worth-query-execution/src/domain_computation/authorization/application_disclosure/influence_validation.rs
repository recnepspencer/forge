use std::collections::{BTreeMap, BTreeSet};

use worth_query_declaration::facade::application_query::{
    ApplicationQueryObservableInfluence, ApplicationQueryResultSlotKey,
};
use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;

use super::contract::{
    denial, AdmittedDisclosureRule, WorthQueryApplicationDisclosureContractDenial,
};

pub(super) type GovernedFieldRules =
    BTreeMap<(String, String, String), Vec<AdmittedDisclosureRule>>;

pub(super) fn validate_influence<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    fields: &GovernedFieldRules,
    results: &BTreeMap<ApplicationQueryResultSlotKey, AdmittedDisclosureRule>,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    for predicate in query.read_graph().predicates() {
        require_field_influence(fields, predicate.field(), membership_surfaces(query))?;
    }
    for path in query.read_graph().root_paths() {
        for guard in path.guards() {
            require_field_influence(
                fields,
                (
                    guard.entity(),
                    guard.aspect().as_str(),
                    guard.field().as_str(),
                ),
                membership_surfaces(query),
            )?;
        }
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
    validate_continuation(query, results)?;
    validate_live(query, fields, results)
}

fn validate_continuation<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    results: &BTreeMap<ApplicationQueryResultSlotKey, AdmittedDisclosureRule>,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    let Some(continuation) = query.continuation() else {
        return Ok(());
    };
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
    )
}

fn validate_live<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
    fields: &GovernedFieldRules,
    results: &BTreeMap<ApplicationQueryResultSlotKey, AdmittedDisclosureRule>,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    if query.live().is_none() {
        return Ok(());
    }
    let surface = ApplicationQueryObservableInfluence::LiveMembership;
    let every_field = fields
        .values()
        .flatten()
        .all(|rule| rule.influence().permits(surface));
    let every_result = results
        .values()
        .all(|rule| rule.influence().permits(surface));
    (every_field && every_result)
        .then_some(())
        .ok_or_else(|| denial("live-observable-influence"))
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
    rules
        .iter()
        .all(|rule| {
            surfaces
                .iter()
                .all(|surface| rule.influence().permits(*surface))
        })
        .then_some(())
        .ok_or_else(|| denial(field.2))
}

fn require_result_influence(
    rules: &BTreeMap<ApplicationQueryResultSlotKey, AdmittedDisclosureRule>,
    slot: &ApplicationQueryResultSlotKey,
    surface: ApplicationQueryObservableInfluence,
) -> Result<(), WorthQueryApplicationDisclosureContractDenial> {
    rules
        .get(slot)
        .is_some_and(|rule| rule.influence().permits(surface))
        .then_some(())
        .ok_or_else(|| denial("observable-influence"))
}
