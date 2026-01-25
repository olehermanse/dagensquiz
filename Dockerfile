FROM docker.io/rust:1.93.0
WORKDIR /dagensquiz
COPY . .
ENV ROCKET_ENV prod
RUN rustup default nightly
RUN cargo build --color never --release
CMD ["cargo", "run", "--release"]
