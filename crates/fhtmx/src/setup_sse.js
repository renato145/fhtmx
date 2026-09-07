let sseId = null;

document.body.addEventListener("htmx:sse:before:message", function (e) {
  if (e.detail.message.event === "sse_id") {
    sseId = e.detail.message.data;
  }
});

document.body.addEventListener("htmx:config:request", function (e) {
  if (sseId) e.detail.ctx.request.body.set("sse_id", sseId);
});
