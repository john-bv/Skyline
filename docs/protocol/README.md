# Protocol Documentation

This is the protocol specification for `Skyline v1.0.0`.

!> This code is still in development and is not ready for production use.

There are a few main concepts to understand before reading this document, they are
- [Protocol Documentation](#protocol-documentation)
  - [Offline Handshake](#offline-handshake)
  - [Online Handshake](#online-handshake)

If you're looking for packet information, please refer to the [packet](./PACKETS.md) section.

## Offline Handshake

## Online Handshake
Skyline utilizes a protocol similar to RakNet. All peers are considered offline until they
undergo the [Handshake](#handshake). An offline packet is a packet that
can be sent to a peer without being connected to them. These packets will be responded to
accordingly. Before performing any request to the network you **MUST** identify yourself!

The proxy identification sequence goes as follows:

1. **Identification** <br />
   In order to determine a connection as genuine you must send a [LoginPacket](./PACKETS.md#login-request), this packet must contain a valid `token` and list of valid `identifiers`. In the scenario that the server doesn't require a token, a [Guest Token](#term-guest-token) is also accepted, but be advised that the guest token is limited to the permissions set by the server config.

2. **Connection Response** <br />
   During this step the server will tell inform the client of whether or not it's been verified. If the client is not verified, a `Disconnect` status will be sent within the [LoginResponse](./PACKETS.md#login-response) packet. If the client is verified, a `Success` status will be sent within the [LoginResponse](./PACKETS.md#login-response) packet.

!> All messages sent here are framed and possibly encrypted.