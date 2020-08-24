# redis-example

## Redis Docker

```bash
docker run -itd \
-v ~/docker/redis/conf/redis.conf:/etc/redis/conf/redis.conf \
-v ~/docker/redis/data:/data \
--name redis \
--restart always \
-p 6379:6379 \
redis redis-server --appendonly yes --requirepass "redis.root"
```

## 参考文档

[Redis-rs Method](https://mitsuhiko.github.io/redis-rs/redis/trait.Commands.html)
