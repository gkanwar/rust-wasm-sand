Prerequisites
=============
This project depends on
 * rust/cargo: Install using [`rustup`](https://rustup.rs/).
 * wasm-pack: Install [here](https://rustwasm.github.io/wasm-pack/installer/).
 * Node.js and npm: See the package manager for your distro.

Building and running
====================
To build the web packages:
 * Install all npm requirements (in `node_modules`)

    ```
    npm install
    ```

 * And either... build the crate and wasm module (in `pkg`)

    ```
    wasm-pack build
    ```

 * Or... run the live dev server with

    ```
    npm run start
    ```

    Note that there may be some issues with fully refreshing the project depending on the filesystem monitoring available on your system.

