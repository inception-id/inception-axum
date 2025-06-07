pub(super) enum SupertokensPath {
    SignIn,
    SignUp,
}

impl SupertokensPath {
    pub(super) fn to_string(self) -> String {
        match self {
            Self::SignIn => "/recipe/signin".to_string(),
            Self::SignUp => "/recipe/signup".to_string(),
        }
    }
}
