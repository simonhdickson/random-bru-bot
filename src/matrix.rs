use matrix_sdk::{
    self,
    api::r0::room::{create_room, Visibility},
    events::{room::message::MessageEventContent, AnyMessageEventContent},
    Client, ClientConfig, SyncSettings,
};
use regex::Regex;
use url::Url;

#[derive(Clone)]
pub struct Bot {
    client: Client,
    username: String,
    room_name: String,
    ignored_members: Regex,
}

impl Bot {
    pub async fn new(
        homeserver_url: String,
        username: String,
        password: String,
        room_name: String,
        ignored_members: String,
    ) -> Result<Bot, Box<dyn std::error::Error>> {
        let ignored_members = Regex::new(&ignored_members)?;

        let mut home = dirs::home_dir().expect("no home directory found");
        home.push("bru_bot");

        let client_config = ClientConfig::new().store_path(home);

        let homeserver_url =
            Url::parse(&homeserver_url).expect("Couldn't parse the homeserver URL");

        let client = Client::new_with_config(homeserver_url, client_config)?;

        client.login(&username, &password, None, None).await?;

        Ok(Self {
            client,
            username,
            room_name,
            ignored_members,
        })
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self
            .client
            .joined_rooms()
            .iter()
            .any(|r| Some(self.room_name.to_owned()) == r.name())
        {
            let mut request = create_room::Request::default();
            request.name = Some(&self.room_name);
            request.visibility = Visibility::Public;
            self.client.create_room(request).await?;
        }

        self.client.sync_once(SyncSettings::default()).await?;

        let settings = SyncSettings::default().token(self.client.sync_token().await.unwrap());

        self.client.sync(settings).await;

        Ok(())
    }

    pub async fn send_message(&self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        let joined_rooms = self.client.joined_rooms();

        let room = joined_rooms
            .iter()
            .find(|r| r.name() == Some(self.room_name.to_owned()))
            .unwrap();

        let content = AnyMessageEventContent::RoomMessage(MessageEventContent::text_plain(message));

        self.client.room_send(room.room_id(), content, None).await?;

        Ok(())
    }

    pub async fn get_members(&self) -> Result<Vec<Member>, Box<dyn std::error::Error>> {
        let joined_rooms = self.client.joined_rooms();

        let room = joined_rooms
            .iter()
            .find(|r| r.name() == Some(self.room_name.to_owned()))
            .unwrap();

        let members = room.joined_members().await?;

        let members = members
            .iter()
            .map(|m| Member {
                user_id: m.user_id().localpart().to_owned(),
                friendly_name: m.name().to_owned(),
            })
            .filter(|r| *r.user_id != self.username)
            .filter(|r| !self.ignored_members.is_match(&r.user_id))
            .collect::<Vec<_>>();

        Ok(members)
    }
}

pub struct Member {
    pub user_id: String,
    pub friendly_name: String,
}
