FROM rust:slim AS build
ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /usr/src/binserve
COPY . .

RUN apt-get update && apt-get install -yf make pkg-config
RUN cargo build --release && \
    install -Dsvm755 ./target/release/binserve ./bin/binserve

FROM gcr.io/distroless/cc:latest AS final
WORKDIR /app
COPY --from=build /usr/src/binserve/bin /app

ENTRYPOINT [ "/app/binserve" ]
