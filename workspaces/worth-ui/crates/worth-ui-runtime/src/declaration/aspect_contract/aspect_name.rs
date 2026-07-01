use worth_ui_dsl::UiDslAspectName;

use crate::declaration::aspect_contract::UiAspectContractAdmissionDenial;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAspectFamily {
    Structure,
    Presence,
    Participation,
    Layout,
    Appearance,
    Content,
    Interaction,
    Service,
    Diagnostic,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAspectSemanticSlice {
    StructureProductRoot,
    AppearanceBackground,
    ContentText,
    InteractionOperability,
}

impl UiAspectSemanticSlice {
    pub fn family(self) -> UiAspectFamily {
        match self {
            Self::StructureProductRoot => UiAspectFamily::Structure,
            Self::AppearanceBackground => UiAspectFamily::Appearance,
            Self::ContentText => UiAspectFamily::Content,
            Self::InteractionOperability => UiAspectFamily::Interaction,
        }
    }

    pub fn canonical_label(self) -> &'static str {
        match self {
            Self::StructureProductRoot => "structure.product-root",
            Self::AppearanceBackground => "appearance.background",
            Self::ContentText => "content.text",
            Self::InteractionOperability => "interaction.operability",
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAspectName {
    semantic_slice: UiAspectSemanticSlice,
}

impl UiAspectName {
    pub(crate) fn admit(
        authored: &UiDslAspectName,
    ) -> Result<Self, UiAspectContractAdmissionDenial> {
        let canonical_segments = canonicalize_aspect_segments(authored.as_str())?;
        let family = admit_aspect_family(authored.as_str(), &canonical_segments[0])?;
        let semantic_slice = admit_aspect_semantic_slice(family, &canonical_segments)?;

        Ok(Self { semantic_slice })
    }

    pub(crate) fn digest_text(&self) -> &str {
        self.semantic_slice.canonical_label()
    }

    pub fn family(&self) -> UiAspectFamily {
        self.semantic_slice.family()
    }

    pub fn semantic_slice(&self) -> UiAspectSemanticSlice {
        self.semantic_slice
    }

    pub fn canonical_label(&self) -> &str {
        self.semantic_slice.canonical_label()
    }
}

fn canonicalize_aspect_segments(
    authored: &str,
) -> Result<Vec<String>, UiAspectContractAdmissionDenial> {
    let canonical_segments = authored
        .split('.')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_ascii_lowercase())
        .collect::<Vec<_>>();

    if canonical_segments.is_empty() {
        return Err(UiAspectContractAdmissionDenial::MalformedAspectName {
            authored: authored.to_owned(),
        });
    }

    Ok(canonical_segments)
}

fn admit_aspect_family(
    authored: &str,
    authored_family: &str,
) -> Result<UiAspectFamily, UiAspectContractAdmissionDenial> {
    match authored_family {
        "structure" => Ok(UiAspectFamily::Structure),
        "presence" => Ok(UiAspectFamily::Presence),
        "participation" => Ok(UiAspectFamily::Participation),
        "layout" => Ok(UiAspectFamily::Layout),
        "appearance" => Ok(UiAspectFamily::Appearance),
        "content" => Ok(UiAspectFamily::Content),
        "interaction" => Ok(UiAspectFamily::Interaction),
        "service" => Ok(UiAspectFamily::Service),
        "diagnostic" => Ok(UiAspectFamily::Diagnostic),
        other => Err(UiAspectContractAdmissionDenial::UnsupportedAspectFamily {
            authored: authored.to_owned(),
            observed_family: other.to_owned(),
        }),
    }
}

fn admit_aspect_semantic_slice(
    family: UiAspectFamily,
    canonical_segments: &[String],
) -> Result<UiAspectSemanticSlice, UiAspectContractAdmissionDenial> {
    match (family, canonical_segments) {
        (UiAspectFamily::Structure, [_, slice]) if slice == "product-root" => {
            Ok(UiAspectSemanticSlice::StructureProductRoot)
        }
        (UiAspectFamily::Appearance, [_, slice]) if slice == "background" => {
            Ok(UiAspectSemanticSlice::AppearanceBackground)
        }
        (UiAspectFamily::Content, [_, slice]) if slice == "text" => {
            Ok(UiAspectSemanticSlice::ContentText)
        }
        (UiAspectFamily::Interaction, [_, slice]) if slice == "operability" => {
            Ok(UiAspectSemanticSlice::InteractionOperability)
        }
        _ => Err(
            UiAspectContractAdmissionDenial::UnsupportedAspectSemanticSlice {
                family,
                canonical_label: canonical_segments.join("."),
            },
        ),
    }
}
