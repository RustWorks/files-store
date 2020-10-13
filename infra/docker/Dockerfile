FROM ubuntu:20.04

RUN apt-get update && apt-get install -y ca-certificates openssl libssl-dev libpq-dev

COPY files-store-server /files-store/files-store-server

WORKDIR /

EXPOSE 2000

CMD /files-store/files-store-server
