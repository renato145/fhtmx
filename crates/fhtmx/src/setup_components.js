document.addEventListener("alpine:init", () => {
  Alpine.data("toast", (millis = 3000, afterPointerMillis = 1000) => ({
    show: true,
    fadeTimeout: null,
    timeout: null,

    init() {
      this.fadeTimeout = setTimeout(() => {
        this.show = false;
      }, millis);
      this.timeout = setTimeout(() => {
        this.$el.remove();
      }, millis + 1000);
      this.$el.addEventListener("pointerenter", () => {
        clearTimeout(this.fadeTimeout);
        clearTimeout(this.timeout);
        this.fadeTimeout = null;
        this.timeout = null;
        this.show = true;
      });
      this.$el.addEventListener("pointerleave", () => {
        if (this.fadeTimeout === null) {
          this.fadeTimeout = setTimeout(() => {
            this.show = false;
          }, afterPointerMillis);
        }
        if (this.timeout === null) {
          this.timeout = setTimeout(() => {
            this.$el.remove();
          }, afterPointerMillis + 1000);
        }
      });
    },

    destroy() {
      clearTimeout(this.fadeTimeout);
      clearTimeout(this.timeout);
    },
  }));
});
