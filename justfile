default:
  @just --list --unsorted --color=always | rg -v "    default"

# generate the crd
generate-crd:
  cargo run --bin crdgen > charts/nats/templates/natscrd.yaml

# generate the crd and install into the current cluster.
install-crd:
  kubectl apply -f charts/nats/templates/natscrd.yaml

