# TLDR GitHub

TLDR Github is a small UI to help you keep track of repositories you are interested and give you an overview of activity.

![Screenshot showing three "cards" representing individual repos](images/overview.png)

## Background

You can look at your notifications and activity on GitHub, but that can get flooded with stars and other acticity that adds noise.
The intent here is to have a very narrow scope, but provide a high signal-to-noise ratio.

## Roadmap

You can see what I am up to [here](https://github.com/felipesere/tldr-github/projects/2) in the Github Project

## Setup

You will need:
 * make
 * Rust v1.14.1+
 * Node v13.2.0+
 * sqlite3
   - install on Mac via `brew install sqlite3`

We use `make` as the default tool to build, start or test the application.
By executing just `make` (without arguments) you will get a list of all available tasks.

With all requirements above met, you should be able to run `make setup` to install any dependencies.
Further, you can run the application via `make run` or run the tests with `make test`.

## Thanks

Hopefully [Charlotte](https://github.com/charlottebrf) and [Christoph](https://github.com/christophgockel) will join me!
