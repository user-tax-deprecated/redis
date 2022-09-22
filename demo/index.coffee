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

key = 'test'
await redis.del key

console.log await redis.exist key
console.log await redis.exist [key,key]
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
await redis.expire key,16
console.log await redis.get key

await redis.del key
await redis.sadd key,['1','3']
await redis.sadd key,'2'
await redis.sadd key,'1'
await redis.expire key,16

###
console.log await redis.fnload '''#!lua name=test
redis.register_function(
  'myfunc',
  function(keys, args)
    return tonumber(keys[1])
  end
)'''

console.log await redis.fi64('myfunc',['34'],['66'])
console.log await redis.fstr('myfunc',['331'])
###
