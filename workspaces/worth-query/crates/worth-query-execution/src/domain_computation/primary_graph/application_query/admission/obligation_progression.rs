use worth_query_installation::facade::{
    WorthQueryInstalledApplicationQuery, WorthQueryInstalledApplicationQueryAuthorization,
    WorthQueryInstalledGraphAuthorizationRequirement,
    WorthQueryInstalledGraphObligationKind as Kind,
    WorthQueryInstalledGraphObligationOwner as Owner,
};

use super::super::{
    WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
};

pub(super) fn validate_installed_obligation_progression<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Scope,
>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
    let rows = query.obligations().rows();
    let graph_read_is_exact = rows.iter().any(|row| {
        row.kind() == Kind::GraphRead
            && row.owner_progression() == [Owner::Relational, Owner::QueryExecution]
    });
    let authorization_rows = rows
        .iter()
        .filter(|row| row.kind() == Kind::AuthorizationObservation)
        .collect::<Vec<_>>();
    let ability_is_exact = match query.authorization() {
        WorthQueryInstalledApplicationQueryAuthorization::Public => {
            authorization_rows.iter().all(|row| {
                !matches!(
                    row.authorization_requirement(),
                    Some(WorthQueryInstalledGraphAuthorizationRequirement::Abilities(
                        _
                    ))
                )
            })
        }
        WorthQueryInstalledApplicationQueryAuthorization::Ability(expected) => {
            authorization_rows.iter().any(|row| {
                matches!(
                    row.authorization_requirement(),
                    Some(WorthQueryInstalledGraphAuthorizationRequirement::Abilities(actual))
                        if actual == std::slice::from_ref(expected)
                )
            })
        }
    };
    let disclosure_capability_is_exact = match (
        query.disclosure().capability_name(),
        query.disclosure().capability_type(),
    ) {
        (None, None) => authorization_rows.iter().all(|row| {
            !matches!(
                row.authorization_requirement(),
                Some(WorthQueryInstalledGraphAuthorizationRequirement::Capabilities(_))
            )
        }),
        (Some(name), Some(capability_type)) => authorization_rows.iter().any(|row| {
            matches!(
                row.authorization_requirement(),
                Some(WorthQueryInstalledGraphAuthorizationRequirement::Capabilities(requirements))
                    if !requirements.is_empty() && requirements.iter().all(|requirement| {
                        requirement.contract().name() == name
                            && requirement.contract().capability_type() == capability_type
                    })
            )
        }),
        _ => false,
    };
    let expected_authorization_rows =
        usize::from(matches!(
            query.authorization(),
            WorthQueryInstalledApplicationQueryAuthorization::Ability(_)
        )) + usize::from(query.disclosure().capability_name().is_some());
    let authorization_is_exact = authorization_rows.len() == expected_authorization_rows
        && ability_is_exact
        && disclosure_capability_is_exact
        && authorization_rows.iter().all(|row| {
            row.owner_progression()
                == [
                    Owner::Relational,
                    Owner::RuntimeBridge,
                    Owner::Signal,
                    Owner::QueryAdmission,
                ]
        });
    if graph_read_is_exact && authorization_is_exact {
        Ok(())
    } else {
        Err(WorthQueryApplicationQueryAdmissionDenial::new(
            WorthQueryApplicationQueryAdmissionDenialKind::InvalidInstalledObligationProgression,
            query.name(),
        ))
    }
}
