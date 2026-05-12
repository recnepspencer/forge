function requireCurrentMaterialization(lineBacking) {
  const materialization = lineBacking.current();
  if (materialization === null) {
    throw new Error(
      "resource line backing is unavailable because the line has no current materialization",
    );
  }
  return materialization;
}

export { requireCurrentMaterialization };
