---
sidebar_position: 1
---

# Prerequisites

This workshop gives you the opportunity to get hands on and build out a real Rust service. To do that though, you're going to need some things installed on your machine:

- [Rust](https://www.rust-lang.org/tools/install)
    - The language itself is kind of important. If you're on a Windows machine I'd highly recommend using WSL2 instead of Windows itself
- Docker Client
    - The ability to run Docker images, and the ability to run `docker compose`
- You will also need to clone the [GitHub repo](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop) with the examples and solutions
    ```sh
    git clone https://github.com/jeastham1993/rust-for-dotnet-devs-workshop
    ```

And that's it, that's the bare minimum you'll need to get through all the content in this workshop.

## Dev Containers

The [GitHub repository](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop) contains a `devcontainers` configuration in the root of the repository. You can use this to startup a local container with the required tooling to run through the examples in the workshop.