# T.U.M. - Tale's Usage Monitor

## About
This project is done for my dear colleague Matej Sokolec. Purpose of this project is various systemd data and publish data over MQTT.
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

<!-- TODO: ## Building -->
<!-- TODO: ## Testing -->
<!-- TODO: ## Benchmark -->