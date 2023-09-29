# medist

My implement of redis in rust.

The strategy of develop this repo is implying function first, and then optimize or refactor.

## Todo

### BASE PERIOD

- [ ] base client and server, support base commands (GET, SET...)
- [ ] support the five base value data type (string, list, hash, set and zset)
- [ ] abstract the file event and time event, optimize the server to be event-driven
- [ ] data persistence (RDB, AOF)
