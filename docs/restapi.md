# Authentication
## POST `/api/login`
**Description:** Returns a JWT.
Expects an `application/json` Body with:
- "handle": String, 
    - the user's unique handle (minus the @symbol)
- "password": String,
    - the user's password

**Responds with**:
- "token": String
    - a json web token to authenticate with secure routes.
- "error": boolean
    - this (currently will only show if there wasn't an error, but if it is present and not false then the request was successfull)
#### or
- a message explaining what went wrong and how to fix it

# Messages
## POST `/api/messages/{channel name}`
**Description:** Send a message to a specified channel based on the path (see the path above where `{channel name}` is)
Expects a `text/plain` body with the user's message
**Responds with**:
- "error": boolean
    - see note on post `/api/login`
#### or 
- a message explaining what went wrong and how to fix it
