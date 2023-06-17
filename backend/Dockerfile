# Get the latest rust docker image
FROM rust:latest

# ARG that we can pass at build time
ARG PORT=8080

# ENV that persists in the build image
ENV PORT $PORT

# Set the working directory
WORKDIR /backend/chessapp/

# Copy the project files
COPY . /backend/chessapp/

# Install the app
# RUN cargo install --path .
RUN cargo build --release

# Define the command to run the server
# here instead of this just put the name of of the command you want to run
CMD ["cargo", "run"]
#CMD ["ls", "-la"]