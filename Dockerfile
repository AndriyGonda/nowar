FROM rust:1.59

WORKDIR /nowar/
COPY . .
RUN cargo build --release
CMD ["/nowar/target/release/nowar"]