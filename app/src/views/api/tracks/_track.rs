use ::models::track::Track;

#[derive(Debug, Deserialize, Serialize)]
pub struct RenderedTrack<'a> {
    pub id: i32,
    pub length: Option<i32>,
    pub link: &'a str,
    pub name: Option<&'a str>,
}

pub fn format(track: &Track) -> RenderedTrack {
    let name = match track.name {
        Some(ref track_name) => Some(track_name.as_str()),
        None => None,
    };
    RenderedTrack {
        id: track.id,
        length: track.length,
        link: &track.link,
        name: name,
    }
}