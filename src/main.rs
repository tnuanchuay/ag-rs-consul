use crate::ag_consul::{AgConsul, FeatureReader, KeyValueReader};

mod ag_consul;

#[tokio::main]
async fn main() {
    let consul = AgConsul::default("http://127.0.0.1:8500", "dc1");
    let enable_test_value = consul.read_key("it-fe/webgate/FeatureSwitch/Engineering/EnableTest").await.unwrap_or_default();
    println!("{:?}", enable_test_value);

    let is_test_enabled = consul.is_feature_enabled("it-fe/webgate/FeatureSwitch/Engineering/EnableTest").await;
    println!("{:?}", is_test_enabled);
}
