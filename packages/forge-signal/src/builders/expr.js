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
  abs(target) {
    return { kind: "abs", target };
  },
  min(...args) {
    return { kind: "min", args };
  },
  max(...args) {
    return { kind: "max", args };
  },
  sqrt(target) {
    return { kind: "sqrt", target };
  },
  sin(target) {
    return { kind: "sin", target };
  },
  cos(target) {
    return { kind: "cos", target };
  },
  floor(target) {
    return { kind: "floor", target };
  },
  mod(left, right) {
    return { kind: "mod", left, right };
  },
  clamp(value, min, max) {
    return { kind: "clamp", value, min, max };
  },
  atan2(y, x) {
    return { kind: "atan2", y, x };
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
