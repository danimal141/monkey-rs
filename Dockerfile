FROM rust:1.49-buster

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY . /usr/src/app

RUN rustup component add clippy rustfmt

CMD ["cargo", "run"]
