(() => {
  const FADE_MS = 1000;
  const FADE_IN_MS = 50;
  const DEFAULT_MILLIS = 3000;
  const DEFAULT_HOVER_MILLIS = 1000;
  const HIDE_CLASS = "fhtmx-toast-hide";
  const STYLE_ID = "fhtmx-toast-style";
  const css = [
    "[data-toast]{",
    "opacity:1;",
    `transition:opacity ${FADE_IN_MS}ms ease-out;`,
    `animation:fhtmx-toast-enter ${FADE_IN_MS}ms ease-out;`,
    "}",
    `[data-toast].${HIDE_CLASS}{opacity:0;transition:opacity ${FADE_MS}ms ease-in}`,
    "@keyframes fhtmx-toast-enter{from{opacity:0}}",
  ].join("");
  const initialized = new WeakSet();

  const injectStyle = () => {
    if (document.getElementById(STYLE_ID) !== null) return;
    const style = document.createElement("style");
    style.id = STYLE_ID;
    style.textContent = css;
    document.head.appendChild(style);
  };

  const setupToast = (el) => {
    if (initialized.has(el)) return;
    initialized.add(el);
    const readMillis = (attr, fallback) => {
      const value = Number.parseInt(el.dataset[attr], 10);
      return Number.isNaN(value) ? fallback : value;
    };
    const millis = readMillis("toastMillis", DEFAULT_MILLIS);
    const hoverMillis = readMillis("toastHoverMillis", DEFAULT_HOVER_MILLIS);
    let fadeTimeout = null;
    let removeTimeout = null;

    const startTimers = (delay) => {
      fadeTimeout = setTimeout(() => {
        el.classList.add(HIDE_CLASS);
      }, delay);
      removeTimeout = setTimeout(() => {
        el.remove();
      }, delay + FADE_MS);
    };
    const cancelTimers = () => {
      clearTimeout(fadeTimeout);
      clearTimeout(removeTimeout);
      fadeTimeout = null;
      removeTimeout = null;
    };

    const pause = () => {
      cancelTimers();
      el.classList.remove(HIDE_CLASS);
    };
    const resume = () => {
      if (fadeTimeout === null) startTimers(hoverMillis);
    };

    startTimers(millis);
    el.addEventListener("pointerenter", pause);
    el.addEventListener("focusin", pause);
    el.addEventListener("pointerleave", resume);
    el.addEventListener("focusout", (e) => {
      if (el.contains(e.relatedTarget)) return;
      resume();
    });
  };

  const scanNode = (node) => {
    if (node.matches?.("[data-toast]")) setupToast(node);
    for (const el of node.querySelectorAll?.("[data-toast]") ?? []) setupToast(el);
  };

  const start = () => {
    injectStyle();
    scanNode(document);
    new MutationObserver((mutations) => {
      for (const mutation of mutations) {
        for (const node of mutation.addedNodes) {
          if (node instanceof Element) scanNode(node);
        }
      }
    }).observe(document.documentElement, { childList: true, subtree: true });
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", start, { once: true });
  } else {
    start();
  }
})();
