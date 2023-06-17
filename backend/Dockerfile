FROM rust:1.67

WORKDIR /backend/chessapp/
COPY . /backend/chessapp/

RUN cargo install --path .

# here instead of this just put the name of of the command you want to run
#ex CMD ["cargo", "run"]
CMD ["ls", "-la"]