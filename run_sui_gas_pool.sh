#!/bin/bash

# Re-export sui_fullnode_url to ensure it's available to all subprocesses
export SUI_FULLNODE_URL

# Start haproxy in background
/usr/sbin/haproxy -f /etc/haproxy/haproxy.cfg

# Start promtail in background
/usr/local/bin/promtail -config.file=/etc/promtail/config.yml &

# Start sui gas pool
/usr/local/bin/sui_gas_pool --config-path /usr/local/bin/mainnet.yaml