const express = require("express");
const http = require("http");
const WebSocket = require("ws");
const url = require("url");

const app = express();
const server = http.createServer(app);
const wss = new WebSocket.Server({ server });

let clientCount = 0;
const adminClients = new Set();

// 🔥 ส่งค่าล่าสุดให้ admin ทุกคน
function broadcastCount() {
  const value = String(clientCount);
  for (const ws of adminClients) {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(value);
    }
  }
}

wss.on("connection", (ws, req) => {
  const parsed = url.parse(req.url, true);
  const role = parsed.query.role || "client";
  const isAdmin = role === "admin";

  if (isAdmin) {
    // 🔥 ส่ง snapshot ทันที
    ws.send(String(clientCount));
    adminClients.add(ws);
  } else {
    clientCount++;
    console.log("client connected ->", clientCount);
    broadcastCount();
  }

  ws.on("close", () => {
    if (isAdmin) {
      adminClients.delete(ws);
    } else {
      clientCount--;
      console.log("client disconnected ->", clientCount);
      broadcastCount();
    }
  });

  ws.on("error", () => {
    ws.close();
  });
});

server.listen(3000, () => {
  console.log("Server running at http://localhost:3000");
});
