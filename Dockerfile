FROM docker.io/rust:1.93.0
WORKDIR /dagensquiz
COPY src/ ./src/
COPY rust-toolchain.toml ./rust-toolchain.toml
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY Rocket.toml ./Rocket.toml
ENV ROCKET_ENV prod
RUN rustup default nightly
RUN cargo build --color never --release
COPY quiz/ ./quiz/
COPY templates ./templates/
CMD ["cargo", "run", "--release"]
