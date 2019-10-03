FROM rustlang/rust:nightly

MAINTAINER Armand Cezar Mathe <cezarmathe [at] gmail [dot] com>

LABEL com.cezarmathe.feeder-version="0.1.0"

RUN mkdir feeder
WORKDIR /feeder

RUN curl -Ssl -o feeder https://github.com/cezarmathe/feeder/releases/0.1.0/feeder-amd64

ENV LOG_LEVEL="WARN"
ENV DB_HOST="localhost"
ENV DB_PORT=27017

EXPOSE 8000
ENTRYPOINT /feeder
