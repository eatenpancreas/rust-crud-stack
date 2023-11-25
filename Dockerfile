FROM rust:1.72.0
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

COPY /api .

RUN cargo install --path .
CMD ["myapp"]