module.exports = {
  ui: {
    port: 8081,
  },
  files: ["./src/**/*.rs"],
  proxy: {
    target: "http://localhost:8000",
  },
  port: 8080,
  injectChanges: false,
  reloadDelay: 2000,
};
