const logins: &'static str = include_str!("login.skrt");

#[derive(Debug)]
struct login {
    _name: String,
    _password: String,
}

impl login {
    pub fn name(&self) -> &String {
        &self._name
    }

    pub fn password(&self) -> &String {
        &self._password
    }

    pub fn validate(name: String, password: String) -> bool {
        match login::check_logins(name, password) {
            Ok(_) => true,
            _ => false
        }
    }

    fn check_logins(name: String, password: String) -> Result<login, &'static str> {
        
        let login_input: Vec<login> =
            logins
                .lines()
                .map(
                    |input| {
                        let data: Vec<&str> = 
                            input
                                .split(".")
                                .collect();
                        
                        login {
                            _name: data[0].to_string(),
                            _password: data[1].to_string(),
                        }
                    }
                )
                .collect();

        for elem in login_input {
            if elem.name().to_string() == name && elem.password().to_string() == password {
                return Ok(elem)
            }
        }

        Err("No login available")
    }
}
#[cfg(test)]
mod tests {
    use super::login;

    #[test]
    fn it_works() {
        assert!(!login::validate("".to_string(), "".to_string()));
    }
}