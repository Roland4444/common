
use serde::{Serialize, Deserialize};


pub const PAYMENTS: &str = "Платежи";
pub const OLIVIA: &str = "ОЛИВИЯ МАКСАКОВА";
pub const BABEFA: &str = "ЖК Бабефа";
pub const OKLAND: &str = "ОКЛАНД РЫБАЦКАЯ";
pub const RED: &str = "РЭД Грузинская";
pub const TETRIS: &str = "ЖК Тетрис на Керченской";
pub const SCANDINAVIA: &str = "Скандинавия - Моздокская";
pub const KUIB: &str = "Куйбышева";
pub const POLZ: &str = "Ползунова";
pub const ZVEZD: &str = "Звездная";
pub const SKY: &str = "СКАЙ ИГАРСКАЯ";
pub const OWN: &str = "OWN";


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Collab { PAYMENTS,    OLIVIA,    BABEFA,    OKLAND,    RED,    TETRIS,    SCANDINAVIA,    KUIB,   POLZ,    ZVEZD,    SKY,    OWN,}

#[derive(Debug, Serialize, PartialEq, Deserialize)]
pub struct ExtractedMessage {    pub author_name: String,   pub text: String,    pub uuid: Option<String>,    pub id: u64,    pub chat_id: u64,}


#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteInfo {pub message_id: String, pub quoted_author: String,pub quoted_text: String,pub reply_text: Option<String>,  }




#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractResp {    pub success: bool,    pub quoted_text: Option<String>,    pub error: Option<String>,}

#[derive(Debug, Serialize, Deserialize)]
pub enum type_operation{    ExtractSimple,    ExtractFull
}

#[derive(Debug, Deserialize)]
pub struct ExtractReq {    pub collab: String,    pub message_id: u64,    pub type__: type_operation}


impl Collab {
    pub fn title(&self) -> &'static str {
        match self {            Collab::PAYMENTS => "Платежи",                      Collab::OLIVIA => "ОЛИВИЯ МАКСАКОВА",            Collab::BABEFA => "ЖК Бабефа",
                                Collab::OKLAND => "ОКЛАНД РЫБАЦКАЯ",                Collab::RED => "РЭД Грузинская",                 Collab::TETRIS => "ЖК Тетрис на Керченской",
                                Collab::SCANDINAVIA => "Скандинавия - Моздокская",  Collab::KUIB => "Куйбышева",                     Collab::POLZ => "Ползунова",
                                Collab::ZVEZD => "Звездная",                        Collab::SKY => "СКАЙ ИГАРСКАЯ",                  Collab::OWN => "OWN",}    }}

pub const VECTORS_COLLABS: &[Collab] = &[    Collab::PAYMENTS,    Collab::OLIVIA,    Collab::BABEFA,    Collab::OKLAND,    Collab::RED,    Collab::TETRIS,
                                             Collab::SCANDINAVIA, Collab::KUIB,      Collab::POLZ,      Collab::ZVEZD,     Collab::SKY,    Collab::OWN,];

pub const VECTORS_COLLABS_____: &[&str] = &[    OLIVIA,    BABEFA,    OKLAND,    RED,    TETRIS,    SCANDINAVIA,    KUIB,    POLZ,    ZVEZD,    SKY,    OWN,];


pub fn add(left: u64, right: u64) -> u64 {    left + right}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
