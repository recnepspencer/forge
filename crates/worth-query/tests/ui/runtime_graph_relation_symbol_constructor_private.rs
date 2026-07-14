use worth_query::facade::runtime::WorthQueryGraphRelationSymbol;

#[allow(invalid_value)]
fn main() {
    let reference = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    let _ = WorthQueryGraphRelationSymbol {
        reference,
    };
}
