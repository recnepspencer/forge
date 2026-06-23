use crate::capability::SurfaceId;
use crate::runtime::WorthUiRuntimeHost;

use super::authored_props::{appearance_state_authored_props, AuthoredAppearanceStateProp};
use super::denial_receipt::WorthUiAppearanceStateValueDenialReceipt;
use super::digest::{
    appearance_state_admission_digest, appearance_state_denial_set_digest,
    appearance_state_receipt_digest, appearance_state_schema_digest,
};
use super::receipt::WorthUiAppearanceStateFieldSet;
use super::report::{
    WorthUiAppearanceStateAdmissionCounters, WorthUiAppearanceStateAdmissionReceipt,
    WorthUiAppearanceStateAdmissionReport, WorthUiAppearanceStateValueDenialSet,
    WorthUiValidatedAppearanceStatePropSet,
};
use super::schema::{
    appearance_state_prop_schema, appearance_state_prop_schemas, WorthUiAppearanceStatePropSchema,
    WorthUiAppearanceStateValueKind,
};
use super::token_resolution::{
    resolve_density_points, resolve_font_size_points, resolve_theme_color,
};
use super::value::{
    default_appearance_state_value, validate_appearance_state_value,
    WorthUiValidatedAppearanceStateValue,
};

impl WorthUiRuntimeHost {
    pub fn resolve_appearance_state_admission_report(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiAppearanceStateAdmissionReport {
        self.admit_appearance_state_props(surface_id)
    }

    pub(crate) fn admit_appearance_state_props(
        &self,
        surface_id: &SurfaceId,
    ) -> WorthUiAppearanceStateAdmissionReport {
        let authored_props = appearance_state_authored_props(self, surface_id);
        let schemas = appearance_state_prop_schemas();
        let mut fields = AppearanceStateFields::default();
        let mut defaults_applied = 0;
        let mut values_validated = 0;
        let mut denials = Vec::new();
        let authored_digest = self
            .active_authoring_snapshot()
            .and_then(|snapshot| {
                snapshot
                    .authored_surface_props()
                    .surface_digest(surface_id.as_str())
            })
            .unwrap_or(0xcbf2_9ce4_8422_2325);

        for schema in &schemas {
            admit_schema_prop(
                self,
                surface_id.as_str(),
                schema,
                &authored_props,
                &mut fields,
                &mut defaults_applied,
                &mut values_validated,
                &mut denials,
            );
        }
        push_unknown_appearance_prop_denials(surface_id.as_str(), &authored_props, &mut denials);
        let counters = WorthUiAppearanceStateAdmissionCounters::new(
            schemas.len(),
            authored_props.len(),
            defaults_applied,
            values_validated,
            denials.len(),
        );
        let schema_digest = appearance_state_schema_digest(&schemas);
        if !denials.is_empty() {
            let denial_set_digest =
                appearance_state_denial_set_digest(surface_id.as_str(), &denials);
            return WorthUiAppearanceStateAdmissionReport::rejected(
                surface_id.as_str(),
                WorthUiAppearanceStateValueDenialSet::new(
                    surface_id.as_str(),
                    denials,
                    denial_set_digest,
                ),
                counters,
                schema_digest,
            );
        }
        let prop_set = fields.into_prop_set();
        let admission_digest =
            appearance_state_admission_digest(surface_id.as_str(), authored_digest, &prop_set);
        WorthUiAppearanceStateAdmissionReport::accepted(
            surface_id.as_str(),
            WorthUiAppearanceStateAdmissionReceipt::new(
                surface_id.as_str(),
                prop_set,
                authored_digest,
                admission_digest,
            ),
            counters,
            schema_digest,
        )
    }
}

impl WorthUiAppearanceStateAdmissionReceipt {
    pub(crate) fn resolved_receipt(&self) -> super::WorthUiStatefulAppearanceRecipeReceipt {
        let receipt_digest =
            appearance_state_receipt_digest(self.admission_digest(), self.prop_set());
        self.prop_set().clone().into_recipe(receipt_digest)
    }
}

#[derive(Default)]
struct AppearanceStateFields {
    rest: WorthUiAppearanceStateFieldSet,
    hover: WorthUiAppearanceStateFieldSet,
    pressed: WorthUiAppearanceStateFieldSet,
    focus: WorthUiAppearanceStateFieldSet,
    disabled: WorthUiAppearanceStateFieldSet,
    selected: WorthUiAppearanceStateFieldSet,
}

impl AppearanceStateFields {
    fn state_mut(&mut self, state: &str) -> &mut WorthUiAppearanceStateFieldSet {
        match state {
            "rest" => &mut self.rest,
            "hover" => &mut self.hover,
            "pressed" => &mut self.pressed,
            "focus" => &mut self.focus,
            "disabled" => &mut self.disabled,
            "selected" => &mut self.selected,
            _ => unreachable!("appearance schema guarantees known state"),
        }
    }

    fn into_prop_set(self) -> WorthUiValidatedAppearanceStatePropSet {
        WorthUiValidatedAppearanceStatePropSet::new(
            self.rest,
            self.hover,
            self.pressed,
            self.focus,
            self.disabled,
            self.selected,
        )
    }
}

fn admit_schema_prop(
    runtime: &WorthUiRuntimeHost,
    surface_id: &str,
    schema: &WorthUiAppearanceStatePropSchema,
    authored_props: &[AuthoredAppearanceStateProp],
    fields: &mut AppearanceStateFields,
    defaults_applied: &mut usize,
    values_validated: &mut usize,
    denials: &mut Vec<WorthUiAppearanceStateValueDenialReceipt>,
) {
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
        return;
    };
    *values_validated += 1;
    let source_span = authored_prop.and_then(|prop| prop.source_span);
    match validate_appearance_state_value(surface_id, schema, raw_value) {
        Ok(value) => apply_value(
            runtime,
            surface_id,
            schema,
            value,
            fields,
            denials,
            source_span,
        ),
        Err(mut denial) => {
            denial.attach_source_span(source_span);
            denials.push(denial);
            if let Some(default_value) = default_appearance_state_value(schema) {
                apply_value(
                    runtime,
                    surface_id,
                    schema,
                    default_value,
                    fields,
                    denials,
                    None,
                );
            }
        }
    }
}

fn apply_value(
    runtime: &WorthUiRuntimeHost,
    surface_id: &str,
    schema: &WorthUiAppearanceStatePropSchema,
    value: WorthUiValidatedAppearanceStateValue,
    fields: &mut AppearanceStateFields,
    denials: &mut Vec<WorthUiAppearanceStateValueDenialReceipt>,
    source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
) {
    let state = fields.state_mut(schema.state());
    match schema.value_kind() {
        WorthUiAppearanceStateValueKind::Color => match value.into_color_or_token() {
            Ok(color) => state.set_color(schema.field(), color),
            Err(token) => match resolve_theme_color(runtime, &token) {
                Ok(color) => state.set_color(schema.field(), color),
                Err(reason) => {
                    denials.push(WorthUiAppearanceStateValueDenialReceipt::token_resolution(
                        surface_id,
                        schema,
                        token,
                        reason,
                        source_span,
                    ))
                }
            },
        },
        WorthUiAppearanceStateValueKind::MeasurementToken => {
            let token = value.into_measurement_token();
            match resolve_density_points(runtime, &token) {
                Ok(points) => state.set_points(schema.field(), points),
                Err(reason) => {
                    denials.push(WorthUiAppearanceStateValueDenialReceipt::token_resolution(
                        surface_id,
                        schema,
                        token,
                        reason,
                        source_span,
                    ));
                }
            }
        }
        WorthUiAppearanceStateValueKind::Opacity => {
            state.set_opacity(value.into_opacity());
        }
        WorthUiAppearanceStateValueKind::TypographyToken => {
            let token = value.into_typography_token();
            match resolve_font_size_points(runtime, &token) {
                Ok(points) => state.set_typography(token, points),
                Err(reason) => {
                    denials.push(WorthUiAppearanceStateValueDenialReceipt::token_resolution(
                        surface_id,
                        schema,
                        token,
                        reason,
                        source_span,
                    ))
                }
            }
        }
        WorthUiAppearanceStateValueKind::Unknown => {}
    }
}

fn push_unknown_appearance_prop_denials(
    surface_id: &str,
    authored_props: &[AuthoredAppearanceStateProp],
    denials: &mut Vec<WorthUiAppearanceStateValueDenialReceipt>,
) {
    for prop in authored_props {
        if appearance_state_prop_schema(&prop.key).is_none() {
            denials.push(WorthUiAppearanceStateValueDenialReceipt::unknown_prop(
                surface_id,
                &prop.key,
                prop.value.clone(),
                prop.source_span,
            ));
        }
    }
}
