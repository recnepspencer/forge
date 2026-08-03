use syn::{BinOp, Expr, UnOp};

use super::super::read_repository_document;
use super::wal_source_syntax::ParsedRustSource;

const REOPEN: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                      durability/wal/inventory/reopen.rs";

pub(super) fn inspect(reopen: &str) -> Result<(), String> {
    let source = ParsedRustSource::parse(reopen, "WAL reopen classification owner")?;
    let reopen = source.function("reopen_wal_inventory")?;
    let predicate = reopen.let_initializer("requires_inspection")?;
    if requires_absent_cutoff_and_missing_origin(predicate) {
        Ok(())
    } else {
        Err(
            "WAL reopen classification must require both an absent cutoff and missing canonical origin"
                .to_owned(),
        )
    }
}

#[test]
fn wal_reopen_classification_calls_the_canonical_origin_owner() {
    inspect(&read(REOPEN)).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn comments_cannot_supply_a_missing_reopen_classification_operand() {
    let source = read(REOPEN);
    let comment_counterfeit = replace_once(
        &source,
        "cutoff.lsn().is_none() && !segment_inventory.retains_canonical_wal_origin()",
        "cutoff.lsn().is_none() /* && !segment_inventory.retains_canonical_wal_origin() */",
    );
    assert!(inspect(&comment_counterfeit).is_err());

    let unproved_trust = replace_once(
        &source,
        "cutoff.lsn().is_none() && !segment_inventory.retains_canonical_wal_origin()",
        "false",
    );
    assert!(inspect(&unproved_trust).is_err());
}

#[test]
fn delimiter_literals_do_not_change_reopen_function_ownership() {
    let source = read(REOPEN);
    let with_literal = replace_once(
        &source,
        "let directory = wal_directory();",
        "let _delimiter_literal = \"{ }\";\n    let directory = wal_directory();",
    );
    inspect(&with_literal).expect("literal braces are syntax-owned");
}

fn requires_absent_cutoff_and_missing_origin(expression: &Expr) -> bool {
    let Expr::Binary(conjunction) = strip_groups(expression) else {
        return false;
    };
    if !matches!(conjunction.op, BinOp::And(_)) {
        return false;
    }
    is_absent_cutoff(conjunction.left.as_ref())
        && is_missing_canonical_origin(conjunction.right.as_ref())
}

fn is_absent_cutoff(expression: &Expr) -> bool {
    let Some(cutoff_lsn) = method_receiver(expression, "is_none") else {
        return false;
    };
    method_receiver(cutoff_lsn, "lsn").is_some()
}

fn is_missing_canonical_origin(expression: &Expr) -> bool {
    let Expr::Unary(negation) = strip_groups(expression) else {
        return false;
    };
    matches!(negation.op, UnOp::Not(_))
        && method_receiver(negation.expr.as_ref(), "retains_canonical_wal_origin").is_some()
}

fn method_receiver<'expression>(
    expression: &'expression Expr,
    method: &str,
) -> Option<&'expression Expr> {
    let Expr::MethodCall(call) = strip_groups(expression) else {
        return None;
    };
    (call.method == method).then_some(call.receiver.as_ref())
}

fn strip_groups(mut expression: &Expr) -> &Expr {
    loop {
        expression = match expression {
            Expr::Group(group) => group.expr.as_ref(),
            Expr::Paren(parenthesized) => parenthesized.expr.as_ref(),
            _ => return expression,
        };
    }
}

fn read(path: &str) -> String {
    read_repository_document(path).unwrap_or_else(|error| panic!("{error}"))
}

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert_eq!(source.matches(from).count(), 1, "control anchor: {from}");
    source.replacen(from, to, 1)
}
