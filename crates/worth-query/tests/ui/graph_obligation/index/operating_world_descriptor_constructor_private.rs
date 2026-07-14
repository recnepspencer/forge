use worth_query::facade::runtime::WorthQueryGraphObligationOperatingWorldDescriptor;

fn main() {
    let _worthd = WorthQueryGraphObligationOperatingWorldDescriptor {
        ..WorthQueryGraphObligationOperatingWorldDescriptor::preview()
    };
}
