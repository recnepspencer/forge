function areLineValuesSemanticallyEqual(left, right) {
  if (Object.is(left, right)) {
    return true;
  }
  if (
    left === null
    || right === null
    || typeof left !== "object"
    || typeof right !== "object"
  ) {
    return false;
  }
  if (Array.isArray(left) || Array.isArray(right)) {
    if (!Array.isArray(left) || !Array.isArray(right) || left.length !== right.length) {
      return false;
    }
    for (let index = 0; index < left.length; index += 1) {
      if (!areLineValuesSemanticallyEqual(left[index], right[index])) {
        return false;
      }
    }
    return true;
  }
  const leftProto = Object.getPrototypeOf(left);
  const rightProto = Object.getPrototypeOf(right);
  if (leftProto !== Object.prototype || rightProto !== Object.prototype) {
    return false;
  }
  const leftKeys = Object.keys(left);
  const rightKeys = Object.keys(right);
  if (leftKeys.length !== rightKeys.length) {
    return false;
  }
  for (const key of leftKeys) {
    if (!Object.hasOwn(right, key)) {
      return false;
    }
    if (!areLineValuesSemanticallyEqual(left[key], right[key])) {
      return false;
    }
  }
  return true;
}

export { areLineValuesSemanticallyEqual };
