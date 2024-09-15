FROM alpine

RUN apk add --no-cache asterisk asterisk-sample-config asterisk-sounds-en bash

COPY ./ari.conf /etc/asterisk/ari.conf
COPY ./asterisk.conf /etc/asterisk/asterisk.conf
COPY ./extensions.conf /etc/asterisk/extensions.conf
COPY ./http.conf /etc/asterisk/http.conf
COPY ./sip.conf /etc/asterisk/sip.conf
COPY ./pjsip.conf /etc/asterisk/pjsip.conf
COPY ./rtp.conf /etc/asterisk/rtp.conf

# SIP
EXPOSE 5060/udp
# RTP
EXPOSE 16384-16394/udp

ENTRYPOINT [ "asterisk", "-cgvvvvvvvvvvvvvvv" ]
