pub mod addons;
pub mod application;
pub mod audiolibrary;
pub mod favourites;
pub mod files;
pub mod gui;
pub mod input;
pub mod jsonrpc;
pub mod player;
pub mod playlist;
pub mod pvr;
pub mod settings;
pub mod system;
pub mod textures;
pub mod videolibrary;
pub mod xbmc;

use std::convert::TryInto;
use std::time::Duration;
use surf::Url;
use surf::{Client, Config};

pub struct Kodi {
  pub addons: addons::AddonsService,
  pub application: application::ApplicationService,
  pub audiolibrary: audiolibrary::AudioLibraryService,
  pub favourites: favourites::FavouritesService,
  pub files: files::FilesService,
  pub gui: gui::GuiService,
  pub input: input::InputService,
  pub jsonrpc: jsonrpc::JsonRpcService,
  pub player: player::PlayerService,
  pub playlist: playlist::PlaylistService,
  pub pvr: pvr::PvrService,
  pub settings: settings::SettingsService,
  pub textures: textures::TexturesService,
  pub system: system::SystemService,
  pub videolibrary: videolibrary::VideoLibraryService,
  pub xbmc: xbmc::XbmcService,
}

impl Kodi {
  pub fn new(base_url: String) -> Self {
    let client: Client = Config::new()
      .set_base_url(Url::parse(&base_url).unwrap())
      .set_timeout(Some(Duration::from_secs(5)))
      .try_into()
      .unwrap();
    Self {
      addons: addons::AddonsService::new(&client),
      application: application::ApplicationService::new(&client),
      audiolibrary: audiolibrary::AudioLibraryService::new(&client),
      favourites: favourites::FavouritesService::new(&client),
      files: files::FilesService::new(&client),
      gui: gui::GuiService::new(&client),
      input: input::InputService::new(&client),
      jsonrpc: jsonrpc::JsonRpcService::new(&client),
      player: player::PlayerService::new(&client),
      playlist: playlist::PlaylistService::new(&client),
      pvr: pvr::PvrService::new(&client),
      settings: settings::SettingsService::new(&client),
      system: system::SystemService::new(&client),
      textures: textures::TexturesService::new(&client),
      videolibrary: videolibrary::VideoLibraryService::new(&client),
      xbmc: xbmc::XbmcService::new(&client),
    }
  }
}
