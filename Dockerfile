FROM rust:1.36
WORKDIR /code
COPY . .

RUN cargo install --path .
CMD ["cargo", "run"]
