function requireActiveLine(materialization, methodName) {
  if (materialization.lifecycle.isReleased()) {
    throw new Error(
      `resource line ${methodName}() cannot be used after line.free()`,
    );
  }
}

export { requireActiveLine };
