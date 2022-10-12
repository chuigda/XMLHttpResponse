use serde::Deserialize;

#[derive(Deserialize)]
pub struct MimeType {
    pub ext: String,
    pub mime: String
}

#[derive(Deserialize)]
pub struct XmlHttpConfig {
    pub socket_addr: String,
    pub work_dir: String,
    pub mimes: Vec<MimeType>
}
