const LOGINS: &'static str = include_str!("login.skrt");

#[derive(Debug)]
struct Login {
    _name: String,
    _password: String,
}

impl Login {
    pub fn name(&self) -> &String {
        &self._name
    }

    pub fn password(&self) -> &String {
        &self._password
    }

    pub fn validate(name: String, password: String) -> bool {
        match Login::check_logins(name, password) {
            Ok(_) => true,
            _ => false
        }
    }

    fn check_logins(name: String, password: String) -> Result<Login, &'static str> {
        
        let login_input: Vec<Login> =
            LOGINS
                .lines()
                .map(
                    |input| {
                        let data: Vec<&str> = 
                            input
                                .split(".")
                                .collect();
                        
                        Login {
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
    use super::Login;

    #[test]
    fn valid_login() {
        assert!(Login::validate("Student".to_string(), "1".to_string()));
    }

    #[test]
    fn invalid_login() {
        assert!(!Login::validate("".to_string(), "".to_string()));
    }
}