use forge_server::ForgeServerAdmission;

fn bogus<T>() -> T {
    None::<T>.unwrap()
}

fn main() {
    let _admission = ForgeServerAdmission::new(bogus(), bogus());
}
