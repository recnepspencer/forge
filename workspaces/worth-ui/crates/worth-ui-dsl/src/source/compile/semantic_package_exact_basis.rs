mod fingerprint;

use crate::source::{
    WorthUiAuthoredMount, WorthUiAuthoredRegion, WorthUiAuthoredStructuralBody,
    WorthUiSemanticDeclaration, WorthUiSemanticModule, WorthUiSourceModuleId,
};
use fingerprint::Fingerprint;

#[derive(Debug, Eq, PartialEq)]
pub(super) struct WorthUiSemanticPackageExactBasis {
    modules: Box<[WorthUiSemanticModuleExactBasis]>,
}

#[derive(Debug, Eq, PartialEq)]
struct WorthUiSemanticModuleExactBasis {
    module_id: String,
    declarations: Box<[WorthUiSemanticDeclarationExactBasis]>,
}

#[derive(Debug, Eq, PartialEq)]
enum WorthUiSemanticDeclarationExactBasis {
    Import {
        target: String,
    },
    Component(WorthUiSemanticBlockExactBasis),
    Surface(WorthUiSemanticBlockExactBasis),
    Binding(WorthUiSemanticBlockExactBasis),
    Projection {
        declaration: String,
        view: String,
        shape: String,
        selected_fields: Box<[String]>,
        row_identity: Option<String>,
        native_family: String,
        lifecycle: String,
        requires_complete_result: Option<bool>,
        permits_continuation: Option<bool>,
    },
    Token {
        name: String,
        authored_identity: Option<String>,
        value: String,
    },
    SemanticArtifact {
        key: String,
        family: String,
        published_aspects: Box<[String]>,
        consumed_aspects: Box<[String]>,
        structural_tokens: Box<[String]>,
        posture_tokens: Box<[String]>,
        support_tokens: Box<[String]>,
    },
}

#[derive(Debug, Eq, PartialEq)]
struct WorthUiSemanticBlockExactBasis {
    name: String,
    authored_identity: Option<String>,
    structure: WorthUiStructuralBodyExactBasis,
}

#[derive(Debug, Eq, PartialEq)]
struct WorthUiStructuralBodyExactBasis {
    root_regions: Box<[WorthUiRegionExactBasis]>,
}

#[derive(Debug, Eq, PartialEq)]
struct WorthUiRegionExactBasis {
    id: String,
    sizing_contract: Option<String>,
    state_slot: Option<String>,
    child_regions: Box<[WorthUiRegionExactBasis]>,
    mounts: Box<[WorthUiMountExactBasis]>,
}

#[derive(Debug, Eq, PartialEq)]
struct WorthUiMountExactBasis {
    surface: String,
    placement_policy: Option<String>,
    state_slot: Option<String>,
}

impl WorthUiSemanticPackageExactBasis {
    pub(super) fn from_modules<'module>(
        modules: impl IntoIterator<
            Item = (
                &'module WorthUiSourceModuleId,
                &'module WorthUiSemanticModule,
            ),
        >,
    ) -> Self {
        Self {
            modules: modules
                .into_iter()
                .map(|(module_id, module)| WorthUiSemanticModuleExactBasis {
                    module_id: module_id.as_str().to_owned(),
                    declarations: module
                        .declarations()
                        .iter()
                        .map(WorthUiSemanticDeclarationExactBasis::from_declaration)
                        .collect(),
                })
                .collect(),
        }
    }

    pub(super) fn narrowing_fingerprint(&self) -> u64 {
        let mut fingerprint = Fingerprint::new("worth-ui:semantic-package:v1");
        fingerprint.fold_usize(self.modules.len());
        for module in &self.modules {
            module.fold_into(&mut fingerprint);
        }
        fingerprint.finish()
    }
}

impl WorthUiSemanticModuleExactBasis {
    fn fold_into(&self, fingerprint: &mut Fingerprint) {
        fingerprint.fold_text("module");
        fingerprint.fold_text(&self.module_id);
        fingerprint.fold_usize(self.declarations.len());
        for declaration in &self.declarations {
            declaration.fold_into(fingerprint);
        }
    }
}

impl WorthUiSemanticDeclarationExactBasis {
    fn from_declaration(declaration: &WorthUiSemanticDeclaration) -> Self {
        match declaration {
            WorthUiSemanticDeclaration::Import(import) => Self::Import {
                target: import.target().authored_text().to_owned(),
            },
            WorthUiSemanticDeclaration::Component(block) => {
                Self::Component(WorthUiSemanticBlockExactBasis::from_block(block))
            }
            WorthUiSemanticDeclaration::Surface(block) => {
                Self::Surface(WorthUiSemanticBlockExactBasis::from_block(block))
            }
            WorthUiSemanticDeclaration::Binding(block) => {
                Self::Binding(WorthUiSemanticBlockExactBasis::from_block(block))
            }
            WorthUiSemanticDeclaration::Projection(projection) => {
                let requirement = projection.requirement();
                Self::Projection {
                    declaration: requirement.declaration_identity().to_owned(),
                    view: requirement.view_identity().to_owned(),
                    shape: requirement.shape().canonical_token().to_owned(),
                    selected_fields: requirement.selected_fields().map(str::to_owned).collect(),
                    row_identity: requirement.row_identity_field().map(str::to_owned),
                    native_family: requirement.native_family().canonical_token().to_owned(),
                    lifecycle: requirement.lifecycle().canonical_token().to_owned(),
                    requires_complete_result: requirement
                        .collection_policy()
                        .map(|policy| policy.requires_complete_result()),
                    permits_continuation: requirement
                        .collection_policy()
                        .map(|policy| policy.permits_continuation()),
                }
            }
            WorthUiSemanticDeclaration::Token(token) => Self::Token {
                name: token.name_text().to_owned(),
                authored_identity: token.authored_identity().map(str::to_owned),
                value: token.value_text().to_owned(),
            },
            WorthUiSemanticDeclaration::SemanticArtifact(artifact) => {
                let declaration = artifact.declaration();
                Self::SemanticArtifact {
                    key: declaration.key().as_str().to_owned(),
                    family: declaration.family().as_str().to_owned(),
                    published_aspects: semantic_texts(declaration.published_aspects()),
                    consumed_aspects: semantic_texts(declaration.consumed_aspects()),
                    structural_tokens: semantic_texts(declaration.structural_tokens()),
                    posture_tokens: semantic_texts(declaration.posture_tokens()),
                    support_tokens: semantic_texts(declaration.support_tokens()),
                }
            }
        }
    }

    fn fold_into(&self, fingerprint: &mut Fingerprint) {
        match self {
            Self::Import { target } => {
                fingerprint.fold_text("import");
                fingerprint.fold_text(target);
            }
            Self::Component(block) => {
                fingerprint.fold_text("component");
                block.fold_into(fingerprint);
            }
            Self::Surface(block) => {
                fingerprint.fold_text("surface");
                block.fold_into(fingerprint);
            }
            Self::Binding(block) => {
                fingerprint.fold_text("binding");
                block.fold_into(fingerprint);
            }
            Self::Projection {
                declaration,
                view,
                shape,
                selected_fields,
                row_identity,
                native_family,
                lifecycle,
                requires_complete_result,
                permits_continuation,
            } => {
                fingerprint.fold_text("projection");
                fingerprint.fold_text(declaration);
                fingerprint.fold_text(view);
                fingerprint.fold_text(shape);
                fingerprint.fold_texts(selected_fields);
                fingerprint.fold_optional_text(row_identity.as_deref());
                fingerprint.fold_text(native_family);
                fingerprint.fold_text(lifecycle);
                fingerprint.fold_optional_bool(*requires_complete_result);
                fingerprint.fold_optional_bool(*permits_continuation);
            }
            Self::Token {
                name,
                authored_identity,
                value,
            } => {
                fingerprint.fold_text("token");
                fingerprint.fold_text(name);
                fingerprint.fold_optional_text(authored_identity.as_deref());
                fingerprint.fold_text(value);
            }
            Self::SemanticArtifact {
                key,
                family,
                published_aspects,
                consumed_aspects,
                structural_tokens,
                posture_tokens,
                support_tokens,
            } => {
                fingerprint.fold_text("semantic-artifact");
                fingerprint.fold_text(key);
                fingerprint.fold_text(family);
                fingerprint.fold_texts(published_aspects);
                fingerprint.fold_texts(consumed_aspects);
                fingerprint.fold_texts(structural_tokens);
                fingerprint.fold_texts(posture_tokens);
                fingerprint.fold_texts(support_tokens);
            }
        }
    }
}

fn semantic_texts<T: SemanticText>(values: &[T]) -> Box<[String]> {
    values
        .iter()
        .map(|value| value.semantic_text().to_owned())
        .collect()
}

trait SemanticText {
    fn semantic_text(&self) -> &str;
}

macro_rules! semantic_text {
    ($type:ty) => {
        impl SemanticText for $type {
            fn semantic_text(&self) -> &str {
                self.as_str()
            }
        }
    };
}

semantic_text!(crate::UiDslAspectName);
semantic_text!(crate::UiDslStructuralToken);
semantic_text!(crate::UiDslPostureToken);
semantic_text!(crate::UiDslSupportToken);

impl WorthUiSemanticBlockExactBasis {
    fn from_block(block: &crate::source::WorthUiSemanticBlock) -> Self {
        Self {
            name: block.name_text().to_owned(),
            authored_identity: block.authored_identity().map(str::to_owned),
            structure: WorthUiStructuralBodyExactBasis::from_structure(block.structure()),
        }
    }

    fn fold_into(&self, fingerprint: &mut Fingerprint) {
        fingerprint.fold_text(&self.name);
        fingerprint.fold_optional_text(self.authored_identity.as_deref());
        self.structure.fold_into(fingerprint);
    }
}

impl WorthUiStructuralBodyExactBasis {
    fn from_structure(structure: &WorthUiAuthoredStructuralBody) -> Self {
        Self {
            root_regions: structure
                .root_regions()
                .iter()
                .map(WorthUiRegionExactBasis::from_region)
                .collect(),
        }
    }

    fn fold_into(&self, fingerprint: &mut Fingerprint) {
        fingerprint.fold_usize(self.root_regions.len());
        for region in &self.root_regions {
            region.fold_into(fingerprint);
        }
    }
}

impl WorthUiRegionExactBasis {
    fn from_region(region: &WorthUiAuthoredRegion) -> Self {
        Self {
            id: region.region_id_text().to_owned(),
            sizing_contract: region.sizing_contract_id_text().map(str::to_owned),
            state_slot: region.state_slot_id_text().map(str::to_owned),
            child_regions: region
                .child_regions()
                .iter()
                .map(Self::from_region)
                .collect(),
            mounts: region
                .mounts()
                .iter()
                .map(WorthUiMountExactBasis::from_mount)
                .collect(),
        }
    }

    fn fold_into(&self, fingerprint: &mut Fingerprint) {
        fingerprint.fold_text(&self.id);
        fingerprint.fold_optional_text(self.sizing_contract.as_deref());
        fingerprint.fold_optional_text(self.state_slot.as_deref());
        fingerprint.fold_usize(self.child_regions.len());
        for child in &self.child_regions {
            child.fold_into(fingerprint);
        }
        fingerprint.fold_usize(self.mounts.len());
        for mount in &self.mounts {
            mount.fold_into(fingerprint);
        }
    }
}

impl WorthUiMountExactBasis {
    fn from_mount(mount: &WorthUiAuthoredMount) -> Self {
        Self {
            surface: mount.surface_id_text().to_owned(),
            placement_policy: mount.placement_policy_id_text().map(str::to_owned),
            state_slot: mount.state_slot_id_text().map(str::to_owned),
        }
    }

    fn fold_into(&self, fingerprint: &mut Fingerprint) {
        fingerprint.fold_text(&self.surface);
        fingerprint.fold_optional_text(self.placement_policy.as_deref());
        fingerprint.fold_optional_text(self.state_slot.as_deref());
    }
}
