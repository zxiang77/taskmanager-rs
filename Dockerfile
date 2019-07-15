FROM rust:1.36
WORKDIR /usr/src/taskmanager_rs
COPY . .

RUN cargo install --path .
CMD ["taskmanager_rs"]
