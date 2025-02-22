# rumqttd

[![crates.io page](https://img.shields.io/crates/v/rumqttd.svg)](https://crates.io/crates/rumqttd)
[![docs.rs page](https://docs.rs/rumqttd/badge.svg)](https://docs.rs/rumqttd)

Rumqttd is a high performance MQTT broker written in Rust. It's light weight and embeddable, meaning
you can use it as a library in your code and extend functionality

## Currently supported features

- MQTT 3.1.1
- QoS 0 and 1
- Retained messages
- Connection via TLS
- Last will
- All MQTT 3.1.1 packets

## Upcoming features

- QoS 2
- Retransmission after reconnect
- MQTT 5


## Getting started

You can directly run the broker by running the binary with a config file with:

```
cargo run --release -- -c demo.toml

```

Example config file is provided on the root of the repo.


### Using Docker

rumqttd can be used with docker by pulling the image from docker hub as follows:
```bash
docker pull bytebeamio/rumqttd
```

To use the rumqttd docker image with the included `demo.toml` while exposing the necessary ports for clients to interact with the broker, use the following command:
```bash
docker run -p 1883:1883 -p 1884:1884 -it bytebeamio/rumqttd -c demo.toml
```

One can also mount the local directory containing configs as a volume and use the appropriate config file as follows:
```bash
docker run -v /path/to/configs:/configs -p 1883:1883 -it bytebeamio/rumqttd -c /configs/config.toml
```

#### Building the docker image

In order to run rumqttd within a docker container, build the image by running `build_docker.sh`. The shell script will build the rumqttd binary file and copy it into the `stage/` directory before building the docker image. You can then run `rumqttd` with the included `demo.toml` as follows:
```bash
./build_docker.sh
docker run -p 1883:1883 -p 1884:1884 -it rumqttd -c demo.toml
```

# How to use with TLS

To connect an MQTT client to rumqttd over TLS, create relevant certificates for the broker and client using [provision](https://github.com/bytebeamio/provision) as follows:
```bash
provision ca // generates ca.cert.pem and ca.key.pem
provision server --ca ca.cert.pem --cakey ca.key.pem --domain localhost // generates localhost.cert.pem and localhost.key.pem
provision client --ca ca.cert.pem --cakey ca.key.pem --device 1 --tenant a // generates 1.cert.pem and 1.key.pem
```

Update config files for rumqttd and rumqttc with the generated certificates:
```toml
[v4.2.tls]
    certpath = "path/to/localhost.cert.pem"
    keypath = "path/to/localhost.key.pem"
    capath = "path/to/ca.cert.pem"
```

You may also use [certgen](https://github.com/minio/certgen), [tls-gen](https://github.com/rabbitmq/tls-gen) or [openssl](https://www.baeldung.com/openssl-self-signed-cert) to generate self-signed certificates, though we recommend using provision.

**NOTE:** Mount the folders containing the generated tls certificates and the proper config file(with absolute paths to the certificate) to enable tls connections with rumqttd running inside docker.
