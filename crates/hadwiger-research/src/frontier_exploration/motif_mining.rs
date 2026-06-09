use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::{ColorabilityVerificationPosture, HadwigerCanonicalArtifact};
use crate::domain_declarations::{declare_research_request_checked, MotifSeedDeclaration};
use crate::frontier_seeds::FrontierGraphSeedArtifact;
use crate::mathematical_verification::{
    KColorabilityVerificationChecked, UnitDistanceVerificationChecked,
};
use crate::motif_language::{
    certify_terminal_forcing_relation_checked, MotifArtifact, MotifForbiddenSameColorPair,
    MotifGeometryTemplateReference, MotifParameterBinding, MotifProofSupportPosture, MotifTerminal,
    MotifUnitEdge, MotifVertex, TerminalForcingRelation, TerminalForcingRelationCertificate,
    TerminalForcingRelationKind,
};
use crate::query_entry::HadwigerResearchHandle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualEdgeMotif {
    motif_reference: HadwigerArtifactReference,
}

impl VirtualEdgeMotif {
    pub fn motif_reference(&self) -> &HadwigerArtifactReference {
        &self.motif_reference
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BichromaticVirtualEdgeMotif {
    motif_reference: HadwigerArtifactReference,
}

impl BichromaticVirtualEdgeMotif {
    pub fn motif_reference(&self) -> &HadwigerArtifactReference {
        &self.motif_reference
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalColorForcingMotif {
    relation_reference: HadwigerArtifactReference,
}

impl TerminalColorForcingMotif {
    pub fn relation_reference(&self) -> &HadwigerArtifactReference {
        &self.relation_reference
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierMotifMiningReport {
    core: HadwigerArtifactCore,
    virtual_edge_motif: VirtualEdgeMotif,
    bichromatic_virtual_edge_motif: BichromaticVirtualEdgeMotif,
    terminal_forcing_motif: Option<TerminalColorForcingMotif>,
    motif: MotifArtifact,
    terminal_relation: Option<TerminalForcingRelation>,
}

impl FrontierMotifMiningReport {
    pub fn contains_virtual_edge_candidate(&self) -> bool {
        true
    }

    pub fn virtual_edge_motif(&self) -> &VirtualEdgeMotif {
        &self.virtual_edge_motif
    }

    pub fn bichromatic_virtual_edge_motif(&self) -> &BichromaticVirtualEdgeMotif {
        &self.bichromatic_virtual_edge_motif
    }

    pub fn terminal_forcing_motif(&self) -> Option<&TerminalColorForcingMotif> {
        self.terminal_forcing_motif.as_ref()
    }

    pub fn motif(&self) -> &MotifArtifact {
        &self.motif
    }

    pub fn terminal_relation(&self) -> Option<&TerminalForcingRelation> {
        self.terminal_relation.as_ref()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(FrontierMotifMiningReport, core);

pub fn mine_virtual_edge_motifs_checked(
    handle: &HadwigerResearchHandle,
    seed: &FrontierGraphSeedArtifact,
    unit_checked: &UnitDistanceVerificationChecked,
    color_checked: &KColorabilityVerificationChecked,
) -> Result<FrontierMotifMiningReport, HadwigerArtifactShapeError> {
    let source_declaration = match declare_research_request_checked(
        handle,
        MotifSeedDeclaration::new(format!("{}-virtual-edge", seed.seed_id()))
            .with_source_family(seed.source_family())
            .with_novelty_signature(format!("{}:virtual-edge", seed.source_digest())),
    ) {
        forge_query::facade::ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => {
            declaration.into()
        }
        _ => {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "motif_seed_declaration",
            })
        }
    };
    let checked_evidence = unit_checked.verification().is_admitted()
        && color_checked.colorability_verification().posture()
            == ColorabilityVerificationPosture::UnsatVerified
        && color_checked
            .not_k_colorable_aspect()
            .satisfies_mathematical_dependency();
    let posture = if checked_evidence {
        MotifProofSupportPosture::CheckerSupported
    } else {
        MotifProofSupportPosture::Candidate
    };
    let motif = build_virtual_edge_motif(seed, source_declaration, posture)?;
    let terminal_relation = if checked_evidence {
        let certificate = TerminalForcingRelationCertificate::from_checked_colorability(
            format!("{}-must-differ", seed.seed_id()),
            motif.reference(),
            TerminalForcingRelationKind::MustDiffer,
            ["left", "right"],
            color_checked.colorability_verification().clone(),
            color_checked.not_k_colorable_aspect().clone(),
        )
        .map_err(|_| HadwigerArtifactShapeError::EmptyField {
            field: "terminal_forcing_certificate",
        })?;
        let study = crate::domain_declarations::TerminalForcingStudyDeclaration::new(
            format!("{}-terminal-study", seed.seed_id()),
            motif.reference().stable_token(),
        )
        .with_terminal("left")
        .map_err(|_| HadwigerArtifactShapeError::EmptyField { field: "left" })?
        .with_terminal("right")
        .map_err(|_| HadwigerArtifactShapeError::EmptyField { field: "right" })?
        .with_relation_goal(TerminalForcingRelationKind::MustDiffer.as_str());
        Some(
            certify_terminal_forcing_relation_checked(handle, study, &motif, certificate).map_err(
                |_| HadwigerArtifactShapeError::EmptyField {
                    field: "terminal_forcing_relation",
                },
            )?,
        )
    } else {
        None
    };
    let mut parents = vec![
        seed.reference(),
        motif.reference(),
        unit_checked.verification().reference(),
        color_checked.colorability_verification().reference(),
    ];
    if let Some(relation) = &terminal_relation {
        parents.push(relation.reference());
    }
    let core = artifact_core(
        HadwigerArtifactKind::FrontierMotifMiningReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "frontier_virtual_edge_motif_mining".to_string(),
        },
        parents,
        vec![
            HadwigerArtifactPayloadEntry::text("seed_id", seed.seed_id()),
            HadwigerArtifactPayloadEntry::text("source_family", seed.source_family()),
            HadwigerArtifactPayloadEntry::text("checked_evidence", checked_evidence.to_string()),
        ],
    )?;
    let virtual_edge_motif = VirtualEdgeMotif {
        motif_reference: motif.reference(),
    };
    let bichromatic_virtual_edge_motif = BichromaticVirtualEdgeMotif {
        motif_reference: motif.reference(),
    };
    let terminal_forcing_motif =
        terminal_relation
            .as_ref()
            .map(|relation| TerminalColorForcingMotif {
                relation_reference: relation.reference(),
            });
    Ok(FrontierMotifMiningReport {
        core,
        virtual_edge_motif,
        bichromatic_virtual_edge_motif,
        terminal_forcing_motif,
        motif,
        terminal_relation,
    })
}

pub(crate) fn mine_virtual_edge_motif_candidates_checked(
    handle: &HadwigerResearchHandle,
    seed: &FrontierGraphSeedArtifact,
) -> Result<FrontierMotifMiningReport, HadwigerArtifactShapeError> {
    let source_declaration = match declare_research_request_checked(
        handle,
        MotifSeedDeclaration::new(format!("{}-virtual-edge", seed.seed_id()))
            .with_source_family(seed.source_family())
            .with_novelty_signature(format!("{}:virtual-edge", seed.source_digest())),
    ) {
        forge_query::facade::ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => {
            declaration.into()
        }
        _ => return Err(shape_error("motif_seed_declaration")),
    };
    let motif = build_virtual_edge_motif(
        seed,
        source_declaration,
        MotifProofSupportPosture::Candidate,
    )?;
    let core = artifact_core(
        HadwigerArtifactKind::FrontierMotifMiningReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "frontier_virtual_edge_candidate_mining".to_string(),
        },
        vec![seed.reference(), motif.reference()],
        vec![
            HadwigerArtifactPayloadEntry::text("seed_id", seed.seed_id()),
            HadwigerArtifactPayloadEntry::text("source_family", seed.source_family()),
            HadwigerArtifactPayloadEntry::text("checked_evidence", "false"),
        ],
    )?;
    Ok(FrontierMotifMiningReport {
        core,
        virtual_edge_motif: VirtualEdgeMotif {
            motif_reference: motif.reference(),
        },
        bichromatic_virtual_edge_motif: BichromaticVirtualEdgeMotif {
            motif_reference: motif.reference(),
        },
        terminal_forcing_motif: None,
        motif,
        terminal_relation: None,
    })
}

fn build_virtual_edge_motif(
    seed: &FrontierGraphSeedArtifact,
    source_declaration: crate::domain_artifacts::HadwigerQueryDeclarationReference,
    posture: MotifProofSupportPosture,
) -> Result<MotifArtifact, HadwigerArtifactShapeError> {
    MotifArtifact::builder(
        format!("{}-virtual-edge", seed.seed_id()),
        source_declaration,
    )
    .with_source_family(seed.source_family())
    .map_err(|_| shape_error("motif_source_family"))?
    .with_novelty_signature(format!("{}:virtual-edge", seed.source_digest()))
    .map_err(|_| shape_error("motif_novelty_signature"))?
    .with_proof_support_posture(posture)
    .with_geometry_template(MotifGeometryTemplateReference::new(
        "frontier-seed-terminal-forcing-template",
    )?)
    .with_vertex(MotifVertex::new("terminal-left-vertex")?)
    .map_err(|_| shape_error("motif_left_vertex"))?
    .with_vertex(MotifVertex::new("terminal-right-vertex")?)
    .map_err(|_| shape_error("motif_right_vertex"))?
    .with_terminal(MotifTerminal::new("left")?)
    .map_err(|_| shape_error("motif_left_terminal"))?
    .with_terminal(MotifTerminal::new("right")?)
    .map_err(|_| shape_error("motif_right_terminal"))?
    .with_parameter(MotifParameterBinding::new(
        "seed_vertex_count",
        seed.vertex_count().to_string(),
    )?)
    .map_err(|_| shape_error("motif_vertex_count_parameter"))?
    .with_parameter(MotifParameterBinding::new(
        "seed_edge_count",
        seed.edge_count().to_string(),
    )?)
    .map_err(|_| shape_error("motif_edge_count_parameter"))?
    .with_unit_edge(MotifUnitEdge::new(
        "terminal-left-vertex",
        "terminal-right-vertex",
    )?)
    .map_err(|_| shape_error("motif_unit_edge"))?
    .with_forbidden_same_color_pair(MotifForbiddenSameColorPair::new("left", "right")?)
    .map_err(|_| shape_error("motif_forbidden_pair"))?
    .finish()
    .map_err(|_| shape_error("motif_finish"))
}

fn shape_error(field: &'static str) -> HadwigerArtifactShapeError {
    HadwigerArtifactShapeError::EmptyField { field }
}
