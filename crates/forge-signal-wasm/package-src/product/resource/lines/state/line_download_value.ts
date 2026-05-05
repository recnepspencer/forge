function createLineDownload(descriptors = Object.freeze([])) {
  const frozenDescriptors = Object.freeze([...descriptors]);
  let readyCount = 0;
  let unavailableCount = 0;
  let incompatibleCount = 0;
  for (const descriptor of frozenDescriptors) {
    if (descriptor.download.kind === "ready") {
      readyCount += 1;
    } else if (descriptor.download.kind === "unavailable") {
      unavailableCount += 1;
    } else {
      incompatibleCount += 1;
    }
  }
  return Object.freeze({
    count: frozenDescriptors.length,
    readyCount,
    unavailableCount,
    incompatibleCount,
    descriptors: frozenDescriptors,
  });
}

export { createLineDownload };
