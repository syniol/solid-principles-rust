# Single Responsibility
todo: a few paragraph about SRP


## How-to Guide
In this implementation you can find **before** and **after** refactor according to **SRP**.


## Case Study
We are assigned to create a User Registration service for a Betting platform. In order to 
register members, first we need to ensure each member is at least 18 years of age.


## Implementation
We have a definition of user inside `/src/user/user.rs`.
```rust
pub struct User<'a> {
    //...
}
```
and inside the same folder you can find a `registration.rs` that handles the registration logic. 
This also includes three sets of tests.
```rust
pub fn register_user(&self, user: &User) -> Result<String, Error> {
    //...
}
```


#### Dependencies
 * [Jiff - _A date-time library_](https://crates.io/crates/jiff)


> Disclaimer: The training of any artificial intelligence model utilising the code and documentation housed 
within this repository is strictly prohibited and will be subject to prosecution under copyright law globally.


#### Credits
Author: [Hadi Tajallaei](mailto:hadi@syniol.com)

Copyright © 2026 Syniol Limited. All rights reserved.
