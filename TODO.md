### Interface
1) Implement DI
2) Possibility to convert data to schema (like in FastStream msg: MsgSchema)

### Runner
1) Graceful shutdown
2) Consumer groups
3) Don't spawn task on each message - group N messages on single task

### App
1) Create lib from project
2) Add CLI support and run with options (for example amount of workers)
