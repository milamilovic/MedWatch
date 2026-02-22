FROM eclipse-mosquitto:2.0.18

COPY mosquitto.conf /mosquitto/config/mosquitto.conf
COPY acl.conf /mosquitto/config/acl.conf
COPY entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]