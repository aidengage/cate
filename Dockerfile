FROM rust:1.82.0
LABEL authors="aidengage"

RUN apt-get update -y
RUN apt-get install apache2 -y
RUN mkdir /var/www/html/files/

EXPOSE 8000
EXPOSE 8080
EXPOSE 80

WORKDIR /usr/src/app/

COPY src/client ./

RUN cargo build --features server-binary --bin server --release

USER root

ENTRYPOINT ["sh", "-c", "service apache2 start && ./target/release/server"]
