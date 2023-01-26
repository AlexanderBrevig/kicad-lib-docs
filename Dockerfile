FROM rust:1.66.1 as build

RUN USER=root cargo new --bin kicad-lib-docs
WORKDIR /kicad-lib-docs

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./resources ./resources
COPY ./src ./src

RUN cargo build --release
RUN cargo test --release

FROM debian:buster-slim

COPY --from=build /kicad-lib-docs/target/release/kicad-lib-docs kicad-lib-docs

CMD ["kicad-lib-docs"]
