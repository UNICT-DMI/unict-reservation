# Unict reservation Telegram bot

This _UNDER DEVELOPMENT_ project is used for the students of University of
Catania to book a space for study into the campus.

With this self-hosted bot you can use your Telegram app to reserve a seat into
your favority study room/library.

## Setup
Clone this repository into your VM
```
git clone https://github.com/dcariotti/unict-reservation
```

Setup the `.env` file or environment
```
cf=
password=
driver_url=
username=
TELOXIDE_TOKEN=
```

Where `cf` is the username of Smartedu, `password` is the password of that
account, `driver_url` is the location where geckodriver is running, `username` is
your Telegram username and `TELOXIDE_TOKEN` is the Telegram bot API token.

This bot uses [geckodriver](https://github.com/mozilla/geckodriver/releases) to
create an instance of browser.
If you want to run a geckodriver instance in your local machine and use that (so
the `driver_url=http://localhost:<port>` you have to run (in parallel) 
```
geckodriver --port=<port>
```

## Run
Now you can create the build of the project (`cargo build` and then execute the
binary file) or just run the software with `cargo run`.
