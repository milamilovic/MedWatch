#!/bin/bash

set -e

echo "Initializing secure RabbitMQ broker..."

until rabbitmqctl status > /dev/null 2>&1; do
  echo "Waiting for RabbitMQ to start..."
  sleep 2
done

echo "RabbitMQ is ready"

# Create IoT user with secure password
rabbitmqctl add_user iot_device "SecurePassword!2026" 2>/dev/null || true

# set permissions
rabbitmqctl set_permissions -p / iot_device "" "health_data_queue_secure" ""

# delete guest user
rabbitmqctl delete_user guest 2>/dev/null || true

echo "Secure users created with restricted permissions"
echo "Initialization complete"