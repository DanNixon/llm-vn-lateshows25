use icd::{ButtonAction, CharacterSelectScreen, ChoiceScreen, Screen};
use log::{debug, info};
use postcard_rpc::{
    header::VarSeqKind,
    host_client::HostClient,
    standard_icd::{PingEndpoint, WireError, ERROR_PATH},
};

pub struct Client {
    pub client: HostClient<WireError>,
}

impl Client {
    pub fn new() -> Self {
        info!("Connecting to USB device...");
        let client = HostClient::new_raw_nusb(
            |d| d.product_string() == Some("llm-vn-controller"),
            ERROR_PATH,
            8,
            VarSeqKind::Seq2,
        );
        info!("Connected");
        Self { client }
    }

    pub(crate) async fn ping(&self, id: u32) -> u32 {
        self.client.send_resp::<PingEndpoint>(&id).await.unwrap()
    }

    pub(crate) async fn wait_for_button_push(&self) -> ButtonAction {
        let mut sub = self
            .client
            .subscribe_exclusive::<icd::ButtonActionPerformed>(1)
            .await
            .unwrap();

        debug!("Waiting for button push");
        sub.recv().await.unwrap()
    }

    pub(crate) async fn show_character_select_screen(&self, screen: CharacterSelectScreen) {
        debug!("Showing character selection screen: {screen:?}");
        self.client
            .send_resp::<icd::SetDisplay>(&Screen::CharacterSelect(screen))
            .await
            .unwrap();
    }

    pub(crate) async fn show_choice_screen(&self, screen: ChoiceScreen) {
        debug!("Showing choice screen: {screen:?}");
        self.client
            .send_resp::<icd::SetDisplay>(&Screen::Choices(screen))
            .await
            .unwrap();
    }
}
