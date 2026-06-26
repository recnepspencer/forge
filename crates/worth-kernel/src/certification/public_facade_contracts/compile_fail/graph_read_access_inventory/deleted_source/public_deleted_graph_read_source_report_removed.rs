use worth_kernel::query_adoption::{
    WorthKernelGraphReadAccessAdoptionError, WorthKernelGraphReadAccessAdoptionReport,
};

fn main() {
    let _ = std::mem::size_of::<WorthKernelGraphReadAccessAdoptionReport>();
    let _ = std::mem::size_of::<WorthKernelGraphReadAccessAdoptionError>();
}
