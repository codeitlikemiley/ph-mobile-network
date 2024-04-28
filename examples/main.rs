use ph_mobile_network::mobile_network::MobileNetwork;
use ph_mobile_network::mutate::{append_dito_prefixes, append_globe_prefixes, append_smart_prefixes, append_sun_prefixes, append_tnt_prefixes, reset_dito_prefixes, reset_globe_prefixes, reset_smart_prefixes, reset_sun_prefixes, reset_tnt_prefixes};
use ph_mobile_network::validate::Validate;


fn main() {
    // if for some reason the default library prefixes has invalid values you can always reset it
    reset_dito_prefixes();
    reset_globe_prefixes();
    reset_smart_prefixes();
    reset_sun_prefixes();
    reset_tnt_prefixes();
    // if the current prefix isnt supported yet by the library you can always append it on compile time
    // append new prefixes on different networks
    append_dito_prefixes(&["0911", "0912"]); // Adding new TNT prefixes
    append_globe_prefixes(&["0917", "0996"]); // Adding new Globe prefixes
    append_smart_prefixes(&["0918", "0919"]); // Adding new Smart prefixes
    append_sun_prefixes(&["0933", "0934"]); // Adding new Sun prefixes
    append_tnt_prefixes(&["0899", "0900"]); // Adding new Dito prefixes

    // Example phone number to validate
    let number = "09171234567";

    // Get the network based on the phone number and validate the number
    match MobileNetwork::get(number) {
        Ok(network) => {
            println!("Network identified: {}", network.to_string());
            match network.validate(number) {
                Ok(valid) => println!("Validation result: {}", valid),
                Err(e) => println!("Validation error: {:?}", e),
            }
        },
        Err(e) => println!("Error retrieving network: {:?}", e),
    }
}

