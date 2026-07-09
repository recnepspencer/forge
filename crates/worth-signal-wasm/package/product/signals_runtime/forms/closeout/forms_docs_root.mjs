import path from "node:path";
import { fileURLToPath } from "node:url";

const closeoutDir = path.dirname(fileURLToPath(import.meta.url));
export const formsCrateRoot = path.resolve(closeoutDir, "../../../../..");

export const formsDocsRoot = path.join(formsCrateRoot, "docs");
