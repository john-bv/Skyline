# Skyline
A binary based load balancer, and message broker designed to be light-weight and versatile over other solutions like RabbitMQ or Kafka.

This repository contains the following crates:
- [`protocol`](./protocol) - The protocol definitions for Skyline, which is used by both the server and client to communicate with each other.
- [`skyline`](./skyline) - The core logic for Skyline, including the protocol, and the server implementation, as well as the client implementation.
- [`server`](./server) - The Skyline server implementation, which is the primary way to use Skyline.
- [`client`](./client) - A rust client for Skyline, which can be used to connect to a Skyline server.

## Protocol
For protocol information, please refer to the [protocol documentation](./PROTOCOL.md).

The protocol operates over UDP, and is designed to be light-weight and simple to implement.
The reason for this is we can reliably send a message to a Skyline server without having to worry about the message being lost, or the server being down.


## Copy Pasta
Skyline is an extremely versatile message broker, and load balancer. What does this mean? This means that Skyline can be used for a variety of different use cases, and can be used to solve a variety of different problems,
such as:
- You're a user and you want to send a message to a server while its live.
- You want to absolutely guarantee that a message is sent to a server.
- You want to receive updates on something live, no restarts.

To visualize this, let's say you're a player on a Game server, this game server has 3 regions, NA, EU, and AS. You're playing on NA, but your friend is on EU (for some reason)
and you want to send them a message. In vanilla this is impossible, with skyline, you would be able to send the message to EU, informing EU that the message is coming from you,
and your friend would receive the dm.