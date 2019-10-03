FROM alpine

LABEL com.cezarmathe.feeder-author="Armand Cezar Mathe <cezarmathe [at] gmail [dot] com>"
LABEL com.cezarmathe.feeder-version="0.1.0"

WORKDIR /

RUN curl -Ssl -o /bin/feeder https://github.com/cezarmathe/feeder/releases/0.1.0/feeder-amd64

ENV DB_HOST="localhost"
ENV DB_PORT=27017

EXPOSE 8000
ENTRYPOINT [ "/bin/feeder" ]
CMD [ "" ]
