import { resourceBinaryDescriptor } from "../../resource/downloads/resource_binary_descriptor.js";
import { resourceDownload } from "../../resource/downloads/resource_download.js";

const apiRouteDownloadsBuilder = Object.freeze({
  ready(options) {
    return resourceDownload.ready({
      method: "GET",
      ...options,
    });
  },
  multipart(options) {
    return resourceDownload.multipart(options);
  },
  unavailable(options) {
    return resourceDownload.unavailable(options);
  },
  incompatible(options) {
    return resourceDownload.incompatible(options);
  },
  file(id, options) {
    return resourceBinaryDescriptor.file({
      id,
      ...options,
    });
  },
  media(id, options) {
    return resourceBinaryDescriptor.media({
      id,
      ...options,
    });
  },
  export(id, options) {
    return resourceBinaryDescriptor.export({
      id,
      ...options,
    });
  },
});

export { apiRouteDownloadsBuilder };
