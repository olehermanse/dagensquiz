FROM docker.io/rust:1.93.0
WORKDIR /dagensquiz
COPY rust-toolchain.toml ./rust-toolchain.toml
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY Rocket.toml ./Rocket.toml
ENV ROCKET_ENV prod
RUN rustup default nightly
RUN mkdir src; touch src/main.rs
RUN cargo rustc --release --package rocket
RUN cargo rustc --release --package rocket_contrib
RUN cargo rustc --release --package handlebars
RUN cargo rustc --release --package rust-crypto
RUN cargo rustc --release --package chrono
COPY src/ ./src/
RUN cargo build --color never --release
COPY quiz/ ./quiz/
COPY templates ./templates/
CMD ["cargo", "run", "--release"]
