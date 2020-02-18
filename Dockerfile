FROM clux/muslrust AS builder
WORKDIR /usr/src/
RUN USER=root apt-get install libssl-dev

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new dalia-challenge
WORKDIR /usr/src/dalia-challenge
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
COPY templates ./templates
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=builder /root/.cargo/bin/dalia-challenge .
USER 1000
CMD ["./dalia-challenge", "-a", "0.0.0.0", "-p", "8080"]