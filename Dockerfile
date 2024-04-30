# Use the official Rust image as a base
FROM rust:latest as builder

# Set the working directory in the container
WORKDIR /app

# Copy the Rust code to the working directory
COPY . .

# Build the Rust application
RUN cargo build --release

# Start a new stage from a smaller base image
FROM debian:buster-slim

# Set the working directory in the container
WORKDIR /app

# Copy the built binary from the previous stage to this stage
COPY --from=builder /app/target/release/actix-crud .

# Expose port 3000
EXPOSE 3000

# Command to run the application
CMD ["./actix-crud"]
