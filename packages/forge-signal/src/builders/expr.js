export const expr = {
  value(value) {
    return { kind: "value", value };
  },
  read(id) {
    return { kind: "read", id };
  },
  get(target, field) {
    return { kind: "get", target, field };
  },
  at(target, index) {
    return { kind: "at", target, index };
  },
  first(target) {
    return { kind: "first", target };
  },
  last(target) {
    return { kind: "last", target };
  },
  slice(target, start, end) {
    return { kind: "slice", target, start, end };
  },
  join(target, separator) {
    return { kind: "join", target, separator };
  },
  flatten(target) {
    return { kind: "flatten", target };
  },
  object(fields) {
    return { kind: "object", fields };
  },
  array(items) {
    return { kind: "array", items };
  },
  sum(...args) {
    return { kind: "sum", args };
  },
  multiply(...args) {
    return { kind: "multiply", args };
  },
  concat(...args) {
    return { kind: "concat", args };
  },
  coalesce(...args) {
    return { kind: "coalesce", args };
  },
  length(target) {
    return { kind: "length", target };
  },
  contains(target, value) {
    return { kind: "contains", target, value };
  },
  mergeObjects(...args) {
    return { kind: "mergeObjects", args };
  },
  keys(target) {
    return { kind: "keys", target };
  },
  values(target) {
    return { kind: "values", target };
  },
  hasField(target, field) {
    return { kind: "hasField", target, field };
  },
  pick(target, ...fields) {
    return { kind: "pick", target, fields };
  },
  omit(target, ...fields) {
    return { kind: "omit", target, fields };
  },
  append(target, value) {
    return { kind: "append", target, value };
  },
  subtract(left, right) {
    return { kind: "subtract", left, right };
  },
  divide(left, right) {
    return { kind: "divide", left, right };
  },
  eq(left, right) {
    return { kind: "eq", left, right };
  },
  neq(left, right) {
    return { kind: "neq", left, right };
  },
  gt(left, right) {
    return { kind: "gt", left, right };
  },
  gte(left, right) {
    return { kind: "gte", left, right };
  },
  lt(left, right) {
    return { kind: "lt", left, right };
  },
  lte(left, right) {
    return { kind: "lte", left, right };
  },
  and(...args) {
    return { kind: "and", args };
  },
  or(...args) {
    return { kind: "or", args };
  },
  not(arg) {
    return { kind: "not", arg };
  },
  ifElse(condition, thenExpr, elseExpr) {
    return { kind: "if", condition, thenExpr, elseExpr };
  }
};
