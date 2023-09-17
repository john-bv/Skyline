# Protocol Documentation

This is the protocol specification for `Skyline v1.0.0`.

!> This code is still in development and is not ready for production use.

There are a few main concepts to understand before reading this document, they are
- [Protocol Documentation](#protocol-documentation)
  - [Offline Handshake](#offline-handshake)
  - [Online Handshake](#online-handshake)

If you're looking for packet information, please refer to the [packet](/protocol/PACKETS.md) section.

## Offline Handshake
Skyline assumes all peers are offline until they undergo the following handshake. An offline packet a packet designed
to be sent to a peer without being connected to them, meaning packets sent offline aren't really that important.

The offline handshake goes as follows:
1. **Ping** <br />
   The client sends a [PingPacket](/protocol/PACKETS.md#ping) to the server, this packet contains a `timestamp` which is used to calculate the latency between the client and the server. As
   well as a check to see if the server is online.

   ?> *While not required, it's generally good practice to send a ping before connecting to the server.*
2. **Connect Request** <br />
   The client should send a [ConnectRequest](/protocol/PACKETS.md#connect-request) packet to the server. This packet contains a `mtu` as well as a `protocol` version. The `mtu` is used to determine the maximum size of a packet that can be sent to the client. The `protocol` is used to determine if the client is compatible with the server. It is important to note that the `mtu` field is padded
   onto the end of the packet, meaning the packet size will be corelated to the `mtu` field.

## Online Handshake
Skyline utilizes a protocol similar to RakNet. All peers are considered offline until they
undergo the [Handshake](#handshake). An offline packet is a packet that
can be sent to a peer without being connected to them. These packets will be responded to
accordingly. Before performing any request to the network you **MUST** identify yourself!

The proxy identification sequence goes as follows:

1. **Identification** <br />
   In order to determine a connection as genuine you must send a [LoginPacket](/protocol/PACKETS.md#login-request), this packet must contain a valid `token` and list of valid `identifiers`. In the scenario that the server doesn't require a token, a [Guest Token](#term-guest-token) is also accepted, but be advised that the guest token is limited to the permissions set by the server config.

2. **Connection Response** <br />
   During this step the server will tell inform the client of whether or not it's been verified. If the client is not verified, a `Disconnect` status will be sent within the [LoginResponse](/protocol/PACKETS.md#login-response) packet. If the client is verified, a `Success` status will be sent within the [LoginResponse](/protocol/PACKETS.md#login-response) packet.

!> All messages sent here are framed and possibly encrypted.