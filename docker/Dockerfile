# User container image
FROM debian:testing-slim
ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN echo "I am running on $BUILDPLATFORM, building for $TARGETPLATFORM"
RUN mkdir -p /etc/fireguard /etc/wireguard

COPY ./$TARGETPLATFORM/fireguard /usr/bin/fireguard

CMD ["/usr/bin/fireguard"]
