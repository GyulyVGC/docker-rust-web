FROM rust

WORKDIR /client

COPY . .

RUN cargo install --path .

CMD ["client"]