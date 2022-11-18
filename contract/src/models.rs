use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

/* ///////////////////////////////////////////////////////////////
                             JOBS
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ */

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
    pub employment_type: Option<EmploymentType>,
    pub job_title: Option<String>,
    pub compensation: Option<u64>,
    pub company_name: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum EmploymentType {
    FullTime,
    PartTime,
    ContractWork,
    EmployeeChoice,
}

/* ///////////////////////////////////////////////////////////////
                             HOUSING
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ */

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Housing {
    rent: Option<u64>,
    per_time_range: Option<PerTimeRange>,
    sqft: Option<u64>,
    pet: Option<bool>,
    air_conditioning: Option<bool>,
    private_room: Option<bool>,
    housing_type: Option<HousingType>,
    laundry: Option<bool>,
    parking: Option<bool>,
    available_date: Option<String>,
    open_house_dates: Option<Vec<String>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum PerTimeRange {
    Day,
    Week,
    Month,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum HousingType {
    Apartment,
    Condo,
    CottageOrCabin,
    Duplex,
    Flat,
    House,
    InLaw,
    Loft,
    TownHouse,
    Manufactured,
    AssistedLiving,
    Land,
}

/* ///////////////////////////////////////////////////////////////
                             FOR SALE
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ */
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct ForSale {
    make_or_manufacturer: Option<String>,
    model_name_or_number: Option<String>,
    size_dimensions: Option<String>,
    condition: Option<Condition>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Condition {
    New,
    LikeNew,
    Excellent,
    Good,
    Fair,
    Salvage,
}

/* ///////////////////////////////////////////////////////////////
                             COMMUNITY
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\ */

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Community {
    garage_sale: Option<GarageOrMovingSales>,
    class_or_event: Option<ClassesOrEvents>,
    lost_or_found: Option<bool>,
    rideshare: Option<bool>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct GarageOrMovingSales {
    garage_sale_start_time: Option<String>,
    garage_sale_dates: Option<Vec<String>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct ClassesOrEvents {
    event_venue: Option<String>,
    event_start_date: Option<String>,
    event_duration: Option<u8>,
    event_features: Option<Vec<String>>,
}
