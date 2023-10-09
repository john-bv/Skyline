# Skyline
A binary based message broker designed to be light-weight and versatile over other solutions like RabbitMQ or Kafka.

## Protocol
For protocol information, please refer to the [protocol documentation](./PROTOCOL.md).

The protocol operates over UDP, and is designed to be light-weight and simple to implement.
The reason for this is we can reliably send a message to a Skyline server without having to worry about the message being lost, or the server being down.