use solid_principles_dependency_inversion::home_office::home_office::HomeOffice;

fn main() {
    let home_office = HomeOffice {};

    home_office.find_passport("Smith".to_string(), 873242341).unwrap();
    home_office.find_visa_application(
        "John".to_string(),
        "Smith".to_string(),
        "1988-09-10".to_string(),
    ).unwrap();
}
