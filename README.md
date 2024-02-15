# anime-schedule

Keep track of anime shows you're watching using a Google Calendar.

Special thanks to:
- [Anilist.co](https://anilist.gitbook.io/anilist-apiv2-docs/)'s GraphQL API
- Google Calendar API

## Requirements
- Access to the [Google Calendar API](https://developers.google.com/calendar/api/guides/overview) and a [service account](https://developers.google.com/identity/protocols/oauth2/service-account)
- A non-GSuite Google Account that owns a calendar
  - Create a calendar and note the calendar ID (present in the calendar's settings)
  - Invite the service account by email to make changes to the calendar. A service account can be invited like a regular user.

## Setup

1) Create a `docker-compose.override.yml` at the root of this project like this:
```yaml
version: "3"
services:
  worker:
    env_file:
      - .env
  
  add_show:
    env_file:
      - .env
```
> Note: The `find` service does not need access to google credentials as it only talks to Anilist.

2) Add an `.env` file with `CALENDAR_ID` (set to your calendar's ID)
3) According to the service account docs above, create a key for your service, and store it inside `worker/service-account-key.json`.
>  Note: This can be any filename. By default, it is `service-account-key.json`, but this can be customized by setting the environment variable `GOOGLE_APPLICATION_CREDENTIALS` inside `.env`. The file _must_ however live inside the `worker/` directory.

## Usage

### Build the containers

```bash
docker-compose build
```

### List all shows currently airing, and get the ids you're interested in

```bash
docker-compose run find
```

Example output:
```
Fetching airing shows...
Currently watching shows:
[✅] [154587] Episode 23 - Frieren: Beyond Journey’s End
...
Other shows:
[❌] [161645] Episode 19 - The Apothecary Diaries
[❌] [151970] Episode 19 - Shangri-La Frontier
...
```
### Keep track of the shows you're interested in

```bash
docker-compose run add_show <id...>
# Example: docker-compose run add_show 161645
```


Example output:
```
Adding `The Apothecary Diaries` to calendar `<calendar name>`...
✅ Event added for `The Apothecary Diaries`
```

### Update the shows you're keeping track of
```bash
docker-compose run worker
```

Example output:
```
Adding `Frieren: Beyond Journey’s End' to calendar `<calendar name>'...
✅ Event added for `Frieren: Beyond Journey’s End`
Adding `The Apothecary Diaries' to calendar `<calendar name>'...
Event already exists for `The Apothecary Diaries'. Skipping...
```

## Any issues?

Feel free to [open an issue](https://github.com/thedrummeraki/anime-schedule/issues/new).
