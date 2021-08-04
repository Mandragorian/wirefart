image: server
	podman build -t wirefart_test:v0.0.1 -f images/server/Dockerfile .

server:
	cd wirefart_server; \
	cargo build --release; \
	cd ..
