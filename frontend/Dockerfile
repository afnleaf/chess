# Get the latest rust docker image
FROM rust:latest

# Set the working directory
WORKDIR /frontend/app

# Copy the project files
#COPY Cargo.toml Cargo.lock ./
COPY . /frontend/app/

# Build the app
RUN cargo build --release

# Expose the port your Rust web server listens on
EXPOSE 8080

# Define the command to run the server
#CMD ["cargo", "run", "--release"]
CMD ["ls", "-la"]
