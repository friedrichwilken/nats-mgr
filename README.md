Let's try to build a k8s controller with Rust.

This is a research project to compare the experience of building controllers between Go + Kubebuilder and Rust + Kube.rs.

This project is inspired by the [coredb operator](https://github.com/CoreDB-io/coredb/tree/main/coredb-operator). Another great resource I use a lot are the official [kube.rs docs](https://kube.rs/controllers/intro/).

# Getting stared
If you don't have it already, get [Rust](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup) and [Just](https://github.com/casey/just#packages).

# Features
To render the crd:

```
    just generate-crd
```

To install the crd on a cluster:

```
    just install-crd
```

