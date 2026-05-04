import { requireActiveLine } from "./actions/line_activity_guard.js";
import { invalidateSingleLine } from "./actions/line_invalidate.js";
import { refreshLine } from "./actions/line_refresh.js";
import { revalidateLine } from "./actions/line_revalidate.js";
import { releaseLine } from "./actions/line_release.js";
import { readLineDescriptor } from "./reads/line_descriptor_read.js";
import { readLineDiagnostics } from "./reads/line_diagnostics_read.js";
import { readLineDiagnosticsSummary } from "./reads/line_diagnostics_summary_read.js";
import { readLineFreshness } from "./reads/line_freshness_read.js";
import { readLineHistory } from "./reads/line_history_read.js";
import { readLineProcessing } from "./reads/line_processing_read.js";
import { readLineRequest } from "./reads/line_request_read.js";
import { readLineSignal } from "./reads/line_signal_read.js";
import { readLineStatus } from "./reads/line_status_read.js";
import { readLineUpload } from "./reads/line_upload_read.js";
import { readLineValue } from "./reads/line_value_read.js";
import { createLineView } from "./line_view_factory.js";

function createLineHandle(materialization) {
  return Object.freeze({
    value() {
      requireActiveLine(materialization, "value");
      return readLineValue(materialization);
    },
    signal() {
      requireActiveLine(materialization, "signal");
      return readLineSignal(materialization);
    },
    descriptor() {
      return readLineDescriptor(materialization);
    },
    request() {
      requireActiveLine(materialization, "request");
      return readLineRequest(materialization);
    },
    history() {
      requireActiveLine(materialization, "history");
      return readLineHistory(materialization);
    },
    processing() {
      requireActiveLine(materialization, "processing");
      return readLineProcessing(materialization);
    },
    upload() {
      requireActiveLine(materialization, "upload");
      return readLineUpload(materialization);
    },
    diagnostics() {
      requireActiveLine(materialization, "diagnostics");
      return readLineDiagnostics(materialization);
    },
    diagnosticsSummary() {
      requireActiveLine(materialization, "diagnosticsSummary");
      return readLineDiagnosticsSummary(materialization);
    },
    free() {
      releaseLine(materialization);
    },
    invalidate() {
      requireActiveLine(materialization, "invalidate");
      return invalidateSingleLine(materialization);
    },
    refresh() {
      requireActiveLine(materialization, "refresh");
      return refreshLine(materialization);
    },
    revalidate() {
      requireActiveLine(materialization, "revalidate");
      return revalidateLine(materialization);
    },
    [Symbol.dispose]() {
      releaseLine(materialization);
    },
    status() {
      requireActiveLine(materialization, "status");
      return readLineStatus(materialization);
    },
    freshness() {
      requireActiveLine(materialization, "freshness");
      return readLineFreshness(materialization);
    },
    view(project) {
      requireActiveLine(materialization, "view");
      return createLineView(materialization, project);
    },
  });
}

export { createLineHandle };
