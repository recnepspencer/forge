function createSearchNamespace() {
  return Object.freeze({
    optional: Object.freeze({
      string() {
        return createSearchField("string", false);
      },
      number() {
        return createSearchField("number", false);
      },
      boolean() {
        return createSearchField("boolean", false);
      },
    }),
    required: Object.freeze({
      string() {
        return createSearchField("string", true);
      },
      number() {
        return createSearchField("number", true);
      },
      boolean() {
        return createSearchField("boolean", true);
      },
    }),
  });
}

function createHashNamespace() {
  return Object.freeze({
    string() {
      return Object.freeze({
        family: "routerHashField",
        valueKind: "string",
      });
    },
  });
}

function createSearchField(valueKind, required) {
  return Object.freeze({
    family: "routerSearchField",
    valueKind,
    required,
  });
}

function isSearchField(value) {
  return value && value.family === "routerSearchField";
}

function isHashField(value) {
  return value && value.family === "routerHashField";
}

export {
  createHashNamespace,
  createSearchNamespace,
  isHashField,
  isSearchField,
};
