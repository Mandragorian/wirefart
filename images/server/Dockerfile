FROM docker.io/library/archlinux

RUN pacman -Syyu --noconfirm && \
    pacman -S ipython --noconfirm

COPY setup.sh /setup.sh
COPY teardown.sh /teardown.sh

RUN chmod +x /setup.sh
RUN chmod +x /teardown.sh

COPY wirefart_server/target/release/wirefart_server /wirefart_server

ENTRYPOINT ["/bin/bash"]

