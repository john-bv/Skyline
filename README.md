# Skyline
A binary based message broker designed to be light-weight and versatile over other solutions like RabbitMQ or Kafka.

This repository contains the following crates:
- [`protocol`](./protocol) - The protocol definitions for Skyline, which is used by both the server and client to communicate with each other.
- [`skyline`](./skyline) - The core logic for Skyline, including the protocol, and the server implementation, as well as the client implementation.
- [`server`](./server) - The Skyline server implementation, which is the primary way to use Skyline.
- [`client`](./client) - A rust client for Skyline, which can be used to connect to a Skyline server.

## Protocol
For protocol information, please refer to the [protocol documentation](./PROTOCOL.md).

The protocol operates over UDP, and is designed to be light-weight and simple to implement.
The reason for this is we can reliably send a message to a Skyline server without having to worry about the message being lost, or the server being down.