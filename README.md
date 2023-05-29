# dIRC

## Distributed Systems Group Project

Nikita Skobelevs, Evan Brierton, Christian Wang

Usage: `docker compose up --build`. Go to `localhost:3000` in the browser.

Initial build times can take up to 10 minutes but subsequent builds are faster as we implemented dependency caching. We experienced a little bit of non-deterministic behaviour where the mysql server could sometimes crash on first build but this did not happen when `docker compose up` was run again.

See Report.pdf for more details and instructions.