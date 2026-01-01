# Socket connection process and options
By default, the server is found on port `3000` and the ***socket server*** is found on `3001`. Unless routed through a reverse proxy (which is recommended strongly), TLS will not be enabled. Assuming both these things are true, you can connect to the server's socket through `ws://example.com:3001` where `example.com` is the ip or dns of your server.

## Authenticating
The first message sent to a socket is assumed to be an authentication challenge, which is a JWT obtained through the rest api's `/api/login` route (see related documentation), it expects this to be sent in plaintext. On an authentication failiure the socket will be automatically closed.

## Using the socket
After authenticating, a socket will recieve all messages for the channel it is currently on, possibilities are:
- `ALL` (recieve all messages from all available channels, used for scanning)
- `NONE` (no messages from any channel, **default**)
- String (any valid string that doesn't exceed the server's maximum size constraints, recieves messages from the relevant channel.)

## Closing
Sockets may be closed at any time by the server for a variety of reasons. Additionally sockets may be closed by the client at any time. **Note:** There may be ungracefull closes on the server side.

# Tracking
The ip of any connection may be tracked by the server
