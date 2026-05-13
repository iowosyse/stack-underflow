use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistroEmpresaReq {
    pub nombre_empresa: String,
    pub dominio: String,
    pub nombre_admin: String,
    pub email_admin: String,
    pub password_admin: String,
    pub captcha_token: String,
}
