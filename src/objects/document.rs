use super::*;

/// <https://vk.com/dev/objects/doc>
#[derive(Deserialize, Clone, Debug)]
pub struct Document {
    pub id: Integer,
    pub owner_id: Integer,
    pub title: String,
    pub size: Integer,
    pub ext: String,
    pub url: String,
    pub date: Integer,

    #[serde(rename = "type")]
    pub type_: Integer,

    pub preview: Option<DocumentPreview>,

    /// Access key may be present in attachments
    /// (
    /// <https://vk.com/dev/objects/attachments_w>
    /// or
    /// <https://vk.com/dev/objects/attachments_m>
    /// )
    pub access_key: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DocumentPreview {
    pub photo: Option<Photo>,
    pub graffiti: Option<Graffiti>,
    pub audio_msg: Option<AudioMessage>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Photo {
    pub sizes: Vec<photo::Size>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Graffiti {
    pub src: String,
    pub width: Integer,
    pub height: Integer,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AudioMessage {
    pub duration: Integer,
    pub waveform: Vec<Integer>,
    pub link_ogg: String,
    pub link_mp3: String,
}

#[derive(Debug, PartialEq)]
// тип документа. Возможные значения: https://vk.com/dev/objects/doc
pub enum DocumentType {
    // 1 - текстовые документы
    Text = 1,
    // 2 - архивы
    Archive = 2,
    // 3 — gif
    Gif = 3,
    // 4 — изображения
    Image = 4,
    // 5 — аудио
    Audio = 5,
    // 6 — видео
    Video = 6,
    // 7 — электронные книги
    Ebook = 7,
    // 8 — неизвестно
    Other = 8,
}

impl From<Integer> for DocumentType {
    fn from(val: Integer) -> Self {
        match val {
            1 => DocumentType::Text,
            2 => DocumentType::Archive,
            3 => DocumentType::Gif,
            4 => DocumentType::Image,
            5 => DocumentType::Audio,
            6 => DocumentType::Video,
            7 => DocumentType::Ebook,
            _ => DocumentType::Other,
        }
    }
}

#[test]
fn test_document_type() {
    assert_eq!(1, DocumentType::Text as Integer);
    assert_eq!(DocumentType::Archive, 2.into());
}
