import { createResourceAuthPosture } from "./auth_posture.js";

const resourceAuth = Object.freeze({
  anonymous() {
    return createResourceAuthPosture("anonymous");
  },
  authenticated() {
    return createResourceAuthPosture("authenticated");
  },
  workspace() {
    return createResourceAuthPosture("workspace");
  },
});

export { resourceAuth };
