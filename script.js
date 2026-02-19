import ws from 'k6/ws';

export default function () {
  ws.connect("ws://localhost:3000/ws?role=client", function (socket) {
    socket.on('open', function () {});
    socket.setTimeout(function () {
      socket.close();
    }, 60000);
  });
}
