use forge_server::ForgeServerRequestContext;

fn bogus<T>() -> T {
    None::<T>.unwrap()
}

fn main() {
    let _context = ForgeServerRequestContext::new(
        bogus(),
        bogus(),
        bogus(),
        bogus(),
    );
}
