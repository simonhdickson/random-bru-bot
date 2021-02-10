use std::{fs::File, io::Write, path::PathBuf, str::FromStr, time::Duration};

use chrono::Local;
use cron::Schedule;
use log::info;
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};

use crate::{
    matrix::Bot,
    picker::{PersonSet, Pick},
};

#[derive(Clone)]
pub struct BruTimeJob {
    bot: Bot,
    cron: String,
    file_path: PathBuf,
}

impl BruTimeJob {
    pub fn new(bot: Bot, cron: String, file_path: PathBuf) -> BruTimeJob {
        BruTimeJob {
            bot,
            cron,
            file_path,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let schedule = Schedule::from_str(&self.cron)?;

        loop {
            let next = schedule.upcoming(Local).next().unwrap();

            info!("next run: {:#?}", next);

            loop {
                if next < Local::now() {
                    break;
                }

                tokio::time::sleep(Duration::from_secs(5)).await;
            }

            let people = self.load_dataset();

            let (picks, people) = people.make_selection();

            let mut message = "bru time!\n\n".to_owned();

            let mut count = 1;

            for pick in picks {
                match pick {
                    Pick::Pair(first, second) => {
                        message.push_str(&format!(
                            "{}. @{} and @{}\n",
                            count, first.name, second.name
                        ));
                    }
                    Pick::Triple(first, second, third) => {
                        message.push_str(&format!(
                            "{}. @{}, @{} and @{}\n",
                            count, first.name, second.name, third.name
                        ));
                    }
                }

                count += 1;
            }

            self.bot.send_message(message).await?;

            self.save_dataset(people);
        }
    }

    fn load_dataset(&self) -> PersonSet {
        File::open(&self.file_path)
            .ok()
            .and_then(|f| from_reader(f).ok())
            .unwrap_or_else(|| PersonSet::new())
    }

    fn save_dataset(&self, people: PersonSet) {
        let mut f = File::create(&self.file_path).expect("failed to save to file");

        let pretty = PrettyConfig::new();

        let s = to_string_pretty(&people, pretty).expect("failed to serialize data");

        f.write_all(s.as_bytes()).expect("msg");
    }
}
