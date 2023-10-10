### STAGE 1: install cargo chef
FROM rust AS chef
RUN cargo install cargo-chef
WORKDIR /app

### STAGE 2: create cargo chef recipe.json
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

### STAGE 3: build dependencies from recipe.json and build the app
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin quotes

### STAGE 4: run the app
# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/quotes app
# RUN chmod +x ./app
ENTRYPOINT [ "./app" ]