# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the project dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the entire project directory to the container
COPY . .

# Compile the project
RUN cargo build --release

# Specify the command to run your application
CMD ["target/release/fps_sfx"]
LABEL authors="Darel"
