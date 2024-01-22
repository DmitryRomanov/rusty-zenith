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

# Build only the dependencies to cache them
RUN cargo build --release --target ${TARGETARCH}-unknown-linux-gnu
RUN rm src/*.rs

# Now that the dependency is built, copy your source code
COPY ./src ./src

# Build for release
RUN rm ./target/${TARGETARCH}-unknown-linux-gnu/release/rusty-zenith*
RUN cargo build --release --target ${TARGETARCH}-unknown-linux-gnu

# Define the final base image
FROM debian:buster-slim

# Copy the build artifact from the build stage
ARG TARGETARCH
COPY --from=build /rusty-zenith/target/${TARGETARCH}-unknown-linux-gnu/release/rusty-zenith .

# Set the startup command to run your binary
CMD ["./rusty-zenith"]
