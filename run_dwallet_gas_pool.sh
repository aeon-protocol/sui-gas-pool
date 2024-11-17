#!/bin/bash

# Start haproxy in background
/usr/sbin/haproxy -f /etc/haproxy/haproxy.cfg

# Start promtail in background
/usr/local/bin/promtail -config.file=/etc/promtail/config.yml &

# Start dwallet gas pool
/usr/local/bin/dwallet_gas_pool --config-path /usr/local/bin/testnet-prod.yaml