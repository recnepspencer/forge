use super::super::*;
use super::state::StatefulBridgeState;

pub(super) fn verify_existing_truth_assertion(
    state: &StatefulBridgeState,
    binding: &ForgeQueryExistingTruthTargetBinding,
    aspects: &[ForgeQueryAspectValue],
    snapshot_token: &str,
) -> Result<ForgeQueryVerifiedExistingTruthAssertion, ForgeQueryExistingTruthAssertionDenial> {
    let row = authoritative_row(state, binding).map_err(|message| {
        ForgeQueryExistingTruthAssertionDenial::new(
            binding,
            ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
            None,
            None,
            None,
            message,
        )
    })?;
    for aspect in aspects {
        if aspect.clears_existing_value() {
            return Err(ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                ForgeQueryExistingTruthAssertionDenialKind::ClearAssertionUnsupported,
                Some(aspect.aspect_path().to_string()),
                None,
                None,
                "backend-verified assertions cannot clear authoritative truth",
            ));
        }
        let Some(found) = get_json_path(row, aspect.aspect_path()) else {
            return Err(ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                Some(aspect.aspect_path().to_string()),
                Some(value_json(aspect.value())),
                None,
                "authoritative truth did not contain the asserted aspect",
            ));
        };
        if found != aspect.value() {
            return Err(ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                ForgeQueryExistingTruthAssertionDenialKind::AssertedValueMismatch,
                Some(aspect.aspect_path().to_string()),
                Some(value_json(aspect.value())),
                Some(value_json(found)),
                "authoritative truth did not match the asserted value",
            ));
        }
    }
    ForgeQueryVerifiedExistingTruthAssertion::new(binding, aspects, snapshot_token).map_err(
        |error| {
            ForgeQueryExistingTruthAssertionDenial::new(
                binding,
                ForgeQueryExistingTruthAssertionDenialKind::MissingAssertedAspect,
                None,
                None,
                None,
                error.to_string(),
            )
        },
    )
}

pub(super) fn probe_existing_truth(
    state: &StatefulBridgeState,
    request: &ForgeQueryExistingTruthProbeRequest,
) -> Result<ForgeQueryExistingTruthProbe, ForgeQueryExistingTruthProbeDenial> {
    let row = authoritative_row(state, request.binding()).map_err(|message| {
        ForgeQueryExistingTruthProbeDenial::new(
            request.binding(),
            ForgeQueryExistingTruthProbeDenialKind::ResolvedTargetUnavailable,
            None,
            message,
        )
    })?;
    let mut fields = Vec::with_capacity(request.aspect_paths().len());
    for aspect_path in request.aspect_paths() {
        let Some(value) = get_json_path(row, aspect_path) else {
            return Err(ForgeQueryExistingTruthProbeDenial::new(
                request.binding(),
                ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                Some(aspect_path.clone()),
                "authoritative truth did not contain the probed aspect",
            ));
        };
        fields.push((aspect_path.clone(), value.clone()));
    }
    Ok(ForgeQueryExistingTruthProbe::backend_verified(
        request, fields,
    ))
}

fn authoritative_row<'a>(
    state: &'a StatefulBridgeState,
    binding: &ForgeQueryExistingTruthTargetBinding,
) -> Result<&'a Value, String> {
    let Some(collection) = state
        .collection_by_identity
        .get(binding.resolved_target_identity())
    else {
        return Err(format!(
            "resolved target `{}` is not present in authoritative truth",
            binding.resolved_target_identity()
        ));
    };
    state
        .rows_by_collection
        .get(collection)
        .and_then(|rows| rows.get(binding.resolved_target_identity()))
        .ok_or_else(|| {
            format!(
                "resolved target `{}` is not present in authoritative truth",
                binding.resolved_target_identity()
            )
        })
}

fn get_json_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for part in path.split('.') {
        current = current.as_object()?.get(part)?;
    }
    Some(current)
}

fn value_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
}
