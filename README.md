# T.U.M. - Tale's Usage Monitor

![Build status](https://github.com/lpaulic/tum/actions/workflows/lint-and-build.yml/badge.svg) ![Release status](https://github.com/lpaulic/tum/actions/workflows/release.yml/badge.svg)

## About
This project is done for my dear colleague [@Soki324](https://github.com/Soki324). Purpose of this project is various systemd data and publish data over MQTT.
The name is an acronym, where the only weird part is the 'T' which stands for Tale, a nickname of mine that I am trying to make it stick :).

## Requirements
1. The tool needs to be run on multiple platforms
2. The tool needs to be run on multiple operating systems
3. The tool monitor the following:
   1. CPU usage
   2. GPU usage
   3. RAM usage
   4. Disk storage usage
   5. Number of disk storage
   6. Network statistics
4. The data should be sent over MQTT protocol
5. The data should be encoded in JSON format
7. The tool should have a configuration file that will define:
   1. The transport protocol
   2. The network interface
   3. The network port
   4. Periodic time for publishing the data over network

## Implementation
### Monitored resource
The JSON representation of the data that is reported by this application is as follows:
* `cpu` array that contains objects with following attributes:
  * `id` -> identification number of the CPU, represented with a integer, starts from 0
  * `load` -> floating point representation of the CPU load
* `memory` object that contains the following attributes:
  * `used_bytes` -> memory under use, specified in B
  * `total_bytes` -> maximum available memory, specified in B
* `networks` array that contains objects with following attributes:
  * `interface`: name of the interface, represented with a string
  * `rx_bytes`: number of bytes received
  * `tx_bytes`: number of bytes transmitted
  * `rx_error_bytes`: number of error bytes received
  * `tx_error_bytes`: number of error bytes transmitted
  * `rx_speed_bps`: download speed in B/s
  * `tx_speed_bps`: upload speed in B/s

An example JSON is shown bellow:
```
{
   "cpus": [
      {
         "id": 0,
         "load": 38.5
      },
      {
         "id": 1,
         "load": 42.24
      },
      {
         "id": 2,
         "load": 83.0
      }
   ],
   "memory": {
      "used_bytes": 1000000000,
      "total_bytes": 15000000000
   },
   "networks": [
      {
         "interface": "eth0",
         "rx_bytes": 15000,
         "tx_bytes": 5000,
         "rx_error_bytes": 2,
         "tx_error_bytes": 0,
         "rx_speed_bps": 2455.3,
         "tx_speed_bps": 55.3
      },
      {
         "interface": "eth1",
         "rx_bytes": 51000,
         "tx_bytes": 1000,
         "rx_error_bytes": 21,
         "tx_error_bytes": 10,
         "rx_speed_bps": 65455.3,
         "tx_speed_bps": 3355.3
      }
   ]
}
```

### Publishing data
After X amount of time get the current system resources. Use the MQTT client to publish the system resources to a MQTT broker.
What are the topics that are used for the message:
* `devices` -> root level hierarchy, all other topics go under it
  * `<hostname>` -> each device will publish messages under the `devices/<hostname>` topic, where `<hostname>` is the hostname of the device
    * `system` -> topic for holding all `<hostname>`'s system related messages
      * `stats` -> resources of a device will be published here, i.e.: `devices/<hostname>/system/stats` will hold the `<hostname>`'s system resources

### Application configuration
The T.U.M. application has a configuration file that specifies necessary data for run time. The configuration file is in `YAML` format, and the available configuration options are:
* `server_addr` -> address of the MQTT server to connect to, string value
* `server_port` -> port of the MQTT server to connect to, integer value [0, (2^16)-1]
* `username` -> username that we use to authenticate with the MQTT server we connect to, string value
* `password` -> password that we use to authenticate with the MQTT server we connect to, string value, plaintext
* `monitoring_rate_s` -> delay in seconds between sending new system usage data to the MQTT server, integer value [0, (2^64)-1]

## Building
To build the code run:
```
cargo build
```

## Testing
To run unit tests execute:
```
cargo test
```

To run integration tests:
* start the MQTT broker container, instructions [here](#mqtt-broker-docker-container).
* run a MQTT subscriber client
```
docker exec --interactive --tty tum-mqtt-container mosquitto_sub -p 1883 -u "${USER}" -P "${USER}" -t device/tum-test/system/stats -v
# NOTE: make sure that '-u' and '-P' arguments match the ones you set in docker build command, the 'MQTT_USERNAME' and 'MQTT_USERNAME' respectfully
```

## MQTT Broker Docker container
To build the MQTT broker container execute the following from the repositories root directory:
```
docker build --build-arg MQTT_USERNAME="${USER}" --build-arg MQTT_PASSWORD="${USER}" --force-rm --tag tum-mqtt --file ./docker/mosquitto.dockerfile docker
```

To run the MQTT broker container execute the following:
```
docker run --detach --name tum-mqtt-container --hostname tum-test --rm -p 1883:1883 -p 9001:9001 --volume ./docker/config/mosquitto.conf:/mosquitto/config/mosquitto.conf --volume ./docker/data:/mosquitto/data/ --volume ./docker/log:/mosquitto/log tum-mqtt
# NOTE: 'hostname' must match the one that is going to be used for the pub/sub topic for system statistics
```

To stop the MQTT broker container execute the following:
```
docker stop tum-mqtt-container
```

## Running
The application ca be ran in two ways:
* using cargo:
```
cargo run [-- -c <configuration-file-path>]
```
* invoking the binary directly:
```
<path-to-executable>/tum [-c <configuration-file-path>]
```

The application usage can be printed as follows:
* using cargo:
```
cargo run -- --help | -h
```
* invoking the binary directly:
```
<path-to-executable>/tum --help | -h
```