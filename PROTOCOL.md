## Protocol

This is the protocol specification for `SysComm-v0.0.1`.

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


```php
// newer
<?php

use zeqa\dispatcher\Dispatcher;
use zeqa\dispatcher\Client;
use zeqa\dispatcher\api\ApiLayer;
use zeqa\dispatcher\api\ApiPacket;
use zeqa\dispatcher\channel\Channel;
use zeqa\dispatcher\channel\ChannelPermission;

// create a new client
$client = new Client("NAME", [ "auth" => "TOKEN" ]);

// connect to the server
$client->connect();

// after identifying the client will have a list of channels it can join
// as well as a list of available clients
// each channel is a `Channel` class, but can have it's own underlying API.
// A channel should not be confused with PUB/SUB channels, as they are not the same.
// We can request to join with the following:
$exampleChannelId = 10;
$channel = $client->channel($exampleChannelId)->join(ChannelPermission::BROADCAST | ChannelPermission::RECEIVE);

// Some channels have an API layer, an api layer is a list of packets that can be sent to the channel.
// The API layer is defined by the server so you can't just send any packet to any channel.
$channel->send($yourPacketId, [ "some" => "data" ]);
// or
$myPacket = new ApiPacket($yourPacketId, $exampleChannelId);
$myPacket->addField("some", "data"); // $myPacket->addField($idOrName, $value);
$channel->send($myPacket);

// we can also subscribe to a channel, this will allow us to receive packets from the channel
$channel->subscribe(function($packet, $apiPacket) {
    // $packet is the raw packet data
    // $apiPacket is the packet data parsed by the api layer
    // we can get props on the api packet, like:
    if ($apiPacket->id === $channel->api()->getPacketId("some packet")) {
        // do something with the packet
        $data = $apiPacket->getPayload();
        $data->some; // data
    }
});


// Here's an example of how a database packet might work.
// keep in mind the server MUST send this to the client in it's dictionary
// or the client will not be able to decode it.
class DbPacket extends ApiPacket {
    public string $query;
    public array $params;

    public function __construct($query, $params = []) {
        // The Packet Id, Owning Channel ID
        parent::__construct(ApiLayer::ID_BY_DICT("query"), ApiLayer::FOR_CHANNEL("db"));
        $this->query = $query;
        $this->params = $params;
    }

    public static function query(string $query, array $params = []) {
        return new DbPacket($query, $params);
    }
}

// Now our apilayer is aware of the "db-query" packet, and will
// decode/encode it to the `DbPacket` class instead of the default `ApiPacket` class.
ApiLayer::registerPacket(DbPacket::class);

// now on our client we can do:
// - send a packet to the client, the library automatically
//   finds the channel id and packet id for us.
// - listen to the channel for this specific packet
$db = $client->channel(ApiLayer::CHANNEL("db"))->join(ChannelPermission::REQUEST_PRIVATE);

$db->send(DbPacket::query("SELECT * FROM `users` WHERE `username` = ?", ["Bavfalcon9"]), function ($response) {
    // safely validate the packet, without using second param in func
    $packet = ApiLayer::toPacket($response);

    // because we registered the DbPacket class, we can safely cast the packet
    // to the DbPacket class, and access it's properties.
    if ($packet instanceof DbPacket) {
        // do something with the packet
        $packet->query;
        $packet->params;
    }

    // However responses wont typically be a query for a query (that'd be stupid)
    // so heres a better example on a annonymous packet as a response sent by the server
    $packet = ApiLayer::toApiPacket($response);

    if ($packet->id === ApiLayer::ID_BY_DICT("user")) {
        // do something with the packet
        $packet->getPayload();
    }
});
```

