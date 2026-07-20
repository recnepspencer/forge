import React from "react";

const narrowCarouselQuery = "(max-width: 720px)";

function readNarrowCarouselViewport() {
  return typeof window !== "undefined" && window.matchMedia(narrowCarouselQuery).matches;
}

function subscribeToNarrowCarouselViewport(notify: () => void) {
  const media = window.matchMedia(narrowCarouselQuery);
  media.addEventListener("change", notify);
  return () => media.removeEventListener("change", notify);
}

export function useLandingCarouselViewport() {
  return React.useSyncExternalStore(
    subscribeToNarrowCarouselViewport,
    readNarrowCarouselViewport,
    () => false,
  );
}
