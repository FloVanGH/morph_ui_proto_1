# morph

Embedded Rust UI-Toolkit (no_std).

# Usage

To include OrbTk in your project, add this dependency
line to your `Cargo.toml` file:

```text
morpth = { git = "https://codeberg.org/morph/morph.git" }
```

# Run examples on web

## Requirements

## Rust

morph requires Rust 1.30 or newer.

## wasm-pack

`wasm-pack` is required for building Rust-generated WebAssembly. Install it from https://rustwasm.github.io/wasm-pack/installer/.

## npm

`npm` is a JavaScript package manger and is used to install and run a JavaScript bundler and development server. You could install `npm` from https://www.npmjs.com/get-npm.

For the examples we uses the latest version of `npm`. You could install it with this command:

```shell
npm install npm@latest -g
```

## Run minimal example

1. Run this command on the minimal example project directory:  

```shell
wasm-pack build
```

2. Navigate to the `www` sub directory and run:

```shell
npm install
```

3. To serve the example locally run:

```shell
npm run start
```

4. Navigate your Web browser to http://localhost:8080/