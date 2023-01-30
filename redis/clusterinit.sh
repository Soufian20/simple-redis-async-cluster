#!/bin/sh
sleep 5

echo "yes" | redis-cli --cluster create \
  redisNode1:6379 \
  redisNode2:6379 \
  redisNode3:6379 \
  redisNode4:6379 \
  redisNode5:6379 \
  redisNode6:6379 \
  --cluster-replicas 1 && echo "Redis cluster initialised"