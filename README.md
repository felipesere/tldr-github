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

## Structure and walkabout

The app is currently split in a backend written in Rust and two front ends.
"Two frontends?" you ask. Yes. 
There is a spike in [Svelte](https://svelte.dev/) in [tldr-github-svelte](./tldr-github-svelte) and early-stage port of the same UI in to React with [Parcel.JS](https://parceljs.org/) in [tldr-github-parcel](./tldr-github-parcel).

### The frontends
There are currently two version of the frontend because I statted toying with Svelte but found that I had a hard time with the testing side of it.
Svelte is great fun to write and has a few concepts I _really_ like:
 * custom events
 * the different built-in stores
 * reactivity _(though I still struggle with it)_

The bits made me go back to React were mostly the tests with Jest and inline JSX.
I like the way my tests could express how different parts of the DOM reacted to events/user input

Take this imaginary test for example:
```
it("shows an error if the repo can't be added", () => {
  const ui = mount(
    <ErrorContext>
       <Errors />
       <AddRepoThing />
    </ErrorContext>
  )

  let addRepoSpy = spyOn(someModule, 'addNewRepo');
  addRepoSpy.mockRejectedValue({error: "Not found"});

  ui.find("AddNewRepoButton).click()

  expect(ui.find(ErrorToast)).toExist();
})
```

It shows the interaction between `ErrorContext`, `Errors`, and `AddRepoThing`.
I could have tested each individually, but they make sense to the user mostly as a unit.
In Svelte, I found no easy way to make it as explicit. 
The way I've read online is that folks use a separate file with a custom Svelte component to bring those elements together.
If you happen to know of better ways of doing it, open an issue with a sample or possibly even a PR :smile:.


### Rust for the backend?

Half the goal of this repo is to experiment with Rust and get to know some of the libraries _in real life_, beyond just tutorials and the Rust book.
We use [Tide](https://github.com/http-rs/tide) to server the page and as much async/await thorugh [async-std](https://github.com/async-rs/async-std) as we can.
I am curious to see how much valuable information can be encoded in the type system without it becoming overbearing, and how we can leverage it for meaningful design along the lines of Eric Evans _Domain Driven Design_.

The backend is split into 
* API, which is mostly in `main.rs` and in `api.rs`
* an  `updater/mod.rs` that constantly checks for updated repos
* the `db` module with various implementation for storage
* the `github` module to retrieve repos, issues and prs via Githubs GraphQL API.
* and a `domain/mod.rs` module that I have tried to keep free of other dependencies
* `config/mod.rs` for some basic configuration.

It's all pretty experimental at the moment. 
If you have ideas/suggestions on techniques to use or to avoid, drop me an issue, or better yet, a PR!