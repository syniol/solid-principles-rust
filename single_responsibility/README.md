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
pub struct Registration {}

impl Registration {
    pub fn register_user(&self, user: &User) -> Result<String, Error> {
        //...
    }
}
```
Inside **before** refactor, you can see all the logic for date parser and span creation and 
comparison are done inside `register_user` method. However, after implementing **SRP** inside 
**after** package. You can see there is a new package called `util` that holds a logic for date 
parser and comparison `src/util/date_time.rs`.

```rust
pub struct DateTime {}

impl DateTime {
    pub fn parse_raw_date(&self, formatted_date: &str) -> Result<Date, Error> {
        //...
    }

    pub fn get_date_span_in_years(&self, comparable_date_from: Date, comparable_date_to: Date) -> Result<i16, Error> {
        //...
    }
}
```
then module is imported inside the `user` and utilised as part of `Registration` struct and its implementation.
```rust
pub struct Registration {
    date_time: DateTime,
}

impl Registration {
    pub fn register_user(&self, user: &User) -> Result<String, Error> {
        let parsed_date = self.date_time.parse_raw_date(user.date_of_birth);
        let span = self.date_time.get_date_span_in_years(parsed_date.unwrap(), Zoned::now().date());


        Ok(format!("Successfully registered user: {}", user.name).to_string())
    }
}
```


#### Dependencies
 * [Jiff - _A date-time library_](https://crates.io/crates/jiff)


> Disclaimer: The training of any artificial intelligence model utilising the code and documentation housed
> within this repository is strictly prohibited and will be subject to prosecution under copyright law globally.


#### Credits
Author: [Hadi Tajallaei](mailto:hadi@syniol.com)

Copyright © 2026 Syniol Limited. All rights reserved.
