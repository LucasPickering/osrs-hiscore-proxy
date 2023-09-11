# OSRS Hiscore API

A (very) minimal HTTP API for accessing Old School RuneScape player hiscores. This is simply a proxy to Jagex's API.

## Why?

The advantages of this API are:

- Responses is structured, easy-to-use JSON instead of cryptic CSV
- Responses have `Access-Control-Allow-Origin: *` set, to allow requests from within the browser ([see CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS))
  - This is entirely safe because the API is 100% public

This API was built to support the [OSRS CLI](https://github.com/LucasPickering/osrs-cli/).

## Usage

The API has a singular route: `/hiscore/:player`

Any minigame/boss that the hiscore does not provide data for will be omitted from the output. In rare cases, Jagex may also omit some skill data for a user, in which case that will be omitted in the response as well. So while in _most_ cases, all skills will be present, don't rely on that fact!!

`GET /hiscore/lynx%20titan`

```json
{
  "skills": [
    {
      "name": "Total",
      "rank": 1,
      "level": 2277,
      "xp": 4600000000
    },
    {
      "name": "Attack",
      "rank": 15,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Defence",
      "rank": 28,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Strength",
      "rank": 18,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Hitpoints",
      "rank": 7,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Ranged",
      "rank": 8,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Prayer",
      "rank": 11,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Magic",
      "rank": 32,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Cooking",
      "rank": 159,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Woodcutting",
      "rank": 15,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Fletching",
      "rank": 12,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Fishing",
      "rank": 9,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Firemaking",
      "rank": 48,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Crafting",
      "rank": 4,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Smithing",
      "rank": 3,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Mining",
      "rank": 25,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Herblore",
      "rank": 5,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Agility",
      "rank": 23,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Thieving",
      "rank": 12,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Slayer",
      "rank": 2,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Farming",
      "rank": 19,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Runecrafting",
      "rank": 7,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Hunter",
      "rank": 4,
      "level": 99,
      "xp": 200000000
    },
    {
      "name": "Construction",
      "rank": 4,
      "level": 99,
      "xp": 200000000
    }
  ],
  "minigames": [
    {
      "name": "Clue Scroll (All)",
      "rank": 552278,
      "score": 22
    },
    {
      "name": "Clue Scroll (Hard)",
      "rank": 375830,
      "score": 22
    },
    {
      "name": "TzTok-Jad",
      "rank": 121,
      "score": 186
    }
  ]
}
```

## Development

The only tool you need to install is [rustup](https://rustup.rs/). After that, simply run `cargo run` in the repo and everything you need will be installed automatically.

You'll need to manually stop and restart the server after making changes. To watch for changes, install [cargo-watch](https://github.com/watchexec/cargo-watch) and start with `cargo watch -x run`.

## Deployment

For production, the API is built into a docker image, published, and deployed via Helm. Kubernetes cluster is provided by [Keskne](https://github.com/LucasPickering/keskne).

To deploy/re-deploy:

```sh
cd deploy
./deploy.sh
```
