use worth_foundational::{
    admit_canonical_export_digest_derivation, derive_canonical_digest,
    prepare_canonical_basis_bundle, prepare_canonical_export_bundle, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalEquivalenceBasis,
    CanonicalExportBundleDigestAlgorithmSlot, CanonicalExportReadyArtifact, CanonicalProducerShape,
};
use worth_proof::TransitionOutcome;

use crate::BlobChunkRootCanonicalBasis;

use super::counters::BlobExportBundleCounters;
use super::denial::BlobExportBundleDenial;

pub(crate) fn prepare_export_artifact(
    export_name: &str,
    canonical_basis: &BlobChunkRootCanonicalBasis,
) -> Result<(CanonicalExportReadyArtifact, CanonicalDerivedDigest), BlobExportBundleDenial> {
    let bundle = match prepare_canonical_basis_bundle(
        canonical_basis.ready_basis().payload().version().clone(),
        [canonical_basis.ready_basis().clone()],
    ) {
        TransitionOutcome::Success(bundle) => bundle,
        _ => {
            return Err(BlobExportBundleDenial::CanonicalExportConstructionDenied {
                counters: BlobExportBundleCounters::start(),
            });
        }
    };
    let export = match prepare_canonical_export_bundle(
        export_name,
        CanonicalProducerShape::NativeFoundational,
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        bundle,
    ) {
        TransitionOutcome::Success(export) => export,
        _ => {
            return Err(BlobExportBundleDenial::CanonicalExportConstructionDenied {
                counters: BlobExportBundleCounters::start(),
            });
        }
    };
    let digest_bundle = match prepare_canonical_basis_bundle(
        canonical_basis.ready_basis().payload().version().clone(),
        [canonical_basis.ready_basis().clone()],
    ) {
        TransitionOutcome::Success(bundle) => bundle,
        _ => {
            return Err(BlobExportBundleDenial::CanonicalExportConstructionDenied {
                counters: BlobExportBundleCounters::start(),
            });
        }
    };
    let digest_export = match prepare_canonical_export_bundle(
        export_name,
        CanonicalProducerShape::NativeFoundational,
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        digest_bundle,
    ) {
        TransitionOutcome::Success(export) => export,
        _ => {
            return Err(BlobExportBundleDenial::CanonicalExportConstructionDenied {
                counters: BlobExportBundleCounters::start(),
            });
        }
    };
    let slot = CanonicalExportBundleDigestAlgorithmSlot::export_bundle(
        CanonicalDigestAlgorithmId::test_stable_fixture(),
        export.payload().bundle().version().clone(),
    );
    let digest_ready = match admit_canonical_export_digest_derivation(digest_export, slot) {
        TransitionOutcome::Success(ready) => ready,
        _ => {
            return Err(BlobExportBundleDenial::CanonicalExportDigestDenied {
                counters: BlobExportBundleCounters::start(),
            });
        }
    };
    Ok((export, derive_canonical_digest(digest_ready)))
}
