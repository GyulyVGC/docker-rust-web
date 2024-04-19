FROM rust

WORKDIR /server

EXPOSE 7878

COPY . .

RUN cargo install --path .

CMD ["cargo", "run"]