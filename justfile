default:
  @just --list --unsorted --color=always | rg -v "    default"

# generate the crd
generate-crd:
  cargo run --bin crdgen > charts/nats/templates/crd.yaml

# generate the crd and install into the current cluster.
install-crd: generate-crd
  kubectl apply -f charts/nats/templates/crd.yaml

