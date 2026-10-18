(() => {
  const lightTheme = document.currentScript?.dataset.lightTheme || "light";
  const darkTheme = document.currentScript?.dataset.darkTheme || "dark";
  const toggles = "[data-theme-toggle]";

  const isLightMode = () => localStorage.getItem("isLightMode") === "true";

  const themeChange = (light) => {
    document.documentElement.dataset.theme = light ? lightTheme : darkTheme;
  };

  const sync = () => {
    const light = isLightMode();
    document.querySelectorAll(toggles).forEach((toggle) => {
      toggle.checked = light;
    });
  };

  const setIsLightMode = (light) => {
    localStorage.setItem("isLightMode", String(light));
    themeChange(light);
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
