FROM rust:1.81.0
WORKDIR /dagensquiz
COPY . .
ENV ROCKET_ENV prod
RUN rustup default nightly
RUN cargo build --color never --release
CMD ["cargo", "run", "--release"]
