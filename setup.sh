ip tuntap add dev tuntest mode tun
ip link set tuntest up
ip addr add 192.168.54.1 dev tuntest
ip route add 192.168.54.0/24 proto kernel dev tuntest
