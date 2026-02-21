(function () {
  const script = document.querySelector('script[src*="online-count.js"]');
  const wsUrl =
    (script && script.dataset.ws) ||
    "ws://localhost:3000/ws?role=client";

  function createPopup() {
    const box = document.createElement("div");
    box.id = "online-popup";

    box.innerHTML = `
      <div class="online-header">Online</div>
      <div class="online-count">Loading...</div>
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
      #online-popup .online-count {
        font-size: 22px;
        font-weight: bold;
        margin-top: 4px;
      }
    `;

    document.head.appendChild(style);
    document.body.appendChild(box);

    return box.querySelector(".online-count");
  }

  function init() {
    const countEl = createPopup();
    const ws = new WebSocket(wsUrl);

    ws.onmessage = (event) => {
      countEl.textContent = event.data;
    };

    ws.onerror = () => {
      countEl.textContent = "Error";
    };

    ws.onclose = () => {
      countEl.textContent = "Offline";
    };
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();