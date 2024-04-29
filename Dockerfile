FROM rust:1.77.1

WORKDIR src
COPY . .

RUN cargo install --path .

ENV DATABASE_URL postgres://felix:abccba123321@db:5432/rir

CMD ["rest_in_rust"]
