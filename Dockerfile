FROM rust:latest as cargo-build
RUN apt-get update
RUN apt-get install cmake libssl-dev libolm-dev -y
WORKDIR /usr/src

RUN USER=root cargo new random_bru_bot
WORKDIR /usr/src/random_bru_bot
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --path .

FROM ubuntu:latest
RUN apt-get update
RUN apt-get install libolm3 openssl -y
  
#RUN addgroup -g 1000 random_bru_bot
#RUN adduser -D -s /bin/sh -u 1000 -G random_bru_bot random_bru_bot

WORKDIR /home/random_bru_bot/bin/
COPY --from=cargo-build /usr/local/cargo/bin/random_bru_bot .
RUN chmod +x random_bru_bot
#RUN chown random_bru_bot:random_bru_bot random_bru_bot

#USER random_bru_bot

ENTRYPOINT ["./random_bru_bot"]
