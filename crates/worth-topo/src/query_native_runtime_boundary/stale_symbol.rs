#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol {
    ExternalRowProjection,
    ExternalProjectionConstructor,
    ExternalRowConsumption,
    RetainedScalarStringLookup,
    RetainedScalarStringConsumption,
    LegacyAspectValue,
    CallerBuiltWriteCommand,
    LiveViewNameRouting,
}

impl WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol {
    pub const fn patterns() -> &'static [(&'static str, Self)] {
        &[
            ("external_row(", Self::ExternalRowProjection),
            (
                "from_external_projection",
                Self::ExternalProjectionConstructor,
            ),
            ("into_external_row", Self::ExternalRowConsumption),
            ("field_value(", Self::RetainedScalarStringLookup),
            (
                "consume_scalar_fields",
                Self::RetainedScalarStringConsumption,
            ),
            ("ForgeQueryAspectValue", Self::LegacyAspectValue),
            ("ForgeQueryWriteCommand", Self::CallerBuiltWriteCommand),
            ("live_entities(", Self::LiveViewNameRouting),
            ("drain_live_patches(", Self::LiveViewNameRouting),
            ("affected_live_view_ids", Self::LiveViewNameRouting),
        ]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExternalRowProjection => "external-row-projection",
            Self::ExternalProjectionConstructor => "external-projection-constructor",
            Self::ExternalRowConsumption => "external-row-consumption",
            Self::RetainedScalarStringLookup => "retained-scalar-string-lookup",
            Self::RetainedScalarStringConsumption => "retained-scalar-string-consumption",
            Self::LegacyAspectValue => "legacy-aspect-value",
            Self::CallerBuiltWriteCommand => "caller-built-write-command",
            Self::LiveViewNameRouting => "live-view-name-routing",
        }
    }
}
