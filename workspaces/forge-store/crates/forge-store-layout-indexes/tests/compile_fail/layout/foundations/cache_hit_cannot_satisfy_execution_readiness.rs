use forge_store_buffer_pool::ResidentFrameAdmission;
use forge_store_layout_indexes::access_lowering::access_lowering;

fn main() {
    let cache_hit: ResidentFrameAdmission = todo!();
    let _ = access_lowering().admit_ready(cache_hit);
}
