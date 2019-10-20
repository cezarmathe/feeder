FROM alpine

LABEL com.cezarmathe.feeder-author="Armand Cezar Mathe <cezarmathe [at] gmail [dot] com>"
LABEL com.cezarmathe.feeder-version="1.0.0"

WORKDIR /

COPY feeder .
COPY Rocket.toml .

ENV DB_HOST="localhost"
ENV DB_PORT=27017

EXPOSE 8000
ENTRYPOINT [ "/feeder" ]
