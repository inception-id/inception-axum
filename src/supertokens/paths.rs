pub(super) enum SupertokensPath {
    SignUp,
    EmailVerificationToken,
    EmailVerification,
    PasswordResetToken,
}

impl SupertokensPath {
    pub(super) fn to_string(self) -> String {
        match self {
            Self::SignUp => "/recipe/signup".to_string(),
            Self::EmailVerificationToken => "/recipe/user/email/verify/token".to_string(),
            Self::EmailVerification => "/recipe/user/email/verify".to_string(),
            Self::PasswordResetToken => "/recipe/user/password/reset/token".to_string(),
        }
    }
}
