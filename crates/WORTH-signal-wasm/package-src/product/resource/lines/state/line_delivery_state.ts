function createLineDeliveryState() {
  const packetIds = new Set();
  return Object.freeze({
    has(packetId) {
      return packetIds.has(packetId);
    },
    remember(packetId) {
      packetIds.add(packetId);
    },
  });
}

export { createLineDeliveryState };
