# Endpoints

- /company_voice
    - GET /:company_name
- /spotlight
    - POST /
    - DELETE /

# company_voice

## GET /:company_name

Gets the list of users in a voice chat

### Sucess
```json
{
    "users": [ { "discord_id": 777892521262841908, "image": "https://cdn.discordapp.com/avatars/193043741676797952/a_df491624579c46ef8fc38b9d2ef8cd68.gif?size=1024", "name":"CeSIUM" } ]
}
```

### Error

A nice 404 status code

# /spotlight

## POST /

```json
{
    "company": "CeSIUM"
}
```
### Sucess

A nice 200 status code

### Error

A nice 404 status code

## DELETE /

### Sucess

A nice 200 status code

### Error

A nice 404 status code
