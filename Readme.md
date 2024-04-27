# PH Mobile Network Library

The PH Mobile Network Library provides a robust solution for identifying and validating mobile network operators based on phone number prefixes in the Philippines. It supports networks such as Globe, Smart, Sun, TNT, and Dito, and ensures comprehensive validation against common input errors.

Features

- Network Identification: Identify the network operator for a given phone number based on its prefix.
- Input Validation: Validate phone numbers for correct length and numeric consistency.
- Error Handling: Detailed error responses for various types of input and internal processing issues.


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ph_mobile_network = "0.1.0"
```

Make sure to replace `"0.1.0"` with the latest version of the library.

Usage

To use the library, include it in your Rust file and utilize the MobileNetwork::get function to determine the network of a phone number and validate it:

```rust
use ph_mobile_network::mobile_network::MobileNetwork;
use ph_mobile_network::validate::Validate;

fn main() {
    let number = "09171234567";
    let network = MobileNetwork::get(number);

    match network {
        Ok(net) => {
            if net.validate(number).unwrap() {
                println!("{} is a valid number.", number);
            } else {
                println!("{} is not a valid number.", number);
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }
}
```

Error Handling

The library defines several errors to manage possible failure scenarios:

- InvalidLength: The phone number does not meet the required length.
- NonNumeric: The phone number contains characters that are not digits.
- UnrecognizedPrefix: The phone number prefix does not match any known network.
- RegexError: There was an error in compiling the regular expression used for validation.

These errors are encapsulated in the `MobileNetworkError` enum.

Errors are handled using Rust's robust error handling features, allowing for detailed debugging and recovery options.

## Contributing

Contributions are welcome! If you would like to contribute, please fork the repository and submit a pull request.

1. Fork it (https://github.com/codeitlikemiley/ph_mobile_network/fork)
2. Create your feature branch (`git checkout -b missing-prefix/xxxx`)
3. Commit your changes (`git commit -am 'Add xxxx prefix on MobileNetwork::{Variants}'`)
4. Push to the branch (`git push origin missing-prefix/xxxx`)
5. Create a new Pull Request

## [License](./LICENSE)

This project is licensed under the MIT License - see the LICENSE file for details.

