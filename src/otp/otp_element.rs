use qrcode::render::unicode;
use qrcode::QrCode;
use serde::{Deserialize, Serialize};

pub enum OTPType {
    TOTP,
    HOTP,
    STEAM,
    YANDEX,
}

#[derive(Serialize, Deserialize)]
pub struct OTPElement {
    secret: String,
    issuer: String,
    label: String,
    digits: u64,
    #[serde(rename = "type")]
    type_: String,
    algorithm: String,
    thumbnail: String,
    last_used: u64,
    used_frequency: u64,
    period: u64,
    counter: Option<u64>,
    tags: Vec<String>,
    yandex_pin: Option<String>,
}

impl OTPElement {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        secret: String,
        issuer: String,
        label: String,
        digits: u64,
        type_: String,
        algorithm: String,
        // thumbnail: String,
        // last_used: u64,
        // used_frequency: u64,
        period: u64,
        counter: Option<u64>,
        // tags: Vec<String>,
        yandex_pin: Option<String>,
    ) -> OTPElement {
        OTPElement {
            secret,
            issuer,
            label,
            digits,
            type_,
            algorithm,
            period,
            counter: counter,
            thumbnail: String::from(""),
            last_used: 0,
            used_frequency: 0,
            tags: vec![],
            yandex_pin,
        }
    }
    pub fn secret(&self) -> String {
        self.secret.to_string().replace('=', "")
    }
    pub fn label(&self) -> String {
        self.label.to_string()
    }
    pub fn issuer(&self) -> String {
        self.issuer.to_string()
    }
    pub fn digits(&self) -> u64 {
        self.digits
    }
    pub fn algorithm(&self) -> String {
        self.algorithm.to_string()
    }
    pub fn type_(&self) -> String {
        self.type_.to_string()
    }
    pub fn counter(&self) -> Option<u64> {
        self.counter
    }
    pub fn period(&self) -> u64 {
        self.period
    }
    pub fn yandex_pin(&self) -> Option<String> {
        self.yandex_pin.clone()
    }

    pub fn set_secret(&mut self, secret: String) {
        self.secret = secret;
    }
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
    pub fn set_issuer(&mut self, issuer: String) {
        self.issuer = issuer;
    }
    pub fn set_algorithm(&mut self, algorithm: String) {
        self.algorithm = algorithm;
    }
    pub fn set_digits(&mut self, digits: u64) {
        self.digits = digits;
    }
    pub fn set_counter(&mut self, counter: Option<u64>) {
        self.counter = counter;
    }

    pub fn get_otpauth_uri(&self) -> String {
        let mut uri: String = String::from("otpauth://");
        uri.push_str(self.type_.to_lowercase().as_str());
        uri.push('/');
        //self.type_.to_lowercase() + String::from("/");
        if self.issuer.chars().count() > 0 {
            uri.push_str(&urlencoding::encode(self.issuer.as_str()));
            uri.push(':');
        }
        uri.push_str(&urlencoding::encode(self.label.as_str()));

        uri.push_str("?secret=");
        uri.push_str(self.secret().as_str());
        uri.push_str("&algorithm=");
        uri.push_str(self.algorithm.to_uppercase().as_str());
        uri.push_str("&digits=");
        uri.push_str(self.digits().to_string().as_str());
        uri.push_str("&period=");
        uri.push_str(self.period.to_string().as_str());
        uri.push_str("&lock=false");
        //uri.push_str("?secret=" + self.secret());
        if self.type_.to_lowercase().as_str() == "hotp" {
            uri.push_str("&counter=");
            uri.push_str(self.counter.unwrap_or(0).to_string().as_str());
        }
        uri
    }

    pub fn get_qrcode(&self) -> String {
        QrCode::new(&self.get_otpauth_uri())
            .unwrap()
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build()
    }
}

#[cfg(test)]
mod test {
    use crate::otp::otp_element::OTPElement;

    #[test]
    fn test_otpauth_uri() {
        let otp_element = OTPElement::new(
            String::from("xr5gh44x7bprcqgrdtulafeevt5rxqlbh5wvked22re43dh2d4mapv5g"),
            String::from("IssuerText"),
            String::from("LabelText"),
            6,
            String::from("TOTP"),
            String::from("SHA1"),
            30,
            None,
            None,
        );
        assert_eq!(otp_element.get_otpauth_uri().as_str(), "otpauth://totp/IssuerText:LabelText?secret=xr5gh44x7bprcqgrdtulafeevt5rxqlbh5wvked22re43dh2d4mapv5g&algorithm=SHA1&digits=6&period=30&lock=false");
    }
}
