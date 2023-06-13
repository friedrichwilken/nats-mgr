This manager is inspired by the [coredb operator](https://github.com/CoreDB-io/coredb/tree/main/coredb-operator).
Another resource are the official [kube.rs docs](https://kube.rs/controllers/intro/).

# Getting stared
If you don't have already, get [Rust](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup) and [just](https://github.com/casey/just#packages).

# Features
To render the crd 

```
    just generate-crd
```

To install the crd on a cluster

```
    just install-crd
```

