// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::b2b::mfa::MfaRequired;
use crate::b2b::organizations::Member;
use crate::b2b::organizations::Organization;
use crate::b2b::passwords_email::Email;
use crate::b2b::passwords_existing_password::ExistingPassword;
use crate::b2b::passwords_session::Sessions;
use crate::b2b::sessions::MemberSession;
use crate::consumer::passwords::Argon2Config;
use crate::consumer::passwords::MD5Config;
use crate::consumer::passwords::PBKDF2Config;
use crate::consumer::passwords::SHA1Config;
use crate::consumer::passwords::ScryptConfig;
use serde::{Deserialize, Serialize};

/// LudsFeedback:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LudsFeedback {
    /// has_lower_case: For LUDS validation, whether the password contains at least one lowercase letter.
    pub has_lower_case: bool,
    /// has_upper_case: For LUDS validation, whether the password contains at least one uppercase letter.
    pub has_upper_case: bool,
    /// has_digit: For LUDS validation, whether the password contains at least one digit.
    pub has_digit: bool,
    /// has_symbol: For LUDS validation, whether the password contains at least one symbol. Any UTF8 character
    /// outside of a-z or A-Z may count as a valid symbol.
    pub has_symbol: bool,
    /// missing_complexity: For LUDS validation, the number of complexity requirements that are missing from the
    /// password.
    ///   Check the complexity fields to see which requirements are missing.
    pub missing_complexity: i32,
    /// missing_characters: For LUDS validation, this is the required length of the password that you've set
    /// minus the length of the password being checked.
    ///   The user will need to add this many characters to the password to make it valid.
    pub missing_characters: i32,
}
/// ZxcvbnFeedback:
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZxcvbnFeedback {
    /// warning: For zxcvbn validation, contains an end user consumable warning if the password is valid but not
    /// strong enough.
    pub warning: String,
    /// suggestions: For zxcvbn validation, contains end user consumable suggestions on how to improve the
    /// strength of the password.
    pub suggestions: std::vec::Vec<String>,
}
/// AuthenticateRequest: Request type for `Passwords.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// email_address: The email address of the Member.
    pub email_address: String,
    /// password: The password to authenticate, reset, or set for the first time. Any UTF8 character is allowed,
    /// e.g. spaces, emojis, non-English characers, etc.
    pub password: String,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: std::option::Option<String>,
    /// session_duration_minutes: Set the session lifetime to be this many minutes from now. This will start a
    /// new session if one doesn't already exist,
    ///   returning both an opaque `session_token` and `session_jwt` for this session. Remember that the
    /// `session_jwt` will have a fixed lifetime of
    ///   five minutes regardless of the underlying session duration, and will need to be refreshed over time.
    ///
    ///   This value must be a minimum of 5 and a maximum of 527040 minutes (366 days).
    ///
    ///   If a `session_token` or `session_jwt` is provided then a successful authentication will continue to
    /// extend the session this many minutes.
    ///
    ///   If the `session_duration_minutes` parameter is not specified, a Stytch session will be created with a
    /// 60 minute duration. If you don't want
    ///   to use the Stytch session product, you can ignore the session fields in the response.
    pub session_duration_minutes: std::option::Option<i32>,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: std::option::Option<String>,
    /// session_custom_claims: Add a custom claims map to the Session being authenticated. Claims are only
    /// created if a Session is initialized by providing a value in
    ///   `session_duration_minutes`. Claims will be included on the Session object and in the JWT. To update a
    /// key in an existing Session, supply a new value. To
    ///   delete a key, supply a null value. Custom claims made with reserved claims (`iss`, `sub`, `aud`,
    /// `exp`, `nbf`, `iat`, `jti`) will be ignored.
    ///   Total custom claims size cannot exceed four kilobytes.
    pub session_custom_claims: std::option::Option<serde_json::Value>,
    /// locale: If the needs to complete an MFA step, and the Member has a phone number, this endpoint will
    /// pre-emptively send a one-time passcode (OTP) to the Member's phone number. The locale argument will be
    /// used to determine which language to use when sending the passcode.
    ///
    /// Parameter is a [IETF BCP 47 language tag](https://www.w3.org/International/articles/language-tags/),
    /// e.g. `"en"`.
    ///
    /// Currently supported languages are English (`"en"`), Spanish (`"es"`), and Brazilian Portuguese
    /// (`"pt-br"`); if no value is provided, the copy defaults to English.
    ///
    /// Request support for additional languages
    /// [here](https://docs.google.com/forms/d/e/1FAIpQLScZSpAu_m2AmLXRT3F3kap-s_mcV6UTBitYn6CdyWP0-o7YjQ/viewform?usp=sf_link")!
    ///
    pub locale: std::option::Option<AuthenticateRequestLocale>,
    /// intermediate_session_token: Adds this primary authentication factor to the intermediate session token.
    /// If the resulting set of factors satisfies the organization's primary authentication requirements and MFA
    /// requirements, the intermediate session token will be consumed and converted to a member session. If not,
    /// the same intermediate session token will be returned.
    pub intermediate_session_token: std::option::Option<String>,
}
/// AuthenticateResponse: Response type for `Passwords.authenticate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object)
    pub member: Member,
    /// session_token: A secret token for a given Stytch Session.
    pub session_token: String,
    /// session_jwt: The JSON Web Token (JWT) for a given Stytch Session.
    pub session_jwt: String,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: Organization,
    /// intermediate_session_token: The returned Intermediate Session Token contains a password factor
    /// associated with the Member. If this value is non-empty, the member must complete an MFA step to finish
    /// logging in to the Organization. The token can be used with the
    /// [OTP SMS Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-otp-sms),
    /// [TOTP Authenticate endpoint](https://stytch.com/docs/b2b/api/authenticate-totp), or
    /// [Recovery Codes Recover endpoint](https://stytch.com/docs/b2b/api/recovery-codes-recover) to complete an
    /// MFA flow and log in to the Organization. Password factors are not transferable between Organizations, so
    /// the intermediate session token is not valid for use with discovery endpoints.
    pub intermediate_session_token: String,
    /// member_authenticated: Indicates whether the Member is fully authenticated. If false, the Member needs to
    /// complete an MFA step to log in to the Organization.
    pub member_authenticated: bool,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// member_session: The [Session object](https://stytch.com/docs/b2b/api/session-object).
    pub member_session: std::option::Option<MemberSession>,
    /// mfa_required: Information about the MFA requirements of the Organization and the Member's options for
    /// fulfilling MFA.
    pub mfa_required: std::option::Option<MfaRequired>,
}
/// MigrateRequest: Request type for `Passwords.migrate`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MigrateRequest {
    /// email_address: The email address of the Member.
    pub email_address: String,
    /// hash: The password hash. For a Scrypt or PBKDF2 hash, the hash needs to be a base64 encoded string.
    pub hash: String,
    /// hash_type: The password hash used. Currently `bcrypt`, `scrypt`, `argon_2i`, `argon2_id`, `md_5`,
    /// `sha_1`, and `pbkdf_2` are supported.
    pub hash_type: MigrateRequestHashType,
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// md_5_config: Optional parameters for MD-5 hash types.
    pub md_5_config: std::option::Option<MD5Config>,
    /// argon_2_config: Required parameters if the argon2 hex form, as opposed to the encoded form, is supplied.
    pub argon_2_config: std::option::Option<Argon2Config>,
    /// sha_1_config: Optional parameters for SHA-1 hash types.
    pub sha_1_config: std::option::Option<SHA1Config>,
    /// scrypt_config: Required parameters if the scrypt is not provided in a **PHC encoded form**.
    pub scrypt_config: std::option::Option<ScryptConfig>,
    /// pbkdf_2_config: Required additional parameters for PBKDF2 hash keys. Note that we use the SHA-256 by
    /// default, please contact [support@stytch.com](mailto:support@stytch.com) if you use another hashing
    /// function.
    pub pbkdf_2_config: std::option::Option<PBKDF2Config>,
    /// name: The name of the Member. Each field in the name object is optional.
    pub name: std::option::Option<String>,
    /// trusted_metadata: An arbitrary JSON object for storing application-specific data or
    /// identity-provider-specific data.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
    /// untrusted_metadata: An arbitrary JSON object of application-specific data. These fields can be edited
    /// directly by the
    ///   frontend SDK, and should not be used to store critical information. See the
    /// [Metadata resource](https://stytch.com/docs/b2b/api/metadata)
    ///   for complete field behavior details.
    pub untrusted_metadata: std::option::Option<serde_json::Value>,
    /// roles: Roles to explicitly assign to this Member.
    ///  Will completely replace any existing explicitly assigned roles. See the
    ///  [RBAC guide](https://stytch.com/docs/b2b/guides/rbac/role-assignment) for more information about role
    /// assignment.
    ///
    ///    If a Role is removed from a Member, and the Member is also implicitly assigned this Role from an SSO
    /// connection
    ///    or an SSO group, we will by default revoke any existing sessions for the Member that contain any SSO
    ///    authentication factors with the affected connection ID. You can preserve these sessions by passing in
    /// the
    ///    `preserve_existing_sessions` parameter with a value of `true`.
    pub roles: std::option::Option<std::vec::Vec<String>>,
    /// preserve_existing_sessions: Whether to preserve existing sessions when explicit Roles that are revoked
    /// are also implicitly assigned
    ///   by SSO connection or SSO group. Defaults to `false` - that is, existing Member Sessions that contain
    /// SSO
    ///   authentication factors with the affected SSO connection IDs will be revoked.
    pub preserve_existing_sessions: std::option::Option<bool>,
}
/// MigrateResponse: Response type for `Passwords.migrate`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MigrateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member.
    pub member_id: String,
    /// member_created: A flag indicating `true` if a new Member object was created and `false` if the Member
    /// object already existed.
    pub member_created: bool,
    /// member: The [Member object](https://stytch.com/docs/b2b/api/member-object)
    pub member: Member,
    /// organization: The [Organization object](https://stytch.com/docs/b2b/api/organization-object).
    pub organization: Organization,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// StrengthCheckRequest: Request type for `Passwords.strength_check`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StrengthCheckRequest {
    /// password: The password to authenticate, reset, or set for the first time. Any UTF8 character is allowed,
    /// e.g. spaces, emojis, non-English characers, etc.
    pub password: String,
    /// email_address: The email address of the Member.
    pub email_address: std::option::Option<String>,
}
/// StrengthCheckResponse: Response type for `Passwords.strength_check`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StrengthCheckResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// valid_password: Returns `true` if the password passes our password validation. We offer two validation
    /// options,
    ///   [zxcvbn](https://stytch.com/docs/passwords#strength-requirements) is the default option which offers a
    /// high level of sophistication.
    ///   We also offer [LUDS](https://stytch.com/docs/passwords#strength-requirements). If an email address is
    /// included in the call we also
    ///   require that the password hasn't been compromised using built-in breach detection powered by
    /// [HaveIBeenPwned](https://haveibeenpwned.com/)
    pub valid_password: bool,
    /// score: The score of the password determined by [zxcvbn](https://github.com/dropbox/zxcvbn). Values will
    /// be between 1 and 4, a 3 or greater is required to pass validation.
    pub score: i32,
    /// breached_password: Returns `true` if the password has been breached. Powered by
    /// [HaveIBeenPwned](https://haveibeenpwned.com/).
    pub breached_password: bool,
    /// strength_policy: The strength policy type enforced, either `zxcvbn` or `luds`.
    pub strength_policy: String,
    /// breach_detection_on_create: Will return `true` if breach detection will be evaluated. By default this
    /// option is enabled.
    ///   This option can be disabled by contacting
    /// [support@stytch.com](mailto:support@stytch.com?subject=Password%20strength%20configuration).
    ///   If this value is false then `breached_password` will always be `false` as well.
    pub breach_detection_on_create: bool,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// luds_feedback: Feedback for how to improve the password's strength using
    /// [luds](https://stytch.com/docs/passwords#strength-requirements).
    pub luds_feedback: std::option::Option<LudsFeedback>,
    /// zxcvbn_feedback: Feedback for how to improve the password's strength using
    /// [zxcvbn](https://stytch.com/docs/passwords#strength-requirements).
    pub zxcvbn_feedback: std::option::Option<ZxcvbnFeedback>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum AuthenticateRequestLocale {
    #[serde(rename = "en")]
    #[default]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ptbr")]
    Ptbr,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum MigrateRequestHashType {
    #[serde(rename = "bcrypt")]
    #[default]
    Bcrypt,
    #[serde(rename = "md_5")]
    Md5,
    #[serde(rename = "argon_2i")]
    Argon2i,
    #[serde(rename = "argon_2id")]
    Argon2id,
    #[serde(rename = "sha_1")]
    Sha1,
    #[serde(rename = "scrypt")]
    Scrypt,
    #[serde(rename = "phpass")]
    Phpass,
    #[serde(rename = "pbkdf_2")]
    Pbkdf2,
}

pub struct Passwords {
    http_client: crate::client::Client,
    pub email: Email,
    pub sessions: Sessions,
    pub existing_password: ExistingPassword,
}

impl Passwords {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            email: Email::new(http_client.clone()),
            sessions: Sessions::new(http_client.clone()),
            existing_password: ExistingPassword::new(http_client.clone()),
        }
    }

    pub async fn strength_check(
        &self,
        body: StrengthCheckRequest,
    ) -> crate::Result<StrengthCheckResponse> {
        let path = String::from("/v1/b2b/passwords/strength_check");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn migrate(&self, body: MigrateRequest) -> crate::Result<MigrateResponse> {
        let path = String::from("/v1/b2b/passwords/migrate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn authenticate(
        &self,
        body: AuthenticateRequest,
    ) -> crate::Result<AuthenticateResponse> {
        let path = String::from("/v1/b2b/passwords/authenticate");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
