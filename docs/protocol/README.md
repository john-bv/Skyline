# Skyline Protocol
Skyline supports both [udp](/udp/README.md) and [tcp](/tcp/README.md) protocols. While both have their own caveats, they both work in tandem to provide a seamless experience.

The basis for each protocol is the same, where you have an sub-protocol that manages the raw payload and makes sure skyline packets are sent and received correctly. The sub-protocol is responsible for handling the raw data, while the main protocol is responsible for handling the packets.

Consider the following diagram:
![Protocol Example](/../images/base-diagram.png)

Breakdown:
1. `Raw Payload` in the diagram is the raw data that is sent over the network. This is something you'd see in wireshark if you were sniffing the packets.
2. `Message`: The protocol wrapper, this manages things like messaging, encryption, and compression. In skyline this is the raw [tcp](/tcp/README.md) or [udp](/udp/README.md) protocol implementation implemented in binary.
3. `SkyLine Packet`: The actual packet that is sent over the network, this is what you would interact with if you chose to send packets to a client. However this is generally advised against.

!> Currently Skyline only uses `tcp` due to the `udp` protocol being in development. <br />
   This will change in the future.