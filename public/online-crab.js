(function () {
  const script = document.querySelector('script[src*="online-crab.js"]');
  const wsUrl =
    (script && script.dataset.ws) ||
    "ws://localhost:3000/ws?role=client";

  const hidden = script && script.dataset.hidden === "true";

  let ws = null;
  let reconnectDelay = 1000;
  const maxDelay = 30000;
  let reconnectTimer = null;
  let countEl = null;

  function createPopup() {
    const box = document.createElement("div");
    box.id = "online-popup";

    box.innerHTML = `
      <div class="online-header">Online</div>
      <div class="online-crab">Loading...</div>
    `;

    const style = document.createElement("style");
    style.innerHTML = `
      #online-popup {
        position: fixed;
        bottom: 20px;
        right: 20px;
        width: 120px;
        background: #018822;
        color: #fff;
        border-radius: 12px;
        padding: 12px;
        font-family: Arial, sans-serif;
        text-align: center;
        box-shadow: 0 8px 25px rgba(51, 255, 0, 0.9);
        z-index: 99999;
      }
      #online-popup .online-header {
        font-size: 12px;
        opacity: 0.7;
      }
      #online-popup .online-crab {
        font-size: 22px;
        font-weight: bold;
        margin-top: 4px;
      }
    `;

    document.head.appendChild(style);
    document.body.appendChild(box);

    return box.querySelector(".online-crab");
  }

  function scheduleReconnect() {
    if (reconnectTimer) return;

    reconnectTimer = setTimeout(() => {
      reconnectTimer = null;
      reconnectDelay = Math.min(reconnectDelay * 2, maxDelay);
      connect();

    }, reconnectDelay);
  }

  function connect() {
    if (ws && ws.readyState === WebSocket.OPEN) return;

    ws = new WebSocket(wsUrl);

    ws.onopen = () => {
      reconnectDelay = 1000;
      if (countEl) countEl.textContent = "Connected";
    };

    ws.onmessage = (event) => {
      if (countEl) countEl.textContent = event.data;
    };

    ws.onerror = () => {
      ws.close();
    };

    ws.onclose = () => {
      if (countEl) countEl.textContent = "Offline";
      scheduleReconnect();
    };
  }

  function init() {
    if (!hidden) {
      countEl = createPopup();
    }
    connect();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();