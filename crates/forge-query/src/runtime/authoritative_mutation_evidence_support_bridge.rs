use crate::identity::hash_parts;

use super::{ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeSupportProfile};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryBridgeBackedVerificationSupportStatus {
    Admitted,
    Denied,
}

impl ForgeQueryBridgeBackedVerificationSupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBridgeBackedVerificationSupportRow {
    operation_family: String,
    target_binding_family: String,
    current_posture_status: ForgeQueryBridgeBackedVerificationSupportStatus,
    compatibility_runtime_supported: bool,
    primary_bridge_backed_runtime_supported: bool,
    denial_class_when_unsupported: Option<String>,
    row_digest: String,
}

impl ForgeQueryBridgeBackedVerificationSupportRow {
    pub(crate) fn new(
        operation_family: impl Into<String>,
        target_binding_family: impl Into<String>,
        current_posture_status: ForgeQueryBridgeBackedVerificationSupportStatus,
        compatibility_runtime_supported: bool,
        primary_bridge_backed_runtime_supported: bool,
        denial_class_when_unsupported: Option<&str>,
    ) -> Self {
        let operation_family = operation_family.into();
        let target_binding_family = target_binding_family.into();
        let denial_class_when_unsupported = denial_class_when_unsupported.map(str::to_string);
        let mut parts = vec![
            format!("operation:{operation_family}"),
            format!("binding:{target_binding_family}"),
            format!("status:{}", current_posture_status.as_str()),
            format!("compatibility:{compatibility_runtime_supported}"),
            format!("primary:{primary_bridge_backed_runtime_supported}"),
        ];
        if let Some(denial) = &denial_class_when_unsupported {
            parts.push(format!("denial:{denial}"));
        }
        let row_digest = hash_parts(&parts);
        Self {
            operation_family,
            target_binding_family,
            current_posture_status,
            compatibility_runtime_supported,
            primary_bridge_backed_runtime_supported,
            denial_class_when_unsupported,
            row_digest,
        }
    }

    pub fn operation_family(&self) -> &str {
        &self.operation_family
    }

    pub fn target_binding_family(&self) -> &str {
        &self.target_binding_family
    }

    pub fn current_posture_status(&self) -> ForgeQueryBridgeBackedVerificationSupportStatus {
        self.current_posture_status
    }

    pub fn compatibility_runtime_supported(&self) -> bool {
        self.compatibility_runtime_supported
    }

    pub fn primary_bridge_backed_runtime_supported(&self) -> bool {
        self.primary_bridge_backed_runtime_supported
    }

    pub fn denial_class_when_unsupported(&self) -> Option<&str> {
        self.denial_class_when_unsupported.as_deref()
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn bridge_backed_verification_support_rows(
    support_profile: &ForgeQueryRuntimeSupportProfile,
) -> Vec<ForgeQueryBridgeBackedVerificationSupportRow> {
    support_profile
        .bridge_backed_verification_support_rows()
        .iter()
        .map(|row| {
            let current_posture_status = match support_profile.posture() {
                ForgeQueryRuntimeBackendPosture::Compatibility
                    if row.compatibility_runtime_supported() =>
                {
                    ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
                }
                ForgeQueryRuntimeBackendPosture::Primary
                    if row.primary_bridge_backed_runtime_supported() =>
                {
                    ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
                }
                ForgeQueryRuntimeBackendPosture::Compatibility
                | ForgeQueryRuntimeBackendPosture::Primary => {
                    ForgeQueryBridgeBackedVerificationSupportStatus::Denied
                }
            };
            let denial_class = match current_posture_status {
                ForgeQueryBridgeBackedVerificationSupportStatus::Admitted => None,
                ForgeQueryBridgeBackedVerificationSupportStatus::Denied => {
                    row.denial_class_when_primary_unsupported()
                }
            };
            ForgeQueryBridgeBackedVerificationSupportRow::new(
                row.operation_family(),
                row.target_binding_family(),
                current_posture_status,
                row.compatibility_runtime_supported(),
                row.primary_bridge_backed_runtime_supported(),
                denial_class,
            )
        })
        .collect()
}
