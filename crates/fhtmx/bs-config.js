module.exports = {
  ui: {
    port: 8081,
  },
  files: ["./examples/*.html"],
  proxy: {
    target: "http://localhost:8000",
  },
  port: 8080,
  injectChanges: true,
  reloadDelay: 50,
  open: false,
};
