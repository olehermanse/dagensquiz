FROM rust:1.34
WORKDIR /dagensquiz
COPY . .
ENV ROCKET_ENV prod
RUN rustup default nightly
RUN cargo build --color never --release
CMD ["cargo", "run", "--release"]
