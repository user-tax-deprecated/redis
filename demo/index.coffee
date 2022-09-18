#!/usr/bin/env coffee

import { Redis, redisConn, serverHostPort, serverCluster } from ".."

{ env } = process

{
  REDIS_DB
  REDIS_PASSWORD
  REDIS_CLUSTER
} = env

if REDIS_CLUSTER
  SERVER = serverCluster(
    REDIS_CLUSTER.split('|').map (i)=>
      [host,port] = i.split(':')
      [
        host
        if port then parseInt(port) else 6379
      ]
  )
else
  {
    REDIS_HOST
    REDIS_PORT
  } = env
  SERVER = serverHostPort(
    REDIS_HOST, parseInt REDIS_PORT
  )

redis = await redisConn(
  3
  SERVER
  'default'
  REDIS_PASSWORD
  parseInt REDIS_DB
)

key = '123'

console.log await redis.get key

await redis.set key,'a'
console.log await redis.get key
console.log await redis.getB key
console.log await redis.del key
console.log await redis.get key
console.log await redis.getB key

console.log '--'
await redis.set key, new Uint8Array([97,98])
console.log await redis.get key
console.log await redis.getB key
console.log await redis.mdel [key]
console.log await redis.get key
console.log await redis.getB key

console.log '--'
await redis.set key, Buffer.from([97,98])
console.log await redis.get key
console.log await redis.getB key
console.log await redis.mdel [key]
console.log await redis.get key
console.log await redis.getB key

await redis.set key, '测试'
await redis.expire key,6
console.log await redis.get key
