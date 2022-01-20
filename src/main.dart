import 'dart:io';

void main() async {
  var socket = await WebSocket.connect('ws://localhost:8080');

  socket.listen((event) {
    print('Dit is terug van de websocket server: $event');
  });

  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
  socket.add('Dit is nieuwe data yo!');
}
