FROM rust

RUN apt update
RUN apt install -y libpq-dev
RUN apt install -y postgresql

WORKDIR /app

COPY . .
COPY .env.prod .env

#RUN cargo install diesel_cli --no-default-features --features postgres
#RUN diesel setup
#RUN diesel migration run

RUN cargo install --path .
CMD ["rust_api"]
EXPOSE 8080