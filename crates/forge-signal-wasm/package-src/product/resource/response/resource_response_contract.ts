import {
  array,
  collection,
  connection,
  entityStore,
  objectItems,
} from "./resource_collection_response_contract.js";
import {
  map,
} from "./resource_map_collection_response_contract.js";
import {
  multiple,
  named,
} from "./resource_named_collection_response_contract.js";
import {
  grouped,
} from "./resource_grouped_collection_response_contract.js";
import {
  sparse,
} from "./resource_sparse_page_response_contract.js";
import {
  discriminated,
} from "./resource_discriminated_tuple_response_contract.js";
import {
  detail,
  detailRegions,
  detailJsonPaths,
} from "./resource_detail_response_contract.js";
import {
  jsonObjectAspects,
  objectAspects,
} from "./aspects/object_aspect_response_contract.js";
import {
  jsonPathAspects,
} from "./aspects/json_path_aspect_response_contract.js";
import {
  summary,
} from "./resource_summary_response_contract.js";
import {
  tree,
} from "./resource_tree_response_contract.js";

const resourceResponse = Object.freeze({
  array,
  collection,
  connection,
  detail,
  detailRegions,
  detailJsonPaths,
  discriminated,
  entityStore,
  summary,
  objectItems,
  jsonObjectAspects,
  jsonPathAspects,
  grouped,
  map,
  multiple,
  named,
  objectAspects,
  sparse,
  tree,
});

export { resourceResponse };
