#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentationDefaultPathAudit {
    suite: &'static str,
    readme_teaches_pleasant_first: bool,
    readme_teaches_raw_escape_hatch: bool,
    readme_includes_scoped_default_lane: bool,
    happy_path_workflow_includes_raw_equivalent: bool,
    happy_path_workflow_uses_raw_import: bool,
    low_level_workflow_names_raw_escape_hatch: bool,
    low_level_workflow_uses_raw_import: bool,
}

impl DocumentationDefaultPathAudit {
    pub fn suite(&self) -> &'static str {
        self.suite
    }

    pub fn readme_teaches_pleasant_first(&self) -> bool {
        self.readme_teaches_pleasant_first
    }

    pub fn readme_teaches_raw_escape_hatch(&self) -> bool {
        self.readme_teaches_raw_escape_hatch
    }

    pub fn readme_includes_scoped_default_lane(&self) -> bool {
        self.readme_includes_scoped_default_lane
    }

    pub fn happy_path_workflow_includes_raw_equivalent(&self) -> bool {
        self.happy_path_workflow_includes_raw_equivalent
    }

    pub fn happy_path_workflow_uses_raw_import(&self) -> bool {
        self.happy_path_workflow_uses_raw_import
    }

    pub fn low_level_workflow_names_raw_escape_hatch(&self) -> bool {
        self.low_level_workflow_names_raw_escape_hatch
    }

    pub fn low_level_workflow_uses_raw_import(&self) -> bool {
        self.low_level_workflow_uses_raw_import
    }
}

const README: &str = include_str!("../../../README.md");
const HAPPY_PATH_WORKFLOW: &str =
    include_str!("../../../docs/workflows/happy-path-recipe-progression.md");
const LOW_LEVEL_WORKFLOW: &str = include_str!("../../../docs/workflows/when-to-stay-low-level.md");

pub fn documentation_default_path_audit() -> DocumentationDefaultPathAudit {
    DocumentationDefaultPathAudit {
        suite: "pleasant_lane_documentation_default_path_audit",
        readme_teaches_pleasant_first: README.contains("Most consumers should start with:")
            && README.contains("use forge_proof::prelude::*;"),
        readme_teaches_raw_escape_hatch: README.contains("## Raw Escape Hatch")
            && README.contains("use forge_proof::raw::*;"),
        readme_includes_scoped_default_lane: README.contains("Common scoped-default lane:")
            && README.contains("proof_flow()"),
        happy_path_workflow_includes_raw_equivalent: HAPPY_PATH_WORKFLOW
            .contains("## Pleasant Lane First")
            && HAPPY_PATH_WORKFLOW.contains("## Equivalent Raw Surface"),
        happy_path_workflow_uses_raw_import: HAPPY_PATH_WORKFLOW
            .contains("use forge_proof::raw::*;"),
        low_level_workflow_names_raw_escape_hatch: LOW_LEVEL_WORKFLOW
            .contains("## Raw Escape Hatch")
            && LOW_LEVEL_WORKFLOW.contains("use forge_proof::raw::*;"),
        low_level_workflow_uses_raw_import: LOW_LEVEL_WORKFLOW.contains("use forge_proof::raw::*;"),
    }
}
