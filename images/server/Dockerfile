FROM docker.io/library/archlinux

RUN pacman -Syyu --noconfirm && \
    pacman -S ipython --noconfirm

COPY wirefart_server/target/release/wirefart_server /wirefart_server
COPY setup.sh /setup.sh
COPY teardown.sh /teardown.sh

RUN chmod +x /setup.sh
RUN chmod +x /teardown.sh

ENTRYPOINT ["/bin/bash"]

