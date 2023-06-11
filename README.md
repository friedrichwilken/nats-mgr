Let's build a Kubernetes controller with [kube.rs](https://github.com/kube-rs).

This manager is inspired by the [coredb operator](https://github.com/CoreDB-io/coredb/tree/main/coredb-operator).
Another resource are the official [kube.rs docs](https://kube.rs/controllers/intro/).

For now all this can do is to build a basic `crd` via
```
cargo run --bin crdgen
```
