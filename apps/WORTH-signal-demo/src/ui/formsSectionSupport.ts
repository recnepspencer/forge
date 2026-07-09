export interface RolloutDraft {
  productName: string;
  baseCost: number;
  price: number;
  targetMargin: number;
  shippingRegions: string[];
  carrierEmails: Record<string, string>;
}

export interface CountryOption {
  code: string;
  label: string;
  approved: boolean;
}

export const COUNTRY_OPTIONS: CountryOption[] = [
  { code: "AU", label: "Australia", approved: false },
  { code: "CA", label: "Canada", approved: true },
  { code: "DE", label: "Germany", approved: false },
  { code: "JP", label: "Japan", approved: false },
  { code: "GB", label: "United Kingdom", approved: true },
  { code: "US", label: "United States", approved: true },
];

export const SOURCE_DRAFT: RolloutDraft = {
  productName: "Northstar Carry-On",
  baseCost: 96,
  price: 120,
  targetMargin: 25,
  shippingRegions: ["US"],
  carrierEmails: {
    US: "us-ops@carrier.test",
    CA: "",
    GB: "",
    AU: "",
    DE: "",
    JP: "",
  },
};

export const currency = new Intl.NumberFormat("en-US", {
  style: "currency",
  currency: "USD",
  maximumFractionDigits: 0,
});

const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

export function toRoundedMargin(baseCost: number, price: number): number {
  if (price <= 0) {
    return 0;
  }

  return Math.round(((price - baseCost) / price) * 100);
}

export function getRequiredPrice(baseCost: number, targetMargin: number): number {
  if (targetMargin >= 100) {
    return Number.POSITIVE_INFINITY;
  }

  return Math.ceil(baseCost / (1 - targetMargin / 100));
}

export function validatePrice(price: number, baseCost: number, targetMargin: number): string | null {
  if (!Number.isFinite(price) || price <= 0) {
    return "Enter a valid retail price.";
  }

  const requiredPrice = getRequiredPrice(baseCost, targetMargin);
  if (price < requiredPrice) {
    return `Raise price to at least ${currency.format(requiredPrice)} to hold a ${targetMargin}% margin.`;
  }

  return null;
}

export function validateShippingSelection(selected: string[]): string | null {
  const missingApproved = COUNTRY_OPTIONS.filter(
    (option) => option.approved && !selected.includes(option.code),
  );
  if (missingApproved.length > 0) {
    return `Select every approved shipping region: ${missingApproved
      .map((option) => option.label)
      .join(", ")}.`;
  }

  const blocked = COUNTRY_OPTIONS.find((option) => selected.includes(option.code) && !option.approved);
  if (!blocked) {
    return null;
  }

  return `${blocked.label} does not have regulatory approval yet.`;
}

export function validateCarrierEmail(region: string, selected: string[], emails: Record<string, string>): string | null {
  if (!selected.includes(region)) {
    return null;
  }

  const value = emails[region] ?? "";
  if (!value.trim()) {
    return `Add a carrier email for ${region}.`;
  }

  if (!emailPattern.test(value.trim())) {
    return `Enter a valid carrier email for ${region}.`;
  }

  return null;
}

export function formatDraft(draft: RolloutDraft): string {
  return [
    `productName: "${draft.productName}"`,
    `baseCost: ${currency.format(draft.baseCost)}`,
    `price: ${currency.format(draft.price)}`,
    `targetMargin: ${draft.targetMargin}%`,
    `shippingRegions: [${draft.shippingRegions.join(", ")}]`,
  ].join("\n");
}

export function formatMessages(messages: string[]): string {
  return messages.length > 0 ? messages.join("\n") : "No visible errors";
}

export function selectedRegionSummary(selected: string[]): string {
  if (selected.length === 0) {
    return "Select shipping regions";
  }

  return selected.join(", ");
}
