# Elm and Rust Boilerplate

This boilerplate is a very simple starting point for using Elm and Rust for web development. As yet, it includes the following features:

- Full Docker Compose setup with ZSH and custom ZSH configuration
- NodeJS setup in the UI with Webpack and HMR
- Yarn 2 (with PnP for ultra-fast dependency management)
- Elm integration with Webpack (including HMR)
- Empty Elm project for frontend use
- Empty Rust project for backend use

For a less opinionated boilerplate, see [elm-rust-boilerplate-minimal](https://github.com/arctic-hen7/elm-rust-boilerplate-minimal).

## Installation

1. Clone this repository.
2. Shell into the Docker container (may take a while to set things up the first time, after that it should be super-fast).
3. In the container, switch to the `ui` directory and run `yarn dev` to run the app.
4. In your browser, go to <http://localhost:9000>, you should see `Hello from Elm!`!

You can do all that like so:
```
git clone git@github.com:arctic-hen7/elm-rust-boilerplate.git
cd elm-rust-boilerplate
rm -rf .git
bonnie shell
```
Then, in the Docker container:
```
cd ui
yarn dev
```

Then go to <http://localhost:9000> in your browser.

## Bonnie

This boilerplate uses [Bonnie](https://github.com/arctic-hen7/bonnie) to manage scripts at the root. You can install Bonnie for Windows, MacOS, or Linux from [here](https://github.com/arctic-hen7/bonnie/releases) or build the source code for any other OSes. Bonnie allows simple, blazingly fast alias management with arguments, and I use it for most of my projects when I need a script manager.

That being said, if you loathe Bonnie for some reason, you can easily substitute it for some other similar software, like Make. If you decide to use NPM or yarn scripts though, you may find problems down the line with using Yarn scripts with just a `package.json` file at the root, so I don't recommend that approach.

*Full Disclosure: I built and maintain Bonnie.*

## Roadmap

This boilerplate is currently identical to [elm-rust-boilerplate-minimal](https://github.com/arctic-hen7/elm-rust-boilerplate-minimal), however the following features are on the roadmap to make this boilerplate significantly more fully-featured. If you think there's a feature that should be in here, please file an issue on this repository!

- [ ] Add Elm UI
- [ ] Set up a GraphQL server in Rust

## License

See [`LICENSE.txt`](./LICENSE)
