export function readControlAvailabilities(availabilityReport) {
  return Object.freeze(
    availabilityReport.artifacts.filter((artifact) => artifact.scope === "control"),
  );
}

export function readControlAvailability(availabilityReport, controlId) {
  return readControlAvailabilities(availabilityReport).find((artifact) => artifact.ownerId === controlId) ?? null;
}
