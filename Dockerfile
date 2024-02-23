FROM rust:latest

WORKDIR /user/src/app
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["cargo", "run"]