use worth_ui_dsl::{
    WorthUiAuthoredMode, WorthUiDslProtocolIdentity, WorthUiSealedSemanticPackage,
    WorthUiSemanticPackageIdentity,
};

/// Read-only evidence identifying the exact DSL package presented at the
/// authored-to-runtime ownership transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticHandoffEvidence {
    identity: WorthUiSemanticPackageIdentity,
    protocol: WorthUiDslProtocolIdentity,
    authored_mode: WorthUiAuthoredMode,
    projection_contents: Box<[WorthUiProjectionContentEdge]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionContentEdge {
    component_identity: Box<str>,
    projection_identity: worth_ui_query_binding::WorthUiQueryViewIdentity,
}

impl WorthUiSemanticHandoffEvidence {
    pub(super) fn from_package(package: &WorthUiSealedSemanticPackage) -> Self {
        Self {
            identity: package.identity().clone(),
            protocol: package.protocol(),
            authored_mode: package.authored_mode(),
            projection_contents: projection_contents(package),
        }
    }

    pub fn identity(&self) -> &WorthUiSemanticPackageIdentity {
        &self.identity
    }

    pub fn protocol(&self) -> WorthUiDslProtocolIdentity {
        self.protocol
    }

    pub fn authored_mode(&self) -> WorthUiAuthoredMode {
        self.authored_mode
    }

    pub fn projection_contents(&self) -> &[WorthUiProjectionContentEdge] {
        &self.projection_contents
    }
}

impl WorthUiProjectionContentEdge {
    pub fn component_identity(&self) -> &str {
        &self.component_identity
    }

    pub fn projection_identity(&self) -> &worth_ui_query_binding::WorthUiQueryViewIdentity {
        &self.projection_identity
    }
}

fn projection_contents(
    package: &WorthUiSealedSemanticPackage,
) -> Box<[WorthUiProjectionContentEdge]> {
    let views = package
        .projection_requirements()
        .map(|requirement| {
            (
                requirement.declaration_identity(),
                requirement.view_identity(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    package
        .module_ids()
        .iter()
        .flat_map(|module_id| {
            package
                .module(module_id)
                .expect("sealed package contains every canonical module")
                .declarations()
                .iter()
                .filter_map(|declaration| match declaration {
                    worth_ui_dsl::WorthUiSemanticDeclaration::Component(component) => {
                        Some(component)
                    }
                    _ => None,
                })
                .flat_map(|component| {
                    component
                        .structure()
                        .projection_contents()
                        .iter()
                        .map(|content| {
                            let view = views[content.projection_identity_text()];
                            WorthUiProjectionContentEdge {
                                component_identity: format!("component:{}", component.name_text())
                                    .into(),
                                projection_identity:
                                    worth_ui_query_binding::WorthUiQueryViewIdentity::new(view)
                                        .expect("sealed DSL projection view identity is valid"),
                            }
                        })
                })
        })
        .collect()
}
