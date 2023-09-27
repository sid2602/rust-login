use pwhash::bcrypt;

pub fn check_password_is_valid_when_register(
    password: String,
    repeated_password: String
) -> Result<String,String> {

    if password != repeated_password {
        return Err("Passwords must be the same".to_string());
    }

    if password.len() < 4 {
        return Err("Password must contain at least 4 characters".to_string());
    }

    return Ok(password);
}

pub fn hash_password(
    password: String
) -> Result<String,String> {
    bcrypt::hash(password).map_err(|err| err.to_string())
}

pub fn is_password_valid_with_hashed_password(
    password: String,
    hashed_password: String
) -> bool {
    bcrypt::verify(password, hashed_password.as_str())
}

