import {
  array,
  collection,
  entityStore,
  objectItems,
} from "./resource_collection_response_contract.js";
import {
  detail,
} from "./resource_detail_response_contract.js";
import {
  jsonObjectAspects,
  jsonPathAspects,
  objectAspects,
} from "./resource_object_aspect_response_contract.js";
import {
  summary,
} from "./resource_summary_response_contract.js";

const resourceResponse = Object.freeze({
  array,
  collection,
  detail,
  entityStore,
  summary,
  objectItems,
  jsonObjectAspects,
  jsonPathAspects,
  objectAspects,
});

export { resourceResponse };
