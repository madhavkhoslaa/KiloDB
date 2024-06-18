# KiloDB

A Key Value Database running on redis protocol

## Commands supported 
1. SET
2. GET
3. DEL
4. HSET
5. HGET
6. HDEL
7. LPUSH
8. RPUSH
9. LPOP
10. RPOP
11. SADD
12. SREM
13. SMEMBERS
14. ZADD
15. ZREM
16. ZRANGE

## Design
1. Have a datastructure that can hold pairs of Keys and Datastructure(https://betterprogramming.pub/implementing-a-hashmap-in-rust-35d055b5ac2b)
    1. Implement own hashmap like datastructure that can store(String/String, String/List, String/Set, String/HashMap)
    2. Arc refrence of this datastructure will be kept in threads
    3. Refcell will be used over the values for mutability
    4. Use of mutexes ?
2. Only one instance of this datastructure exists in the lifetime of the program
3. Impls of command traits can execute an alteration on this datastructure
4. Command trait objects can be spawned to multiple threads and alter the data in our datastruct
 
## RoadMap
### Stage 1
1. Be able to connect to redis-cli
2. Have simple hashmap like datastructure (be able to do get/set/del on keys from redis cli)

## Stage 2
1. Be able to connect any language through any redis connector
2. Have support for datastructure
    1. Sorted Lists(logn insertion and logn fetch)
    2. Unsorted Lists(lpush and rpush will be constant time and linear search for fetches)
3. Add tests

## Stage 3
1. AOF backups and AOF writes
2. Have support for datastructure
    1. Sorted Lists(logn insertion and logn fetch)
    2. Unsorted Lists(lpush and rpush will be constant time and linear search for fetches)
## Stage 4
1. Benchmark against redis
2. think about it later

