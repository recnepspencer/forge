#[derive(Clone, Copy)]
pub(super) struct ProfileFaceDescriptor {
    pub(super) id: &'static str,
    pub(super) path: &'static str,
    pub(super) face_index: u32,
    pub(super) byte_length: usize,
    pub(super) digest: [u8; 32],
    pub(super) fallback_rank: u16,
    pub(super) emoji: bool,
    pub(super) last_resort: bool,
}

include!(concat!(env!("OUT_DIR"), "/global_text_profile.rs"));

pub(crate) fn is_rgi_emoji(source: &str) -> bool {
    UNICODE_17_RGI_EMOJI.binary_search(&source).is_ok()
}
