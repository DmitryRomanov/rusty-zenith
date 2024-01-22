FROM rust:1.75 as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin rusty-zenith
WORKDIR /rusty-zenith

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/rusty-zenith*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /rusty-zenith/target/release/rusty-zenith .

# set the startup command to run your binary
CMD ["./rusty-zenith"]
