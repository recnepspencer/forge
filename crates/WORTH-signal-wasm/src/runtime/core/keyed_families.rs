use crate::expression::model::{Expr, SignalValue};
use crate::recipe::model::RecipeFamilyReadSpec;

pub(super) fn rgba_signal_value(r: u8, g: u8, b: u8, a: u8) -> SignalValue {
    SignalValue::Object(vec![
        ("r".to_owned(), SignalValue::Number(r as f64)),
        ("g".to_owned(), SignalValue::Number(g as f64)),
        ("b".to_owned(), SignalValue::Number(b as f64)),
        ("a".to_owned(), SignalValue::Number(a as f64)),
    ])
}

pub(super) fn set_rgba_signal_value(value: &mut SignalValue, r: u8, g: u8, b: u8, a: u8) {
    match value {
        SignalValue::Object(fields) if fields.len() == 4 => {
            fields[0].1 = SignalValue::Number(r as f64);
            fields[1].1 = SignalValue::Number(g as f64);
            fields[2].1 = SignalValue::Number(b as f64);
            fields[3].1 = SignalValue::Number(a as f64);
        }
        _ => {
            *value = rgba_signal_value(r, g, b, a);
        }
    }
}

pub(super) fn composite_keyed_id(family_id: &str, key: &str) -> String {
    format!("{family_id}::{key}")
}

pub(super) fn parse_tile_key(key: &str) -> Option<(u32, u32)> {
    let payload = key.strip_prefix("tile-")?;
    let (column, row) = payload.split_once('-')?;
    Some((column.parse().ok()?, row.parse().ok()?))
}

pub(super) fn object_number_field(fields: &[(String, SignalValue)], field: &str) -> Option<f64> {
    fields.iter().find_map(|(name, value)| {
        if name != field {
            return None;
        }
        match value {
            SignalValue::Number(number) => Some(*number),
            _ => None,
        }
    })
}

pub(super) fn rewrite_keyed_expr(expr: &Expr, reads: &[RecipeFamilyReadSpec], key: &str) -> Expr {
    match expr {
        Expr::Value { value } => Expr::Value {
            value: value.clone(),
        },
        Expr::Read { id } => {
            let rewritten = reads
                .iter()
                .find_map(|read| match read {
                    RecipeFamilyReadSpec::Signal { .. } => None,
                    RecipeFamilyReadSpec::Keyed { family_id, .. } if family_id == id => {
                        Some(composite_keyed_id(family_id, key))
                    }
                    RecipeFamilyReadSpec::Keyed { .. } => None,
                })
                .unwrap_or_else(|| id.clone());
            Expr::Read { id: rewritten }
        }
        Expr::Get { target, field } => Expr::Get {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            field: field.clone(),
        },
        Expr::At { target, index } => Expr::At {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            index: Box::new(rewrite_keyed_expr(index, reads, key)),
        },
        Expr::First { target } => Expr::First {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Last { target } => Expr::Last {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Slice { target, start, end } => Expr::Slice {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            start: Box::new(rewrite_keyed_expr(start, reads, key)),
            end: end
                .as_ref()
                .map(|value| Box::new(rewrite_keyed_expr(value, reads, key))),
        },
        Expr::Join { target, separator } => Expr::Join {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            separator: Box::new(rewrite_keyed_expr(separator, reads, key)),
        },
        Expr::Flatten { target } => Expr::Flatten {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Object { fields } => Expr::Object {
            fields: fields
                .iter()
                .map(|(name, value)| (name.clone(), rewrite_keyed_expr(value, reads, key)))
                .collect(),
        },
        Expr::Array { items } => Expr::Array {
            items: items
                .iter()
                .map(|item| rewrite_keyed_expr(item, reads, key))
                .collect(),
        },
        Expr::Sum { args } => Expr::Sum {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Multiply { args } => Expr::Multiply {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Concat { args } => Expr::Concat {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Coalesce { args } => Expr::Coalesce {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Length { target } => Expr::Length {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Contains { target, value } => Expr::Contains {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            value: Box::new(rewrite_keyed_expr(value, reads, key)),
        },
        Expr::MergeObjects { args } => Expr::MergeObjects {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Keys { target } => Expr::Keys {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Values { target } => Expr::Values {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::HasField { target, field } => Expr::HasField {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            field: field.clone(),
        },
        Expr::Pick { target, fields } => Expr::Pick {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            fields: fields.clone(),
        },
        Expr::Omit { target, fields } => Expr::Omit {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            fields: fields.clone(),
        },
        Expr::Append { target, value } => Expr::Append {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            value: Box::new(rewrite_keyed_expr(value, reads, key)),
        },
        Expr::Abs { target } => Expr::Abs {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Min { args } => Expr::Min {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Max { args } => Expr::Max {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Sqrt { target } => Expr::Sqrt {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Sin { target } => Expr::Sin {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Cos { target } => Expr::Cos {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Floor { target } => Expr::Floor {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Mod { left, right } => Expr::Mod {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Clamp { value, min, max } => Expr::Clamp {
            value: Box::new(rewrite_keyed_expr(value, reads, key)),
            min: Box::new(rewrite_keyed_expr(min, reads, key)),
            max: Box::new(rewrite_keyed_expr(max, reads, key)),
        },
        Expr::Atan2 { y, x } => Expr::Atan2 {
            y: Box::new(rewrite_keyed_expr(y, reads, key)),
            x: Box::new(rewrite_keyed_expr(x, reads, key)),
        },
        Expr::Subtract { left, right } => Expr::Subtract {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Divide { left, right } => Expr::Divide {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Eq { left, right } => Expr::Eq {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Neq { left, right } => Expr::Neq {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Gt { left, right } => Expr::Gt {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Gte { left, right } => Expr::Gte {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Lt { left, right } => Expr::Lt {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Lte { left, right } => Expr::Lte {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::And { args } => Expr::And {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Or { args } => Expr::Or {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Not { arg } => Expr::Not {
            arg: Box::new(rewrite_keyed_expr(arg, reads, key)),
        },
        Expr::If {
            condition,
            then_expr,
            else_expr,
        } => Expr::If {
            condition: Box::new(rewrite_keyed_expr(condition, reads, key)),
            then_expr: Box::new(rewrite_keyed_expr(then_expr, reads, key)),
            else_expr: Box::new(rewrite_keyed_expr(else_expr, reads, key)),
        },
    }
}
