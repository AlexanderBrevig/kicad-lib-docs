FROM rust:1.66.1 as build

RUN USER=root cargo new --bin kicad-lib-docgen
WORKDIR /kicad-lib-docgen

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN ls ./target/release/deps
RUN rm ./target/release/deps/kicad_lib_docgen*
RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /kicad-lib-docgen/target/release/kicad-lib-docgen kicad-lib-docgen

CMD ["kicad-lib-docgen"]
