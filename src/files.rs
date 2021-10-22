use surf::Client;

pub struct FilesService {
  client: Client,
}

impl FilesService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn download(&self) {}

  pub fn get_directory(&self) {}

  pub fn get_file_details(&self) {}

  pub fn get_sources(&self) {}

  pub fn prepare_download(&self) {}

  pub fn set_file_details(&self) {}
}
