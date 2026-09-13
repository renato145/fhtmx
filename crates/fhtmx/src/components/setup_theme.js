(() => {
  const lightTheme = document.currentScript?.dataset.lightTheme || "light";
  const darkTheme = document.currentScript?.dataset.darkTheme || "dark";
  const toggles = "[data-theme-toggle]";

  const isLightMode = () => localStorage.getItem("isLightMode") === "true";

  const themeChange = (isLightMode) => {
    document.documentElement.dataset.theme = isLightMode
      ? lightTheme
      : darkTheme;
  };

  const sync = () => {
    const isLightMode = isLightMode();
    document.querySelectorAll(toggles).forEach((toggle) => {
      toggle.checked = isLightMode;
    });
  };

  const setIsLightMode = (isLightMode) => {
    localStorage.setItem("isLightMode", String(isLightMode));
    themeChange(isLightMode);
    sync();
  };

  document.addEventListener("change", (event) => {
    if (event.target instanceof Element && event.target.matches(toggles)) {
      setIsLightMode(event.target.checked);
    }
  });

  document.addEventListener("DOMContentLoaded", sync);
  document.addEventListener("htmx:afterSettle", sync);

  themeChange(isLightMode());
})();
