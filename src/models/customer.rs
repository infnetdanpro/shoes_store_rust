use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AcceptEnum {
    On,
    Off,
}
fn default_accept_enum() -> AcceptEnum {
    AcceptEnum::Off
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewCustomer {
    pub(crate) email: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) date_birth: NaiveDate,
    pub(crate) phone: String,
    pub(crate) city: String,
    pub(crate) country: String,
    pub(crate) password: String,
    pub(crate) confirm_password: String,

    #[serde(default = "default_accept_enum")]
    pub(crate) accept_all: AcceptEnum,
}
