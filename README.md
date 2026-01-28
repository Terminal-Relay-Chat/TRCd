# TERMINAL RELAY CHAT DAEMON
The server program that runs TRC. Inspired by IRC, this program aims to fix certain problems of modern chats. It aims to be as open source and extensible as the ancient but still valued IRC.

This project is under construction.

# Running
the `JWT_SECRET` environment variable **must** be set for functionality, the easiest way to do this is
```
JWT_SECRET=">>your_secret_here (any password)<<" cargo run
```

# Docker 
> This will require manual setup, I am not a docker wizard. Here are some basic instructions:
Make sure you have docker installed and have permission to use it!
1. Make a copy of the compose-example.yaml and edit it to meet your goals. `cp compose-example.yaml compose.yaml`
2. Make a `.env` file containing a jwt secret. `echo "type in a secret phrase (no spaces!)"; read && echo "JWT_SECRET=$REPLY" > ./.env`
3. run `docker compose up -d` (run as a background process)

