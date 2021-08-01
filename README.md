# Wirefart

An exploration of tun devices, using rust.


## Docker

To build the images run

```
docker build -t wirefart_server:v0.0.1 -f images/server/Dockerfile .
```

And to run an image do

```
docker run -it -v /dev/net/tun:/dev/net/tun --cap-add=NET_ADMIN  --entrypoint /bin/bash localhost/wirefart_server:v0.0.1
```

## Demo

Run the `setup.sh` script in order to create the tun device.
Then start the demo by running the `wirefart_server` executable.

Ping some address inside the 192.168.54.0/24 prefix (but not 192.168.54.1).
The executable will print the raw IP packets. If you are using docker, run the
ping inside the container by using `docker exec`.
