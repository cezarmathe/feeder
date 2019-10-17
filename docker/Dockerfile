FROM alpine

LABEL com.cezarmathe.feeder-author="Armand Cezar Mathe <cezarmathe [at] gmail [dot] com>"
LABEL com.cezarmathe.feeder-version="0.1.0"

WORKDIR /

COPY ./target/release/feeder /feeder
COPY ./config/Rocket.toml /Rocket.toml

ENV DB_HOST="localhost"
ENV DB_PORT=27017

EXPOSE 8000
CMD [ "" ]
ENTRYPOINT [ "/feeder" ]
