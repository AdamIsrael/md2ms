use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct PII {
    /// Your full legal name
    pub legal_name: Option<String>,

    /// Your email address
    pub email: Option<String>,

    /// Your phone number
    pub phone: Option<String>,

    /// Your physical address
    pub address1: Option<String>,
    /// Your physical address
    pub address2: Option<String>,
    /// Your physical address
    pub city: Option<String>,
    /// Your physical address
    pub state: Option<String>,
    /// Your physical address
    pub postal_code: Option<String>,
    /// Your physical address
    pub country: Option<String>,

    /// A list of professional affiliations, if applicable.
    pub affiliations: Option<Vec<String>>,
}
