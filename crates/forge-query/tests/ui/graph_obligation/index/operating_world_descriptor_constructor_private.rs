use forge_query::facade::runtime::ForgeQueryGraphObligationOperatingWorldDescriptor;

fn main() {
    let _forged = ForgeQueryGraphObligationOperatingWorldDescriptor {
        ..ForgeQueryGraphObligationOperatingWorldDescriptor::preview()
    };
}
