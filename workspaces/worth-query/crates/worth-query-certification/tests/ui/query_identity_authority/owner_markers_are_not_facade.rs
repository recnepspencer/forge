use worth_query::facade::identity_authority::{
    QueryOperationProgressionAuthority, QueryReceiptAdmissionAuthority,
};

fn main() {
    let _ = std::any::TypeId::of::<QueryReceiptAdmissionAuthority>();
    let _ = std::any::TypeId::of::<QueryOperationProgressionAuthority>();
}
