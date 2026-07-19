#[macro_export]
macro_rules! worth_query_schema {
    ($($tokens:tt)*) => {
        worth_query_declaration::worth_query_schema! { $($tokens)* }
    };
}
