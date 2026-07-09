function readLineBasisHistory(lifecycle) {
  const advances = [];
  let advanceCount = 0;
  let currentBasisId = null;
  let lastAdvanceFromId = null;
  let lastAdvanceToId = null;

  for (const entry of lifecycle) {
    currentBasisId = entry.currentBasisId;
    if (entry.basisAdvanceCount <= advanceCount) {
      continue;
    }
    advanceCount = entry.basisAdvanceCount;
    lastAdvanceFromId = entry.lastBasisAdvanceFromId;
    lastAdvanceToId = entry.lastBasisAdvanceToId;
    advances.push(Object.freeze({
      sequence: entry.sequence,
      event: entry.event,
      operation: entry.lastOperation,
      deliveryKind: entry.lastDeliveryKind,
      deliveryScope: entry.lastDeliveryScope,
      deliveryPacketId: entry.lastDeliveryPacketId,
      deliveryBasisId: entry.lastDeliveryBasisId,
      fromBasisId: entry.lastBasisAdvanceFromId,
      toBasisId: entry.lastBasisAdvanceToId,
      currentBasisId: entry.currentBasisId,
    }));
  }

  return Object.freeze({
    currentBasisId,
    advanceCount,
    lastAdvanceFromId,
    lastAdvanceToId,
    advances: Object.freeze(advances),
  });
}

export { readLineBasisHistory };
