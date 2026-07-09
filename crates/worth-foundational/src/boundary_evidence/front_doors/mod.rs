mod attachments;
mod lineage;
mod primitives;
mod provenance;
mod receipts;
mod support;

use super::attachment_front_doors::FoundationalBoundaryEvidenceAttachmentFrontDoor;
use super::lineage_front_doors::FoundationalBoundaryEvidenceLineageFrontDoor;
use super::provenance_front_doors::FoundationalBoundaryEvidenceProvenanceFrontDoor;
use super::receipt_front_doors::FoundationalBoundaryEvidenceReceiptFrontDoor;
use super::support_front_doors::FoundationalBoundaryEvidenceSupportFrontDoor;

#[derive(Debug, Clone, Copy, Default)]
pub struct BoundaryEvidenceFrontDoor;

pub fn boundary_evidence() -> BoundaryEvidenceFrontDoor {
    BoundaryEvidenceFrontDoor
}

pub const fn provenance() -> FoundationalBoundaryEvidenceProvenanceFrontDoor {
    FoundationalBoundaryEvidenceProvenanceFrontDoor
}

pub const fn lineage() -> FoundationalBoundaryEvidenceLineageFrontDoor {
    FoundationalBoundaryEvidenceLineageFrontDoor
}

pub const fn receipt() -> FoundationalBoundaryEvidenceReceiptFrontDoor {
    FoundationalBoundaryEvidenceReceiptFrontDoor
}

pub const fn support() -> FoundationalBoundaryEvidenceSupportFrontDoor {
    FoundationalBoundaryEvidenceSupportFrontDoor
}

pub const fn attachment() -> FoundationalBoundaryEvidenceAttachmentFrontDoor {
    FoundationalBoundaryEvidenceAttachmentFrontDoor
}
