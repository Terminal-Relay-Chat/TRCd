# TERMINAL RELAY CHAT DAEMON
The server program that runs TRC. Inspired by IRC, this program aims to fix certain problems of modern chats. It aims to be as open source and extensible as the ancient but still valued IRC.

This project is under construction.

# Running
the `JWT_SECRET` environment variable **must** be set for functionality, the easiest way to do this is
```
JWT_SECRET=">>your_secret_here (any password)<<" cargo run
```

# Docker 
> This will require manual setup, I am not a docker wizard.
make a copy of the compose-example.yaml and edit it to meet your goals.
run `docker compose up -d`

