FROM rust


RUN apt update
RUN apt install -y libpq-dev
RUN apt install -y postgresql

WORKDIR /app

COPY . .
COPY .env.prod .env

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --path .

CMD bash -c "diesel setup && diesel migration run && rust_api"