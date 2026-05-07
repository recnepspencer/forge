#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryBridgeBackedVerificationSupportProfileRow {
    operation_family: String,
    target_binding_family: String,
    scaffold_profile_supported: bool,
    primary_bridge_backed_runtime_supported: bool,
    denial_class_when_primary_unsupported: Option<String>,
}

impl ForgeQueryBridgeBackedVerificationSupportProfileRow {
    pub(crate) fn new(
        operation_family: impl Into<String>,
        target_binding_family: impl Into<String>,
        scaffold_profile_supported: bool,
        primary_bridge_backed_runtime_supported: bool,
        denial_class_when_primary_unsupported: Option<&str>,
    ) -> Self {
        Self {
            operation_family: operation_family.into(),
            target_binding_family: target_binding_family.into(),
            scaffold_profile_supported,
            primary_bridge_backed_runtime_supported,
            denial_class_when_primary_unsupported: denial_class_when_primary_unsupported
                .map(str::to_string),
        }
    }

    pub(crate) fn operation_family(&self) -> &str {
        &self.operation_family
    }

    pub(crate) fn target_binding_family(&self) -> &str {
        &self.target_binding_family
    }

    pub(crate) fn scaffold_profile_supported(&self) -> bool {
        self.scaffold_profile_supported
    }

    pub(crate) fn primary_bridge_backed_runtime_supported(&self) -> bool {
        self.primary_bridge_backed_runtime_supported
    }

    pub(crate) fn denial_class_when_primary_unsupported(&self) -> Option<&str> {
        self.denial_class_when_primary_unsupported.as_deref()
    }
}

pub(crate) fn default_bridge_backed_verification_support_rows(
) -> Vec<ForgeQueryBridgeBackedVerificationSupportProfileRow> {
    let mut rows = vec![];
    for (operation_family, denial_class) in [
        ("verify_existing", Some("backend_verification_unsupported")),
        ("probe_existing", Some("backend_probe_unsupported")),
        (
            "update_existing_verified",
            Some("backend_verification_unsupported"),
        ),
        (
            "delete_existing_verified",
            Some("backend_verification_unsupported"),
        ),
    ] {
        for target_binding_family in ["direct_entity_identity", "direct_relation_identity"] {
            rows.push(ForgeQueryBridgeBackedVerificationSupportProfileRow::new(
                operation_family,
                target_binding_family,
                true,
                false,
                denial_class,
            ));
        }
    }
    rows
}
