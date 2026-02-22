#!/bin/sh
set -e

PASSWD=/mosquitto/config/passwd

echo "[BROKER] Kreiranje passwd fajla..."

# Stara verzija 1.4.10 zahteva odvojene komande
touch $PASSWD
mosquitto_passwd -b $PASSWD patient1 pass_patient1
mosquitto_passwd -b $PASSWD patient2 pass_patient2
mosquitto_passwd -b $PASSWD monitor  pass_monitor
mosquitto_passwd -b $PASSWD '#'      wildcard_pass

echo "[BROKER] Passwd kreiran, pokretanje brokera..."
exec mosquitto -c /mosquitto/config/mosquitto.conf