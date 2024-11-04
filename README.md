# rust-monero-handshake

This program sends a handshake request to a Monero seed node, and parses messages received in response to this request.

The seed node addresses for each network are hard-coded into the Monero repository.


## Monero message protocol

The Monero data exchange protocol is described in these files:

https://github.com/monero-project/monero/blob/master/docs/LEVIN_PROTOCOL.md
https://github.com/monero-project/monero/blob/master/docs/PORTABLE_STORAGE.md

However, there are some mistakes in the documentation. The correct protocol can be derived from the C++ source code itself.

The header of each TCP message can be directly deserialized into a Rust object, while the body is encoded with a more complicated 'Portable Storage' protocol. I have written a simple deserializer for this, which parses the incoming byte streams into Rust objects.

The program allows the user to select which Monero network to use (main/test/stage). It then sends a handshake command to a seed node, and process the incoming response messages for around 2-3 seconds before exiting.

The main handshake response message contains a list of 250 Monero peer nodes, along with some other information about the network. The program prints the first 10 of these addresses, to demonstrate that the TCP data stream was parsed correctly.

## Compilation instructions

Run these commands in the root directory of the repository:

```
docker build -t rust-monero-handshake .
docker run -it rust-monero-handshake
```