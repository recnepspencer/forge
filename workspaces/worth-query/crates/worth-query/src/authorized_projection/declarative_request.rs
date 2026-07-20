use std::collections::BTreeSet;

use crate::authoring::AspectFieldKey;
use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativeProjectionField};

use super::AuthorizedProjectionArtifact;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct AuthorizedDeclarativeProjection {
    fields: Vec<DeclarativeProjectionField>,
}

impl AuthorizedDeclarativeProjection {
    pub(super) fn new(fields: Vec<DeclarativeProjectionField>) -> Self {
        Self { fields }
    }

    pub(crate) fn into_fields(self) -> Vec<DeclarativeProjectionField> {
        self.fields
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AuthorizedDeclarativeProjectionError {
    DuplicateAuthorizedField(AspectFieldKey),
    MissingAuthorizedFields(Vec<AspectFieldKey>),
    UnauthorizedResultField(AspectFieldKey),
}

pub(crate) fn reconcile_authorized_declarative_projection(
    request: &DeclarativeLiveQueryRequest,
    authorized: &AuthorizedProjectionArtifact,
) -> Result<AuthorizedDeclarativeProjection, AuthorizedDeclarativeProjectionError> {
    let authorized_sources = authorized_source_fields(authorized);
    reject_unauthorized_result_fields(request, &authorized_sources)?;
    collect_authorized_request_fields(request, &authorized_sources)
}

fn authorized_source_fields(authorized: &AuthorizedProjectionArtifact) -> BTreeSet<AspectFieldKey> {
    authorized
        .visible_field_paths()
        .iter()
        .filter_map(|field| {
            Some(AspectFieldKey::from_native_keys(
                field.native_aspect_key(),
                field.native_field_key()?,
            ))
        })
        .collect()
}

fn reject_unauthorized_result_fields(
    request: &DeclarativeLiveQueryRequest,
    authorized_sources: &BTreeSet<AspectFieldKey>,
) -> Result<(), AuthorizedDeclarativeProjectionError> {
    if let Some(field) = request
        .result_fields()
        .iter()
        .find(|field| !authorized_sources.contains(field.source_field_key()))
    {
        return Err(
            AuthorizedDeclarativeProjectionError::UnauthorizedResultField(
                field.source_field_key().clone(),
            ),
        );
    }
    Ok(())
}

fn collect_authorized_request_fields(
    request: &DeclarativeLiveQueryRequest,
    authorized_sources: &BTreeSet<AspectFieldKey>,
) -> Result<AuthorizedDeclarativeProjection, AuthorizedDeclarativeProjectionError> {
    let mut unconsumed_sources = authorized_sources.clone();
    let mut fields = Vec::with_capacity(authorized_sources.len());
    for field in request.query_projection() {
        if !authorized_sources.contains(field.source_field_key()) {
            continue;
        }
        if !unconsumed_sources.remove(field.source_field_key()) {
            return Err(
                AuthorizedDeclarativeProjectionError::DuplicateAuthorizedField(
                    field.source_field_key().clone(),
                ),
            );
        }
        fields.push(field.clone());
    }
    if !unconsumed_sources.is_empty() {
        return Err(
            AuthorizedDeclarativeProjectionError::MissingAuthorizedFields(
                unconsumed_sources.into_iter().collect(),
            ),
        );
    }
    Ok(AuthorizedDeclarativeProjection::new(fields))
}
