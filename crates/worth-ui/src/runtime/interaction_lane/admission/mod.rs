mod target_resolution;

use crate::capability::SurfaceId;
use crate::runtime::WorthUiRuntimeHost;

use super::authored_props::{interaction_authored_props, AuthoredInteractionProp};
use super::denial_receipt::WorthUiInteractionValueDenialReceipt;
use super::digest::{
    interaction_admission_digest, interaction_denial_set_digest, interaction_schema_digest,
};
use super::report::{
    WorthUiInteractionAdmissionCounters, WorthUiInteractionAdmissionReceipt,
    WorthUiInteractionAdmissionReport, WorthUiInteractionValueDenialSet,
    WorthUiValidatedInteractionPropSet,
};
use super::schema::{
    interaction_prop_schema, interaction_prop_schemas, WorthUiInteractionPropSchema,
    INTERACTION_ID_PROP, INTERACTION_KIND_PROP, INTERACTION_PAYLOAD_PROP,
    INTERACTION_READINESS_PROP,
};
use super::value::{validate_interaction_value, WorthUiValidatedInteractionValue};
use target_resolution::resolve_target;

impl WorthUiRuntimeHost {
    pub fn resolve_interaction_admission_report(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiInteractionAdmissionReport {
        self.admit_interaction_props(surface_id)
    }

    pub(crate) fn admit_interaction_props(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiInteractionAdmissionReport {
        let authored_props = interaction_authored_props(self, surface_id);
        let schemas = interaction_prop_schemas();
        let mut defaults_applied = 0;
        let mut values_validated = 0;
        let mut denials = Vec::new();
        let authored_digest = authored_surface_digest(self, surface_id);

        let kind = admit_required(
            surface_id.as_str(),
            schema_for(INTERACTION_KIND_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        );
        let interaction_id = admit_required(
            surface_id.as_str(),
            schema_for(INTERACTION_ID_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        );
        let payload = admit_required(
            surface_id.as_str(),
            schema_for(INTERACTION_PAYLOAD_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        );
        let target = resolve_target(
            self,
            surface_id.as_str(),
            kind.as_ref(),
            &authored_props,
            &mut values_validated,
            &mut denials,
        );
        let readiness = admit_required(
            surface_id.as_str(),
            schema_for(INTERACTION_READINESS_PROP),
            &authored_props,
            &mut defaults_applied,
            &mut values_validated,
            &mut denials,
        );
        push_unknown_interaction_prop_denials(surface_id.as_str(), &authored_props, &mut denials);

        let counters = WorthUiInteractionAdmissionCounters::new(
            schemas.len(),
            authored_interaction_prop_count(&authored_props),
            defaults_applied,
            values_validated,
            denials.len(),
        );
        let schema_digest = interaction_schema_digest(schemas);
        if !denials.is_empty() {
            return rejected_interaction_report(
                surface_id.as_str(),
                denials,
                counters,
                schema_digest,
            );
        }
        let prop_set = WorthUiValidatedInteractionPropSet::new(
            kind.expect("denials empty means kind admitted").into_kind(),
            interaction_id
                .expect("denials empty means interaction id admitted")
                .into_identifier(),
            payload
                .expect("denials empty means payload admitted")
                .into_payload(),
            target.expect("denials empty means target admitted"),
            readiness
                .expect("denials empty means readiness admitted")
                .into_readiness(),
        );
        accepted_interaction_report(
            surface_id.as_str(),
            authored_digest,
            prop_set,
            counters,
            schema_digest,
        )
    }
}

pub(super) fn admit_required(
    surface_id: &str,
    schema: &'static WorthUiInteractionPropSchema,
    authored_props: &[AuthoredInteractionProp],
    defaults_applied: &mut usize,
    values_validated: &mut usize,
    denials: &mut Vec<WorthUiInteractionValueDenialReceipt>,
) -> Option<WorthUiValidatedInteractionValue> {
    let authored_prop = authored_props
        .iter()
        .find(|prop| prop.key == schema.prop_key());
    let raw_value = authored_prop.map(|prop| prop.value.clone()).or_else(|| {
        schema.default_value().map(|value| {
            *defaults_applied += 1;
            value.to_owned()
        })
    });
    let Some(raw_value) = raw_value else {
        denials.push(WorthUiInteractionValueDenialReceipt::missing_required(
            surface_id, schema,
        ));
        return None;
    };
    *values_validated += 1;
    match validate_interaction_value(
        surface_id,
        schema,
        raw_value,
        authored_prop.and_then(|prop| prop.source_span),
    ) {
        Ok(value) => Some(value),
        Err(denial) => {
            denials.push(denial);
            None
        }
    }
}

pub(super) fn schema_for(prop_key: &str) -> &'static WorthUiInteractionPropSchema {
    interaction_prop_schema(prop_key).expect("interaction prop schema exists")
}

fn authored_surface_digest(runtime: &WorthUiRuntimeHost, surface_id: &SurfaceId) -> u64 {
    runtime
        .active_authoring_snapshot()
        .and_then(|snapshot| {
            snapshot
                .authored_surface_props()
                .surface_digest(surface_id.as_str())
        })
        .unwrap_or(0xcbf2_9ce4_8422_2325)
}

fn authored_interaction_prop_count(authored_props: &[AuthoredInteractionProp]) -> usize {
    authored_props
        .iter()
        .filter(|prop| prop.key.starts_with("interaction_"))
        .count()
}

fn push_unknown_interaction_prop_denials(
    surface_id: &str,
    authored_props: &[AuthoredInteractionProp],
    denials: &mut Vec<WorthUiInteractionValueDenialReceipt>,
) {
    for prop in authored_props {
        if prop.key.starts_with("interaction_") && interaction_prop_schema(&prop.key).is_none() {
            denials.push(WorthUiInteractionValueDenialReceipt::unknown_prop(
                surface_id,
                &prop.key,
                prop.value.clone(),
                prop.source_span,
            ));
        }
    }
}

fn rejected_interaction_report(
    surface_id: &str,
    denials: Vec<WorthUiInteractionValueDenialReceipt>,
    counters: WorthUiInteractionAdmissionCounters,
    schema_digest: u64,
) -> WorthUiInteractionAdmissionReport {
    let denial_set_digest = interaction_denial_set_digest(surface_id, &denials);
    WorthUiInteractionAdmissionReport::rejected(
        surface_id,
        WorthUiInteractionValueDenialSet::new(surface_id, denials, denial_set_digest),
        counters,
        schema_digest,
    )
}

fn accepted_interaction_report(
    surface_id: &str,
    authored_digest: u64,
    prop_set: WorthUiValidatedInteractionPropSet,
    counters: WorthUiInteractionAdmissionCounters,
    schema_digest: u64,
) -> WorthUiInteractionAdmissionReport {
    let admission_digest = interaction_admission_digest(surface_id, authored_digest, &prop_set);
    WorthUiInteractionAdmissionReport::accepted(
        surface_id,
        WorthUiInteractionAdmissionReceipt::new(
            surface_id,
            prop_set,
            authored_digest,
            admission_digest,
        ),
        counters,
        schema_digest,
    )
}
