# -------- Stage 1: Builder --------
FROM rust:1.88 AS builder

WORKDIR /usr/src/nlytics
COPY . .
RUN cargo build --release

# -------- Stage 2: Runtime --------
FROM debian:bookworm-slim

WORKDIR /usr/src/nlytics

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/nlytics/target/release/nlytics .

EXPOSE 8080
CMD ["./nlytics"]