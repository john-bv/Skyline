## Protocol

This is the protocol specification for `MAgent-v0.0.1`.

### Handshake

Before performing any request to the network you **MUST** identify yourself! The proxy idenfitication sequence goes as follows:

1. **Identification**
   In order to determine a connection as genuine you must send a [ConnectionRequest](#packet-connection-request) with a valid `token` and `identifier` or a list of valid identifiers. In the scenario that the server doesn't require a token, a [Guest Token](#term-guest-token) is also accepted, but be advised that the guest token is limited to the permissions set by the server config.

2. **Connection Response**
   
   During this step the server will prompt the client of whether or not it's been verified.
   
   The server will first send a [ConnectionReply](#packet-connection-reply) packet to the client. This packet will tell the client it's unique ID and the status of the connection. It is here that the server will inform the client that it should disconnect.
   
   > From here on out, all packets will be encrypted and `framed`! The only exception to this rule is the [HeartBeatAck](#packet-heartbeat-ack) packet, and the [Disconnect](packet-disconnect) packet.  
   
   After receiving the [ConnectionReply](#packet-connection-reply) packet, the client must send a heartbeat with the interval specified in the packet.

3. **Connection Accepted**
   
   Once the server receives the first ACK after the connection response, the server will send a [ProxyInformation](#packet-proxy-information) packet. This packet will contain information regarding the server, such as available channels for you, as well as all clients you can communicate with, eg: NA would connect and see that `EU-1`, `EU-2`, and `NA-2` are all connected to the proxy. 

## Packets

1. [LoginPacket](#login-packet)

2. [DisconnectPacket](#disconnect-packet)

3. 
