use forge_query::facade::ForgeQueryGraphRelationSymbol;

#[allow(invalid_value)]
fn main() {
    let reference = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    let _ = ForgeQueryGraphRelationSymbol {
        reference,
    };
}
