# PH Mobile Network Library

The PH Mobile Network Library provides a robust solution for identifying and validating mobile network operators based on phone number prefixes in the Philippines. It supports networks such as Globe, Smart, Sun, TNT, and Dito, and ensures comprehensive validation against common input errors.

Features

- Network Identification: Identify the network operator for a given phone number based on its prefix.
- Ability to Reset Specific Mobile Network to Empty Sets
- Appending Mobile Network Prefixes on Compile Time
- Input Validation: Validate phone numbers for correct length and numeric consistency.
- Error Handling: Detailed error responses for various types of input and internal processing issues.


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ph_mobile_network = "0.1.1"
```


Usage

To use the library, include it in your Rust file and utilize the MobileNetwork::get function to determine the network of a phone number and validate it:

```rust
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
```

run: `cargo run -p ph-mobile-network --bin ph-mobile-network`

output:

```sh
Network identified: globe
Validation result: true
```

Error Handling

The library defines several errors to manage possible failure scenarios:

- InvalidLength: The phone number does not meet the required length.
- NonNumeric: The phone number contains characters that are not digits.
- UnrecognizedPrefix: The phone number prefix does not match any known network.
- RegexError: There was an error in compiling the regular expression used for validation.
- MutexError: This Happens when you cannot obtain a lock on Mutex

These errors are encapsulated in the `MobileNetworkError` enum.

Errors are handled using Rust's robust error handling features, allowing for detailed debugging and recovery options.

## Contributing

Note: there might be missing , prefix or invalid prefix on the library ,if there is please submit a PR , all the prefixes are define on `./src/globals`

Contributions are welcome! If you would like to contribute, please fork the repository and submit a pull request.

1. Fork it (https://github.com/codeitlikemiley/ph_mobile_network/fork)
2. Create your feature branch (`git checkout -b missing-prefix/xxxx`)
3. Commit your changes (`git commit -am 'Add xxxx prefix on MobileNetwork::{Variants}'`)
4. Push to the branch (`git push origin missing-prefix/xxxx`)
5. Create a new Pull Request

## [License](./LICENSE)

This project is licensed under the MIT License - see the LICENSE file for details.

