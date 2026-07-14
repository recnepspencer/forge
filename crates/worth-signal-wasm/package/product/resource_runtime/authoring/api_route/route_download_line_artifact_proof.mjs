import { normalizeRouteLineArtifact } from "./route_line_artifact_proof.mjs";

export function normalizeDownloadLineArtifact(line) {
  return {
    route: normalizeRouteLineArtifact(line),
    download: JSON.parse(JSON.stringify(line.download())),
    downloadDiagnostics: JSON.parse(
      JSON.stringify(line.diagnostics().download),
    ),
  };
}
