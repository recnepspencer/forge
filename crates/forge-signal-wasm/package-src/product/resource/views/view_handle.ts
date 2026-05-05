function createResourceViewHandle(signal) {
  let released = false;
  const viewHandle = function viewHandle() {
    requireActiveView(released);
    return signal();
  };
  Object.defineProperties(viewHandle, {
    id: {
      enumerable: true,
      get() {
        return signal.id;
      },
    },
    debugName: {
      enumerable: true,
      get() {
        return signal.debugName;
      },
    },
    get: {
      enumerable: false,
      value() {
        requireActiveView(released);
        return signal.get();
      },
    },
    peek: {
      enumerable: false,
      value() {
        requireActiveView(released);
        return typeof signal.peek === "function" ? signal.peek() : signal();
      },
    },
    free: {
      enumerable: false,
      value() {
        if (released) {
          return;
        }
        released = true;
        signal.free();
      },
    },
    [Symbol.dispose]: {
      enumerable: false,
      value() {
        if (released) {
          return;
        }
        released = true;
        if (typeof signal[Symbol.dispose] === "function") {
          signal[Symbol.dispose]();
          return;
        }
        signal.free();
      },
    },
  });
  return viewHandle;
}

function requireActiveView(released) {
  if (!released) {
    return;
  }
  throw new TypeError(
    "resource line view cannot be used after line.free()",
  );
}

export { createResourceViewHandle };
