use matrix_sdk::{
    self,
    api::r0::room::{create_room, Visibility},
    events::{room::message::MessageEventContent, AnyMessageEventContent},
    Client, ClientConfig, SyncSettings,
};
use url::Url;

#[derive(Clone)]
pub struct Bot {
    client: Client,
    username: String,
}

impl Bot {
    pub async fn new(
        homeserver_url: String,
        username: String,
        password: String,
    ) -> Result<Bot, Box<dyn std::error::Error>> {
        let mut home = dirs::home_dir().expect("no home directory found");
        home.push("party_bot");

        let client_config = ClientConfig::new().store_path(home);

        let homeserver_url =
            Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");

        let client = Client::new_with_config(homeserver_url, client_config)?;

        client
            .login(&username, &password, None, Some("command bot"))
            .await?;

        Ok(Self { client, username })
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self
            .client
            .joined_rooms()
            .iter()
            .any(|r| Some("Random Bru".to_owned()) == r.name())
        {
            let mut request = create_room::Request::default();
            request.name = Some("Random Bru");
            request.visibility = Visibility::Public;
            self.client.create_room(request).await?;
        }

        self.client.sync_once(SyncSettings::default()).await?;

        let settings = SyncSettings::default().token(self.client.sync_token().await.unwrap());

        self.client.sync(settings).await;

        Ok(())
    }

    pub async fn send_message(&self, message: String) -> Result<(), matrix_sdk::Error> {
        let joined_rooms = self.client.joined_rooms();

        let room = joined_rooms
            .iter()
            .find(|r| r.name() == Some("Random Bru".to_owned()))
            .unwrap();

        let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(message));

        self.client.room_send(room.room_id(), content, None).await?;

        Ok(())
    }

    pub async fn get_members(&self) -> Result<Vec<String>, matrix_sdk::Error> {
        let joined_rooms = self.client.joined_rooms();

        let room = joined_rooms
            .iter()
            .find(|r| r.name() == Some("Random Bru".to_owned()))
            .unwrap();

        let members = room.joined_members().await?;

        let members = members
            .iter()
            .map(|m| m.name().to_owned())
            .filter(|r| *r != self.username)
            .collect::<Vec<_>>();

        Ok(members)
    }
}
