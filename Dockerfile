FROM rust
WORKDIR /app

COPY . .
RUN cargo install --path .
CMD ["rust_api"]
EXPOSE 8080