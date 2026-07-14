import React from "react";
import type {
  DemoRoleProfile,
  DemoThreeData,
  NavLinkDef,
  RouteAccessResolution,
} from "./demoThreeRouterModel";
import { productForRoute } from "./DemoThreeContentHelpers";

interface DemoThreeModalProps {
  currentRouteId: NavLinkDef["id"];
  data: DemoThreeData;
  role: DemoRoleProfile;
  activeHref: string;
  resolution: RouteAccessResolution | null;
  navigateTo: (link: NavLinkDef) => Promise<void>;
  navLinkForRoute: (model: any, routeId: NavLinkDef["id"], productId?: string) => NavLinkDef;
  routerModel: any;
}

export const DemoThreeModal: React.FC<DemoThreeModalProps> = ({
  currentRouteId,
  data,
  role,
  activeHref,
  navigateTo,
  navLinkForRoute,
  routerModel,
}) => {
  if (currentRouteId !== "addProduct" && currentRouteId !== "editProduct") {
    return null;
  }

  const product = productForRoute(data, activeHref) ?? data.products[0];
  const closeLink =
    currentRouteId === "addProduct"
      ? navLinkForRoute(routerModel, "catalog")
      : navLinkForRoute(routerModel, "product", product.id);
  const titleValue = currentRouteId === "addProduct" ? "Northstar Weekender Duffel" : product.name;
  const typeValue = currentRouteId === "addProduct" ? "Travel bags" : product.productType;
  const statusValue = currentRouteId === "addProduct" ? "Draft" : product.status;
  const descriptionValue =
    currentRouteId === "addProduct"
      ? "Premium weekender built for carry-on travel, everyday commuting, and gift merchandising."
      : product.description;

  return (
    <div className="storefront-sheet-backdrop">
      <div className="storefront-sheet">
        <div className="storefront-sheet-header">
          <div>
            <h2>{currentRouteId === "addProduct" ? "Add product" : "Edit product"}</h2>
            <p>
              {currentRouteId === "addProduct"
                ? "Create a new product without leaving the products index."
                : "Update product details in a routed side sheet without losing page context."}
            </p>
          </div>
          <div className="storefront-sheet-actions">
            <button className="storefront-button" onClick={() => void navigateTo(closeLink)} type="button">
              Cancel
            </button>
            <button className="storefront-button primary" type="button">
              {currentRouteId === "addProduct" ? "Save draft" : "Save"}
            </button>
          </div>
        </div>

        <div className="storefront-sheet-body">
          <div className="storefront-sheet-grid">
            <section className="storefront-sheet-section">
              <h3>Product details</h3>
              <label>
                Title
                <input readOnly value={titleValue} />
              </label>
              <label>
                Description
                <textarea readOnly value={descriptionValue} />
              </label>
              <label>
                Product type
                <input readOnly value={typeValue} />
              </label>
            </section>

            <section className="storefront-sheet-section">
              <h3>Media</h3>
              <div className="storefront-sheet-media">
                <img src={product.imageUrl} alt={product.name} />
              </div>
              <div className="storefront-sheet-placeholder">Add gallery image</div>
            </section>
          </div>

          <div className="storefront-sheet-grid">
            <section className="storefront-sheet-section">
              <h3>Pricing</h3>
              <label>
                Price
                <input readOnly value={currentRouteId === "addProduct" ? "$126.00" : product.price} />
              </label>
              <label>
                Compare-at price
                <input readOnly value={currentRouteId === "addProduct" ? "$148.00" : product.compareAtPrice} />
              </label>
            </section>

            <section className="storefront-sheet-section">
              <h3>Publishing</h3>
              <label>
                Status
                <input readOnly value={statusValue} />
              </label>
              <label>
                Editor
                <input readOnly value={`${role.userName} · ${role.label}`} />
              </label>
            </section>
          </div>
        </div>
      </div>
    </div>
  );
};
