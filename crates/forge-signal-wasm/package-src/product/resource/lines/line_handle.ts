import { requireActiveLine } from "./actions/line_activity_guard.js";
import { invalidateSingleLine } from "./actions/line_invalidate.js";
import { refreshLine } from "./actions/line_refresh.js";
import { revalidateLine } from "./actions/line_revalidate.js";
import { releaseLine } from "./actions/line_release.js";
import { readLineDescriptor } from "./reads/line_descriptor_read.js";
import { readLineDownload } from "./reads/line_download_read.js";
import { readLineDiagnostics } from "./reads/line_diagnostics_read.js";
import { readLineDiagnosticsSummary } from "./reads/line_diagnostics_summary_read.js";
import { readLineFreshness } from "./reads/line_freshness_read.js";
import { readLineHistory } from "./reads/line_history_read.js";
import { readLineProcessing } from "./reads/line_processing_read.js";
import { readLineRequest } from "./reads/line_request_read.js";
import { readLineSignal } from "./reads/line_signal_read.js";
import { readLineStatus } from "./reads/line_status_read.js";
import { readLineSummary } from "./reads/line_summary_read.js";
import { readLineUpload } from "./reads/line_upload_read.js";
import { readLineValue } from "./reads/line_value_read.js";
import { createLineView } from "./line_view_factory.js";
import { requireCurrentMaterialization } from "./state/line_handle_helpers.js";

function createLineHandle(lineBacking) {
  return Object.freeze({
    value() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "value");
      return readLineValue(materialization);
    },
    signal() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "signal");
      return readLineSignal(materialization);
    },
    descriptor() {
      const materialization = requireCurrentMaterialization(lineBacking);
      return readLineDescriptor(materialization);
    },
    request() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "request");
      return readLineRequest(materialization);
    },
    summary() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "summary");
      return readLineSummary(materialization);
    },
    download() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "download");
      return readLineDownload(materialization);
    },
    history() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "history");
      return readLineHistory(materialization);
    },
    processing() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "processing");
      return readLineProcessing(materialization);
    },
    upload() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "upload");
      return readLineUpload(materialization);
    },
    diagnostics() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "diagnostics");
      return readLineDiagnostics(materialization);
    },
    diagnosticsSummary() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "diagnosticsSummary");
      return readLineDiagnosticsSummary(materialization);
    },
    mutationResponse() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "mutationResponse");
      const diagnostics = materialization.binding.diagnosticsSignal();
      return "lastMutationResponsePlan" in diagnostics
        ? diagnostics.lastMutationResponsePlan
        : null;
    },
    free() {
      releaseLine(requireCurrentMaterialization(lineBacking));
    },
    invalidate() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "invalidate");
      return invalidateSingleLine(materialization);
    },
    refresh() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "refresh");
      return refreshLine(materialization);
    },
    revalidate() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "revalidate");
      return revalidateLine(materialization);
    },
    [Symbol.dispose]() {
      releaseLine(requireCurrentMaterialization(lineBacking));
    },
    status() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "status");
      return readLineStatus(materialization);
    },
    freshness() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "freshness");
      return readLineFreshness(materialization);
    },
    view(project) {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "view");
      return createLineView(lineBacking, project);
    },
  });
}

export { createLineHandle };
