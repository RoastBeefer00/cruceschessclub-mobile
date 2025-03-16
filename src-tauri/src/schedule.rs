use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Year {
    #[serde(rename = "January")]
    pub january: Vec<Business>,
    #[serde(rename = "February")]
    pub february: Vec<Business>,
    #[serde(rename = "March")]
    pub march: Vec<Business>,
    #[serde(rename = "April")]
    pub april: Vec<Business>,
    #[serde(rename = "May")]
    pub may: Vec<Business>,
    #[serde(rename = "June")]
    pub june: Vec<Business>,
    #[serde(rename = "July")]
    pub july: Vec<Business>,
    #[serde(rename = "August")]
    pub august: Vec<Business>,
    #[serde(rename = "September")]
    pub september: Vec<Business>,
    #[serde(rename = "October")]
    pub october: Vec<Business>,
    #[serde(rename = "November")]
    pub november: Vec<Business>,
    #[serde(rename = "December")]
    pub december: Vec<Business>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Business {
    pub name: String,
    pub address: String,
}
