use worth_store_physical_integrity::PreDecodePhysicalDenial;

use crate::corruption::classification::classify_physical_damage_before_decode;
use crate::BlobDamageCase;

pub fn classify_physical_pre_decode_damage(denial: &PreDecodePhysicalDenial) -> BlobDamageCase {
    classify_physical_damage_before_decode(denial)
}
