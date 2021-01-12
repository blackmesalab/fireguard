# User container image
FROM alpine:latest
RUN mkdir -p /etc/fireguard /etc/wireguard && \
        apk update && \
        apk add bash iptables ca-certificates \
        wireguard-tools dnsmasq 

COPY fireguard /usr/bin/fireguard
COPY boringtun /usr/bin/boringtun
COPY entrypoint.sh /entrypoint.sh

CMD ["/entrypoint.sh"]
