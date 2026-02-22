#!/bin/sh
set -e

PASSWD=/mosquitto/config/passwd

echo "[MITIGATED BROKER] Kreiranje passwd fajla..."
mosquitto_passwd -b -c $PASSWD patient1 pass_patient1
mosquitto_passwd -b    $PASSWD patient2 pass_patient2
mosquitto_passwd -b    $PASSWD monitor  pass_monitor

echo "[MITIGATED BROKER] Passwd kreiran, pokretanje brokera..."
exec mosquitto -c /mosquitto/config/mosquitto.conf