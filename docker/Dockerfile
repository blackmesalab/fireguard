# User container image
FROM debian:testing-slim
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN echo "I am running on $BUILDPLATFORM, building for $TARGETPLATFORM"
RUN mkdir -p /etc/fireguard /etc/wireguard && \
    apt-get update && \
    apt-get install -y git && \
    apt-get -y clean && \
    rm -rf /var/lib/apt/lists/*

COPY ./$TARGETPLATFORM/fireguard /usr/bin/fireguard

CMD ["/usr/bin/fireguard"]
