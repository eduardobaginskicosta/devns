FROM scratch
COPY target/x86_64-unknown-linux-musl/release/devns /devns
COPY config /config

ENV PORT=53
ENV DEBUG_MODE=false
ENV MAX_WORKERS=10
ENV MAX_MESSAGES=20
ENV DNS_SERVERS="1.1.1.1;1.0.0.1;8.8.8.8"

EXPOSE 53/udp
EXPOSE 53/tcp

ENTRYPOINT ["/devns"]
