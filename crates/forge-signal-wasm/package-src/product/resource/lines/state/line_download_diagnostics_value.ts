function createDownloadDiagnostics(download) {
  return Object.freeze({
    count: download.count,
    readyCount: download.readyCount,
    unavailableCount: download.unavailableCount,
    incompatibleCount: download.incompatibleCount,
    descriptors: Object.freeze(
      download.descriptors.map((descriptor) =>
        Object.freeze({
          kind: descriptor.kind,
          id: descriptor.id,
          label: descriptor.label,
          fileName: descriptor.fileName,
          mediaType: descriptor.mediaType,
          byteLength: descriptor.byteLength,
          download: descriptor.download.kind === "ready"
            ? Object.freeze({
                kind: "ready",
                url: descriptor.download.url,
                method: descriptor.download.method,
                headerNames: Object.freeze(
                  Object.keys(descriptor.download.headers).sort(),
                ),
                expiresAt: descriptor.download.expiresAt,
              })
            : descriptor.download,
        })
      ),
    ),
  });
}

export { createDownloadDiagnostics };
