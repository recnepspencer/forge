function createLineHistoryState() {
  let nextSequence = 0;
  const entries = [];
  return Object.freeze({
    append(entry) {
      nextSequence += 1;
      entries.push(
        Object.freeze({
          ...entry,
          sequence: nextSequence,
        }),
      );
    },
    entries() {
      return Object.freeze([...entries]);
    },
  });
}

export { createLineHistoryState };
