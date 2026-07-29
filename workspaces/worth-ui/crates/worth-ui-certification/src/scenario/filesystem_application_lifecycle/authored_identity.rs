use super::FilesystemApplicationLifecycleScenario;
use crate::scenario::application_authority_closure::application_definition::{
    CANDIDATE_COMPONENT, CURRENT_COMPONENT, IMPORTED_CANDIDATE_COMPONENT,
    IMPORTED_CURRENT_COMPONENT,
};

impl FilesystemApplicationLifecycleScenario {
    pub fn current_component_declaration_identity() -> String {
        format!("component:{CURRENT_COMPONENT}")
    }

    pub fn candidate_component_declaration_identity() -> String {
        format!("component:{CANDIDATE_COMPONENT}")
    }

    pub fn imported_current_component_declaration_identity() -> String {
        format!("component:{IMPORTED_CURRENT_COMPONENT}")
    }

    pub fn imported_candidate_source_text() -> String {
        format!("component {IMPORTED_CANDIDATE_COMPONENT} {{}}")
    }
}
