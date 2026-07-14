mod encoding;
mod manifest_encoding;
mod support;

use crate::PhysicalSecurityMetadataResultExclusion;

#[test]
fn physical_metadata_vocabulary_excludes_authenticity_result() {
    let _ =
        PhysicalSecurityMetadataResultExclusion::authenticity_result_is_not_metadata_declaration();
}
