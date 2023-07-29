// provides connection(s) and access to a mysql database
// pull connection string from env
// provide a method to verify if a registration code is valid
// provide a method to claim the registion code
// provide a method to save a users: email and password
// hash and salt the password

struct Code {
    created_at: String,
    modified_on: String,
    modified_by: String,
    claimed: bool,
    owned_by: Option<String>,
}

pub fn verify() -> bool {
    true
}