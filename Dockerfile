# Define the base image
FROM rust:1.75.0-alpine3.18 as build

RUN apk add --no-cache ca-certificates \
  musl-dev \
  libressl-dev && \
  rm -rf /var/cache/apk/*

# Define the build argument for the target architecture
ARG TARGETARCH

# Create a new empty shell project
RUN USER=root cargo new --bin rusty-zenith
WORKDIR /rusty-zenith

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN case "$TARGETARCH" in \
  "amd64") echo x86_64-unknown-linux-musl > /rust_target.txt ;; \
  "arm64") echo aarch64-unknown-linux-musl > /rust_target.txt ;; \
  *) echo ${TARGETARCH}-unknown-linux-musl > /rust_target.txt ;; \
esac

# Build only the dependencies to cache them
RUN rustup target add $(cat /rust_target.txt)
# RUN cargo build --release --target $(cat /rust_target.txt)

# Now that the dependency is built, copy your source code

# Build for release
# RUN rm ./target/$(cat /rust_target.txt)/release/rusty-zenith*
RUN cargo build --release --target $(cat /rust_target.txt)
RUN cp /rusty-zenith/target/$(cat /rust_target.txt)/release/rusty-zenith /rusty-zenith/rusty-zenith
# RUN rm src/*.rs

# Define the final base image
FROM alpine:3.18

# Copy the build artifact from the build stage
COPY --from=build /rusty-zenith/rusty-zenith .

# Set the startup command to run your binary
CMD ["./rusty-zenith"]
