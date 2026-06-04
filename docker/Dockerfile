FROM debian:bookworm-slim
WORKDIR /app

RUN apt update && apt install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY target/x86_64-unknown-linux-gnu/release/devns /usr/local/bin/devns
COPY docker/config /config

RUN chmod +x /usr/local/bin/devns
ENV PORT=53
ENV DEBUG_MODE=false
ENV MAX_WORKERS=10
ENV MAX_MESSAGES=20
ENV DNS_SERVERS="1.1.1.1;1.0.0.1;8.8.8.8"

EXPOSE 53/udp
EXPOSE 53/tcp
USER root

WORKDIR /
ENTRYPOINT ["/usr/local/bin/devns"]
