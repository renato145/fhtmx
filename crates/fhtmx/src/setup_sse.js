let sseId = null;

document.body.addEventListener("htmx:sseBeforeMessage", function (e) {
  if (e.detail.type === "sse_id") {
    sseId = e.detail.data;
    e.preventDefault();
  }
});

document.body.addEventListener("htmx:configRequest", function (e) {
  if (sseId) e.detail.parameters["sse_id"] = sseId;
});
