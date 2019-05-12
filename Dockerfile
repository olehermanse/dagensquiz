FROM rust:1.28

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80

WORKDIR /dagensquiz
COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]
