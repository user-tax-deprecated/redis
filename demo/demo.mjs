#!/usr/bin/env -S  node --es-module-specifier-resolution=node --trace-uncaught --experimental-fetch --expose-gc --unhandled-rejections=strict 

import { Redis, redisConn, serverHostPort, serverCluster } from "..";

const server = serverHostPort("xvc.com", 123);

console.log(Redis.prototype);
