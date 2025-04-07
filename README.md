Golem AI
========

Golem AI is a project meant to run on [Golem Cloud](https://golem.cloud) that aims to implement

Each component in the architecture exposes a WebAssembly interface that can be called by other components.

Setup
-----

The project requires:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Component](https://github.com/bytecodealliance/cargo-component)
  ```bash
  cargo install cargo-component
  ```
- [Golem](https://github.com/golemcloud/golem/releases)
    - You can download the latest binary from the GitHub releases page
- [Golem CLI](https://github.com/golemcloud/golem-cli/releases)
    - You can download the latest binary from the GitHub releases page
    - Or you can build it:
      ```bash
      cargo install golem-cli --locked
      ```
      Note: Requires protobuf installed on your system

Run
---

1. Spin up Golem OSS (optional, to run locally):

    ```bash
    golem server run -vv
    ```

2. Building is as simple as:

    ```bash
    golem-cli app build -b release
    ```

3. Deployment

   ```bash
   golem-cli app deploy
   ```

4. Instantiate input analyzer worker

   ```bash
   golem-cli worker new golem-ai:input-analyzer/golem-ai-input-analyzer-1 --env OPENAI_API_KEY="...."
   ```

5. Send a task to analyze:

   ```bash
   export INPUT="$(cat INPUT_ESCAPED.md)"
   golem-cli worker invoke golem-ai:input-analyzer/golem-ai-input-analyzer-1 {analyze} $INPUT
   ```