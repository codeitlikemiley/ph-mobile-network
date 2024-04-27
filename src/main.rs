use ph_mobile_network::mobile_network::MobileNetwork;
use ph_mobile_network::validate::Validate;

fn main() {
    let number = "09171234567";
    let network_result = MobileNetwork::get(number).unwrap();
    println!("{:?}", network_result.validate(number).unwrap());
}
