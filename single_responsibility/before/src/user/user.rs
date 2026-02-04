pub struct User<'a> {
    pub name: &'a str,
    // formatted: YYYY-MM-DD (e.g., 2026-02-05)
    pub date_of_birth: &'a str,
}
