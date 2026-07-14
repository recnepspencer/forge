import { normalizeRouteLineArtifact } from "./route_line_artifact_proof.mjs";

export function normalizeTransferLineArtifact(line) {
  const requestDiagnostics = JSON.parse(JSON.stringify(line.diagnostics().request));
  delete requestDiagnostics.baseUrl;
  delete requestDiagnostics.target;
  return {
    route: normalizeRouteLineArtifact(line),
    upload: JSON.parse(JSON.stringify(line.upload())),
    processing: JSON.parse(JSON.stringify(line.processing())),
    requestDiagnostics,
  };
}
