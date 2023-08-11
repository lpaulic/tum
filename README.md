# T.U.M. - Tale's Usage Monitor

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
<!-- TODO: ## Benchmark -->
