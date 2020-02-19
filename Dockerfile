FROM clux/muslrust AS builder
WORKDIR /usr/src/

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new dalia-challenge
WORKDIR /usr/src/dalia-challenge
COPY Cargo.toml Cargo.lock ./

# Copy the source and build the application.
COPY migrations ./migrations
COPY src ./src
COPY templates ./templates
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM alpine:3.11
RUN apk add sqlite
COPY --from=builder /root/.cargo/bin/dalia-challenge .
COPY images ./images
CMD ["./dalia-challenge", "-a", "0.0.0.0", "-p", "8080"]