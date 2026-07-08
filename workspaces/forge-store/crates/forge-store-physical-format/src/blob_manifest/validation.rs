use super::{
    BlobPhysicalManifestDenial, BlobPhysicalManifestDenialKind, BlobPhysicalManifestRow,
    BlobPhysicalManifestRowKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPhysicalManifestValidation {
    reachability: BlobPhysicalManifestRow,
    placement: BlobPhysicalManifestRow,
}

impl BlobPhysicalManifestValidation {
    pub fn validate(
        reachability: BlobPhysicalManifestRow,
        placement: BlobPhysicalManifestRow,
    ) -> Result<Self, BlobPhysicalManifestDenial> {
        if reachability.kind() != BlobPhysicalManifestRowKind::Reachability {
            return Err(BlobPhysicalManifestDenial::new(
                BlobPhysicalManifestDenialKind::WrongRowKind,
                reachability.row_digest(),
            ));
        }
        if placement.kind() != BlobPhysicalManifestRowKind::Placement {
            return Err(BlobPhysicalManifestDenial::new(
                BlobPhysicalManifestDenialKind::WrongRowKind,
                placement.row_digest(),
            ));
        }
        if !placement.external_chunk_present() {
            return Err(BlobPhysicalManifestDenial::new(
                BlobPhysicalManifestDenialKind::MissingExternalChunk,
                placement.row_digest(),
            ));
        }
        if reachability.generation_sequence() != placement.generation_sequence() {
            return Err(BlobPhysicalManifestDenial::new(
                BlobPhysicalManifestDenialKind::StaleGenerationRow,
                placement.row_digest(),
            ));
        }
        Ok(Self {
            reachability,
            placement,
        })
    }

    pub fn reject_orphaned_placement(
        placement: BlobPhysicalManifestRow,
    ) -> BlobPhysicalManifestDenial {
        BlobPhysicalManifestDenial::new(
            BlobPhysicalManifestDenialKind::OrphanedPlacementResidue,
            placement.row_digest(),
        )
    }

    pub const fn reachability(&self) -> &BlobPhysicalManifestRow {
        &self.reachability
    }

    pub const fn placement(&self) -> &BlobPhysicalManifestRow {
        &self.placement
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_validation_detects_missing_stale_and_orphaned_rows() {
        let missing = BlobPhysicalManifestValidation::validate(
            physical_row(
                BlobPhysicalManifestRowKind::Reachability,
                1,
                true,
                "missing-r",
            ),
            physical_row(
                BlobPhysicalManifestRowKind::Placement,
                1,
                false,
                "missing-p",
            ),
        )
        .expect_err("missing external chunk must deny");
        assert_eq!(
            missing.kind(),
            BlobPhysicalManifestDenialKind::MissingExternalChunk
        );

        let stale = BlobPhysicalManifestValidation::validate(
            physical_row(
                BlobPhysicalManifestRowKind::Reachability,
                1,
                true,
                "stale-r",
            ),
            physical_row(BlobPhysicalManifestRowKind::Placement, 2, true, "stale-p"),
        )
        .expect_err("stale generation row must deny");
        assert_eq!(
            stale.kind(),
            BlobPhysicalManifestDenialKind::StaleGenerationRow
        );

        let orphan = BlobPhysicalManifestValidation::reject_orphaned_placement(physical_row(
            BlobPhysicalManifestRowKind::Placement,
            1,
            true,
            "orphan-p",
        ));
        assert_eq!(
            orphan.kind(),
            BlobPhysicalManifestDenialKind::OrphanedPlacementResidue
        );
    }

    fn physical_row(
        kind: BlobPhysicalManifestRowKind,
        generation: u64,
        present: bool,
        digest: &str,
    ) -> BlobPhysicalManifestRow {
        BlobPhysicalManifestRow::new(kind, digest, generation, present).expect("row admits")
    }
}
