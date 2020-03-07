FROM rust:1.41.1-buster

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY . /usr/src/app
CMD ["cargo", "run"]
