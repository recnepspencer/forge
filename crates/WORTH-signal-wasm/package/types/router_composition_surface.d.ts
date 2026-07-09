import type {
  ControllerContract,
} from "./controller_surface.js";
import type {
  PublishedSignalGraph,
} from "./graph_surface.js";
import type {
  RouteResourceMap,
} from "./router_resource_surface.js";

export type RouteControllerMap = Record<string, ControllerContract>;
export type RouteGraphMap = Record<string, PublishedSignalGraph>;

export interface RouterRouteCompositionOptions<
  TControllers extends RouteControllerMap = Record<string, never>,
  TGraphs extends RouteGraphMap = Record<string, never>,
  TResources extends RouteResourceMap = Record<string, never>,
> {
  controllers?: TControllers;
  graphs?: TGraphs;
  resources?: TResources;
}
