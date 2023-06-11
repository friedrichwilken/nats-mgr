use controller::apis::nats_types::Nats;
use kube::CustomResourceExt;

fn main() {
    print!("{}", serde_yaml::to_string(&Nats::crd()).unwrap())
}
