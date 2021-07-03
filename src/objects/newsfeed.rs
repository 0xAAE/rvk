use super::*;

#[derive(Deserialize, Clone, Debug)]
pub struct NewsFeed {
    // массив новостей для текущего пользователя
    pub items: Option<Vec<Item>>,
    // информация о пользователях (<https://vk.com/dev/objects/user>), которые находятся в списке новостей
    pub profiles: Option<Vec<user::User>>,
    // содержит массив объектов сообществ (<https://vk.com/dev/objects/groups>), которые присутствуют в новостях
    pub groups: Option<Vec<group::Group>>,
    // offset, который необходимо передать, для того, чтобы получить следующую часть новостей (в более старых версиях API)
    pub new_offset: Option<Integer>,
    // start_from, который необходимо передать, для того, чтобы получить следующую часть новостей.
    // Позволяет избавиться от дубликатов, которые могут возникнуть при появлении новых новостей между вызовами этого метода.
    pub next_from: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Item {
    // тип списка новости, соответствующий одному из значений параметра filters
    #[serde(rename = "type")]
    pub type_: String,
    // идентификатор источника новости (положительный — новость пользователя, отрицательный — новость группы)
    pub source_id: Integer,
    // время публикации новости в формате unixtime
    pub date: Integer,
    // находится в записях со стен и содержит идентификатор записи на стене владельца
    pub post_id: Option<Integer>,
    // находится в записях со стен, содержит тип новости (post или copy)
    pub post_type: Option<String>,
    // передается в случае, если этот пост сделан при удалении
    pub final_post: Option<String>,
    // находится в записях со стен, если сообщение является копией сообщения с чужой стены,
    // и содержит идентификатор владельца стены, у которого было скопировано сообщение
    pub copy_owner_id: Option<Integer>,
    // находится в записях со стен, если сообщение является копией сообщения с чужой стены,
    // и содержит идентификатор скопированного сообщения на стене его владельца
    pub copy_post_id: Option<String>,
    // массив, содержащий историю репостов для записи. Возвращается только в том случае,
    // если запись является репостом. Каждый из объектов массива, в свою очередь,
    // является объектом-записью стандартного формата (wtf?)
    pub copy_history: Option<Vec<HistoryItem>>,
    // находится в записях со стен, если сообщение является копией сообщения с чужой стены,
    // и содержит дату скопированного сообщения
    pub copy_post_date: Option<String>,
    // находится в записях со стен и содержит текст записи
    pub text: Option<String>,
    // содержит 1, если текущий пользователь может редактировать запись
    pub can_edit: Option<Integer>,
    // возвращается, если пользователь может удалить новость, всегда содержит 1
    pub can_delete: Option<Integer>,
    // находится в записях со стен и содержит информацию о комментариях к записи,
    pub comments: Option<post::Comments>,
    //  находится в записях со стен и содержит информацию о числе людей, которым понравилась данная запись
    pub likes: Option<post::Likes>,
    // находится в записях со стен и содержит информацию о числе людей, которые скопировали данную запись на свою страницу
    pub reposts: Option<post::Reposts>,
    // находится в записях со стен и содержит массив объектов, которые прикреплены к текущей новости (фотография, ссылка и т.п.).
    // Более подробная информация представлена на странице <https://vk.com/dev/objects/attachments_w>
    pub attachments: Option<Vec<NewsAttachment>>,
    // geo — находится в записях со стен, в которых имеется информация о местоположении
    pub geo: Option<geo::Geo>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub photos: Option<PhotoSet>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub photo_tags: Option<PhotoTags>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub notes: Option<NoteSet>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub friends: Option<FriendSet>,
}

/// undocumented, differs from WallAttachment <https://vk.com/dev/objects/attachments_w> by album
/// which does not equal to album::Album (id: String)
#[derive(Deserialize, Clone, Debug)]
pub struct NewsAttachment {
    #[serde(rename = "type")]
    pub type_: String,

    // type = photo
    pub photo: Option<photo::Photo>,

    // type = posted_photo
    pub posted_photo: Option<attachment::PostedPhoto>,

    // type = video
    pub video: Option<video::Video>,

    // type = audio
    pub audio: Option<audio::Audio>,

    // type = doc
    pub doc: Option<document::Document>,

    // type = graffiti
    pub graffiti: Option<attachment::Graffiti>,

    // type = link
    pub link: Option<link::Link>,

    // type = note
    pub note: Option<note::Note>,

    // type = app
    pub app: Option<attachment::App>,

    // type = poll
    pub poll: Option<poll::Poll>,

    // type = page
    pub page: Option<page::Page>,

    // type = album
    pub album: Option<photo::Album>,

    // type = photos_list
    pub photos_list: Option<Vec<String>>,

    // type = market
    pub market: Option<market_item::MarketItem>,

    // type = market_album
    pub market_album: Option<market_album::MarketAlbum>,

    // type = sticker
    pub sticker: Option<sticker::Sticker>,

    // type = pretty_cards
    pub cards: Option<Vec<attachment::Card>>,

    // type = event
    pub event: Option<attachment::Event>,

    // type = podcast
    pub podcast: Option<podcast::Podcast>,
}

// specific for newsfeed types

#[derive(Deserialize, Debug, Clone)]
pub struct PhotoSet {
    // информация о количестве объектов
    pub count: i64,
    // и до 5 последних объектов, связанных с данной новостью
    pub items: Option<Vec<photo::Photo>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhotoTags {
    // информация о количестве объектов
    pub count: i64,
    // и до 5 последних объектов, связанных с данной новостью
    //pub items: Option<Vec<?>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NoteSet {
    // информация о количестве объектов
    pub count: i64,
    // и до 5 последних объектов, связанных с данной новостью
    pub items: Option<Vec<note::Note>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FriendItem {
    pub user_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FriendSet {
    // информация о количестве объектов
    pub count: i64,
    // и до 5 последних объектов, связанных с данной новостью
    pub items: Option<Vec<FriendItem>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HistoryItem {
    pub date: u64,
    pub from_id: i64,
    pub id: i64,
    pub owner_id: i64,
    // находится в записях со стен и содержит массив объектов, которые прикреплены к текущей новости (фотография, ссылка и т.п.).
    // Более подробная информация представлена на странице <https://vk.com/dev/objects/attachments_w>
    pub attachments: Option<Vec<NewsAttachment>>,
    // тип
    pub post_type: Option<String>,
    // находится в записях со стен и содержит текст записи
    pub text: Option<String>,
    // source
    pub post_source: Option<post_source::PostSource>,
}
