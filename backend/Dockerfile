FROM rust:1.57 as builder

WORKDIR /build

COPY ./src/ ./src/
COPY Cargo.* .
COPY ./migrations/ ./migrations/
RUN cargo build --release

RUN mkdir /app && cp target/release/backend /app/

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /build/target/release/backend ./
COPY ./security/ /app/security/
RUN apt update && apt install -y mariadb-client ca-certificates
RUN useradd --system --create-home --home-dir /home/skyrocket --shell /bin/bash skyrocket
RUN chown -R skyrocket /app
USER skyrocket

CMD ["/app/backend"]
