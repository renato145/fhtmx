module.exports = {
  ui: {
    port: 8081,
  },
  files: ["target/debug/actix-example"],
  proxy: {
    target: "http://localhost:8000",
  },
  port: 8080,
  injectChanges: true,
  reloadDelay: 500,
  open: false,
  notify: false,
};
