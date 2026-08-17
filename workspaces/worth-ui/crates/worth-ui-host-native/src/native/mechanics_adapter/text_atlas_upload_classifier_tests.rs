use super::*;
use crate::native::text_atlas::{
    UiNativeTextAtlas, UiNativeTextAtlasDemand, UiNativeTextAtlasDenial,
    UiNativeTextAtlasPinTransition, UiNativeTextAtlasUpload,
};
use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiGlyphRasterDemandIdentity,
    UiGlyphRasterFractionalOrigin, UiGlyphRasterKey, UiGlyphRasterKeyInput, UiGlyphRasterPalette,
    UiGlyphRasterSize, UiGlyphRasterSource, UiGlyphVariationCoordinates,
    UiQualifiedFontFaceIdentity, UiTextProfileGeneration,
};

struct FaultAfterFirst {
    uploads: usize,
}

impl AtlasUploadOperations for FaultAfterFirst {
    fn page_count(&self, _kind: UiNativeGpuAtlasKind) -> usize {
        1
    }

    fn ensure_page(&mut self, _kind: UiNativeGpuAtlasKind) -> Result<(), UiNativeTextAtlasDenial> {
        Ok(())
    }

    fn upload(
        &mut self,
        _transaction: u64,
        _validated: &ValidatedUpload,
        _upload: &UiNativeTextAtlasUpload,
    ) -> Result<(), UiNativeTextAtlasDenial> {
        self.uploads += 1;
        (self.uploads == 1)
            .then_some(())
            .ok_or(UiNativeTextAtlasDenial::UploadRejected)
    }
}

#[test]
fn production_upload_loop_classifies_failure_after_the_first_submit_as_indeterminate() {
    let first_key = key(1);
    let second_key = key(2);
    let identity = UiGlyphRasterDemandIdentity::from_text_mechanics([11; 32]);
    let demands = [
        UiNativeTextAtlasDemand::from_native_geometry(identity, first_key, 2, 2, 4),
        UiNativeTextAtlasDemand::from_native_geometry(identity, second_key, 2, 2, 4),
    ];
    let atlas = UiNativeTextAtlas::new();
    let plan = atlas
        .plan_demands(&demands, &UiNativeTextAtlasPinTransition::default())
        .unwrap();
    let uploads = [upload(first_key), upload(second_key)];
    let failure = submit_uploads(
        &mut FaultAfterFirst { uploads: 0 },
        UploadRequest {
            plan: &plan,
            uploads: &uploads,
        },
    )
    .unwrap_err();
    assert!(matches!(failure, GpuUploadFailure::Indeterminate));
}

fn upload(key: UiGlyphRasterKey) -> UiNativeTextAtlasUpload {
    UiNativeTextAtlasUpload::from_text_mechanics(key, 2, 2, 2, vec![0; 4], [0; 32])
}

fn key(glyph_id: u32) -> UiGlyphRasterKey {
    UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
        font_collection: UiFontCollectionGeneration::new(1).unwrap(),
        font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([4; 32]),
        profile: UiTextProfileGeneration::new(1).unwrap(),
        face: UiQualifiedFontFaceIdentity::from_text_mechanics([5; 32], 0),
        glyph_id,
        variations: UiGlyphVariationCoordinates::empty(),
        palette: UiGlyphRasterPalette::new(0),
        size: UiGlyphRasterSize::from_millipoints(12_000).unwrap(),
        source: UiGlyphRasterSource::AlphaOutline,
        dpi_milli: 1_000,
        origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
    })
    .unwrap()
}
