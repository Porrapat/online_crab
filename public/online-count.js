(function () {
  const script = document.querySelector('script[src*="online-count.js"]');
  const wsUrl =
    (script && script.dataset.ws) ||
    "ws://localhost:3000/ws?role=client";

  function init() {
    const span = document.createElement("span");
    span.textContent = "Loading...";
    script.parentNode.insertBefore(span, script);

    const ws = new WebSocket(wsUrl);

    ws.onmessage = (event) => {
      span.textContent = event.data;
    };

    ws.onerror = () => {
      span.textContent = "Error";
    };

    ws.onclose = () => {
      span.textContent = "Disconnected";
    };
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();