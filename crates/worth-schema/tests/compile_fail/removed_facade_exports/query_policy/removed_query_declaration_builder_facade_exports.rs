use schema::facade::{
    QueryComputedDeclarationBuilder, QueryDeclarationError, QueryLiveDeclarationBuilder,
};

fn main() {
    let _ = (
        None::<QueryLiveDeclarationBuilder>,
        None::<QueryComputedDeclarationBuilder>,
        None::<QueryDeclarationError>,
    );
}
