pub(super) enum SupertokensPath {
    SignUp,
    SignIn,
    EmailVerificationToken,
    EmailVerification,
    PasswordResetToken,
    PasswordResetTokenConsume,
    UpdateUser,
    NewSession,
    VerifySession,
    RefreshSession,
}

impl SupertokensPath {
    pub(super) fn to_string(self) -> String {
        match self {
            Self::SignUp => "/recipe/signup".to_string(),
            Self::SignIn => "/recipe/signin".to_string(),
            Self::EmailVerificationToken => "/recipe/user/email/verify/token".to_string(),
            Self::EmailVerification => "/recipe/user/email/verify".to_string(),
            Self::PasswordResetToken => "/recipe/user/password/reset/token".to_string(),
            Self::PasswordResetTokenConsume => {
                "/recipe/user/password/reset/token/consume".to_string()
            }
            Self::UpdateUser => "/recipe/user".to_string(),
            Self::NewSession => "/recipe/session".to_string(),
            Self::VerifySession => "/recipe/session/verify".to_string(),
            Self::RefreshSession => "/recipe/session/refresh".to_string(),
        }
    }
}
