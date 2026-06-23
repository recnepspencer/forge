use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneAgreementDenial, PlanarBooleanCommonPlaneOperandSide,
    PlanarBooleanCommonPlaneWitness,
};
use crate::workload_platform::surface_support::CertifiedSurfaceSupport;

pub struct PlanarBooleanCommonPlaneAgreementWorkload {
    left_surface_support: CertifiedSurfaceSupport,
    right_surface_support: CertifiedSurfaceSupport,
    declaration: String,
}

impl PlanarBooleanCommonPlaneAgreementWorkload {
    pub fn for_surface_support_pair(
        left_surface_support: CertifiedSurfaceSupport,
        right_surface_support: CertifiedSurfaceSupport,
    ) -> Self {
        Self {
            left_surface_support,
            right_surface_support,
            declaration: "planar boolean common-plane agreement".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn certify(
        self,
    ) -> Result<
        crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneAgreementReceipt,
        PlanarBooleanCommonPlaneAgreementDenial,
    >{
        require_declaration(&self.declaration)?;

        let left_support_identity = self
            .left_surface_support
            .receipts()
            .stage_identity()
            .receipt_identity()
            .to_string();
        let right_support_identity = self
            .right_surface_support
            .receipts()
            .stage_identity()
            .receipt_identity()
            .to_string();
        let left_witness = certify_operand_plane_witness(
            &self.left_surface_support,
            PlanarBooleanCommonPlaneOperandSide::Left,
            &left_support_identity,
        )?;
        let right_witness = certify_operand_plane_witness(
            &self.right_surface_support,
            PlanarBooleanCommonPlaneOperandSide::Right,
            &right_support_identity,
        )?;

        require_matching_plane_identity(
            &left_witness,
            &right_witness,
            left_support_identity.clone(),
            right_support_identity.clone(),
        )?;

        Ok(
            crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneAgreementReceipt::new(
                &self.declaration,
                left_support_identity,
                right_support_identity,
                left_witness,
                right_witness,
            ),
        )
    }
}

fn require_declaration(declaration: &str) -> Result<(), PlanarBooleanCommonPlaneAgreementDenial> {
    if declaration.trim().is_empty() {
        Err(PlanarBooleanCommonPlaneAgreementDenial::MissingDeclaration)
    } else {
        Ok(())
    }
}

fn certify_operand_plane_witness(
    surface_support: &CertifiedSurfaceSupport,
    side: PlanarBooleanCommonPlaneOperandSide,
    surface_support_identity: &str,
) -> Result<PlanarBooleanCommonPlaneWitness, PlanarBooleanCommonPlaneAgreementDenial> {
    let (unique_plane_digests, supporting_face_rows) =
        collect_unique_face_plane_digests(surface_support);
    require_nonempty_plane_witness(
        side,
        surface_support_identity,
        &unique_plane_digests,
        supporting_face_rows,
    )?;
    require_unambiguous_plane_witness(side, surface_support_identity, &unique_plane_digests)?;

    let plane_identity_digest = unique_plane_digests
        .into_iter()
        .next()
        .expect("single plane witness must exist");
    Ok(PlanarBooleanCommonPlaneWitness::new(
        plane_identity_digest,
        supporting_face_rows,
    ))
}

fn collect_unique_face_plane_digests(
    surface_support: &CertifiedSurfaceSupport,
) -> (BTreeSet<String>, usize) {
    let mut unique_plane_digests = BTreeSet::new();
    let mut supporting_face_rows = 0usize;
    for row in surface_support.geometry_snapshot().face_rows() {
        if row.support_plane_identity_digests().is_empty() {
            continue;
        }
        supporting_face_rows += 1;
        unique_plane_digests.extend(row.support_plane_identity_digests().iter().cloned());
    }
    (unique_plane_digests, supporting_face_rows)
}

fn require_nonempty_plane_witness(
    side: PlanarBooleanCommonPlaneOperandSide,
    surface_support_identity: &str,
    unique_plane_digests: &BTreeSet<String>,
    supporting_face_rows: usize,
) -> Result<(), PlanarBooleanCommonPlaneAgreementDenial> {
    if supporting_face_rows == 0 || unique_plane_digests.is_empty() {
        Err(
            PlanarBooleanCommonPlaneAgreementDenial::MissingCertifiedFacePlaneWitness {
                side,
                surface_support_identity: surface_support_identity.to_string(),
            },
        )
    } else {
        Ok(())
    }
}

fn require_unambiguous_plane_witness(
    side: PlanarBooleanCommonPlaneOperandSide,
    surface_support_identity: &str,
    unique_plane_digests: &BTreeSet<String>,
) -> Result<(), PlanarBooleanCommonPlaneAgreementDenial> {
    if unique_plane_digests.len() != 1 {
        Err(
            PlanarBooleanCommonPlaneAgreementDenial::AmbiguousCertifiedFacePlaneWitness {
                side,
                surface_support_identity: surface_support_identity.to_string(),
                plane_identity_count: unique_plane_digests.len(),
            },
        )
    } else {
        Ok(())
    }
}

fn require_matching_plane_identity(
    left_witness: &PlanarBooleanCommonPlaneWitness,
    right_witness: &PlanarBooleanCommonPlaneWitness,
    left_surface_support_identity: String,
    right_surface_support_identity: String,
) -> Result<(), PlanarBooleanCommonPlaneAgreementDenial> {
    if left_witness.plane_identity_digest() == right_witness.plane_identity_digest() {
        Ok(())
    } else {
        Err(
            PlanarBooleanCommonPlaneAgreementDenial::DistinctCertifiedPlanes {
                left_surface_support_identity,
                right_surface_support_identity,
                left_plane_identity_digest: left_witness.plane_identity_digest().to_string(),
                right_plane_identity_digest: right_witness.plane_identity_digest().to_string(),
            },
        )
    }
}
