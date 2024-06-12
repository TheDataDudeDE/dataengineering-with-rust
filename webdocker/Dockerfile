# Stage 1: Build the Rust application
FROM rust:latest as builder
ENV APP webdocker
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

# Stage 2: Create a lightweight image for the application
FROM ubuntu:latest 
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
EXPOSE 8080
CMD ["webdocker"]

