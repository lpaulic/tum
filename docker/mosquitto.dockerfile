FROM eclipse-mosquitto

# PUBLIC arguments
ARG MQTT_USERNAME
ARG MQTT_PASSWORD

# PRIVATE arguments
# NOTE: _MOSQUITO_DIR is prepared in eclipse-mosquitto base image
ARG _MOSQUITO_DIR="/mosquitto"
ARG _HOME_DIR="/home/$MQTT_USERNAME"
ARG _MQTT_PASSWD_FILE="$_MOSQUITO_DIR/config/passwd"

RUN mosquitto_passwd -b -c "$_MQTT_PASSWD_FILE" $MQTT_USERNAME $MQTT_PASSWORD

RUN addgroup \
    "$MQTT_USERNAME"
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "$_HOME_DIR" \
    --ingroup "$MQTT_USERNAME" \
    "$MQTT_USERNAME"

RUN chown -R "$MQTT_USERNAME":"$MQTT_USERNAME" "$_MOSQUITO_DIR"
USER "$MQTT_USERNAME"

# NOTE:
#   - things inherited from the  eclipse-mosquitto image:
#       - WORKDIR
#       - USER
#       - ENTRYPOINT
#       - CMD