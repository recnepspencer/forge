import { readResourceLineHandle } from "./form_sources.js";
import { stableValueDigest } from "../values/value_paths.js";

export function readResourceTransferReport(source) {
  const line = readResourceLineHandle(source);
  if (line === null) {
    return null;
  }
  const summary = line.summary();
  const report = {
    upload: summary.upload,
    processing: summary.processing,
    download: summary.download,
    summary: Object.freeze({
      uploadActive:
        summary.upload.kind === "prepared"
        || summary.upload.kind === "uploaded",
      processingActive:
        summary.processing.kind === "accepted"
        || summary.processing.kind === "processing",
      downloadReadyCount: summary.download.readyCount,
      downloadUnavailableCount: summary.download.unavailableCount,
      downloadIncompatibleCount: summary.download.incompatibleCount,
    }),
  };
  return Object.freeze({
    ...report,
    digest: stableValueDigest(report),
  });
}

export function resourceTransferHasActiveWork(report) {
  return report !== null
    && (
      report.summary.uploadActive
      || report.summary.processingActive
    );
}
