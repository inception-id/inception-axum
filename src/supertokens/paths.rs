pub(super) enum SupertokensPath {
    SignUp,
    EmailVerificationToken,
}

impl SupertokensPath {
    pub(super) fn to_string(self) -> String {
        match self {
            Self::SignUp => "/recipe/signup".to_string(),
            Self::EmailVerificationToken => "/recipe/user/email/verify/token".to_string(),
        }
    }
}
