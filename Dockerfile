# Define the base image
FROM rust:1.75 as build

# Define the build argument for the target architecture
ARG TARGETARCH

# Create a new empty shell project
RUN USER=root cargo new --bin rusty-zenith
WORKDIR /rusty-zenith

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN case "$TARGETARCH" in \
  "amd64") echo x86_64-unknown-linux-gnu > /rust_target.txt ;; \
  "arm64") echo aarch64-unknown-linux-gnu > /rust_target.txt ;; \
  *) echo ${TARGETARCH}-unknown-linux-gnu > /rust_target.txt ;; \
esac

# Build only the dependencies to cache them
RUN rustup target add $(cat /rust_target.txt)
RUN cargo build --release --target $(cat /rust_target.txt)
RUN rm src/*.rs

# Now that the dependency is built, copy your source code
COPY ./src ./src

# Build for release
RUN rm ./target/$(cat /rust_target.txt)/release/rusty-zenith*
RUN cargo build --release --target $(cat /rust_target.txt)

# Define the final base image
FROM debian:buster-slim

# Copy the build artifact from the build stage
ARG TARGETARCH
COPY --from=build /rusty-zenith/target/$(cat /rust_target.txt)/release/rusty-zenith .

# Set the startup command to run your binary
CMD ["./rusty-zenith"]
