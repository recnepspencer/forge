import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import { getLandingCarouselPosition } from "../src/ui/landingCarouselPosition.ts";

test("mobile carousel keeps both neighboring cards in a visible 3D stage", () => {
  const previous = getLandingCarouselPosition({ narrow: true, reducedMotion: false, relative: -1 });
  const active = getLandingCarouselPosition({ narrow: true, reducedMotion: false, relative: 0 });
  const next = getLandingCarouselPosition({ narrow: true, reducedMotion: false, relative: 1 });

  assert.equal(previous.x, "-72%");
  assert.equal(previous.rotateY, 58);
  assert.equal(previous.opacity, 0.68);
  assert.equal(active.x, "0%");
  assert.equal(active.opacity, 1);
  assert.equal(next.x, "72%");
  assert.equal(next.rotateY, -58);
  assert.equal(next.opacity, 0.68);
});

test("desktop carousel retains its established depth positions", () => {
  const previous = getLandingCarouselPosition({ narrow: false, reducedMotion: false, relative: -1 });
  const next = getLandingCarouselPosition({ narrow: false, reducedMotion: false, relative: 1 });

  assert.equal(previous.x, "-42rem");
  assert.equal(previous.rotateY, 68);
  assert.equal(next.x, "42rem");
  assert.equal(next.rotateY, -68);
});

test("reduced motion exposes only the active carousel card", () => {
  const active = getLandingCarouselPosition({ narrow: true, reducedMotion: true, relative: 0 });
  const inactive = getLandingCarouselPosition({ narrow: true, reducedMotion: true, relative: 1 });

  assert.equal(active.opacity, 1);
  assert.equal(active.pointerEvents, "auto");
  assert.equal(inactive.opacity, 0);
  assert.equal(inactive.pointerEvents, "none");
});

test("mobile controls, swipe, and inactive-card accessibility share active-index authority", async () => {
  const page = await readFile(new URL("../src/ui/LandingPage.tsx", import.meta.url), "utf8");
  const css = await readFile(new URL("../src/ui/landingMobileCarousel.css", import.meta.url), "utf8");

  assert.match(page, /onPanEnd=\{handleCarouselPanEnd\}/u);
  assert.match(page, /aria-hidden=\{relative !== 0\}/u);
  assert.match(page, /inert=\{relative !== 0 \? true : undefined\}/u);
  assert.match(page, /onClick=\{\(\) => goTo\(index\)\}/u);
  assert.match(css, /\.xai-capability-carousel \{[^}]*width: calc\(100% - 5rem\);/su);
  assert.match(css, /\.xai-carousel-window \{[^}]*perspective: 1050px;/su);
  assert.match(css, /\.xai-carousel-arrow \{[^}]*top: auto;[^}]*bottom: 0\.25rem;/su);
  assert.match(css, /\.xai-carousel-arrow-left \{ left: calc\(50% - 7rem\); \}/u);
  assert.match(css, /\.xai-carousel-arrow-right \{ right: calc\(50% - 7rem\); \}/u);
  assert.match(css, /\.xai-carousel-pagination \{[^}]*bottom: 1\.3rem;/su);
});
