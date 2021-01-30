use std::{str::FromStr, time::Duration};

use chrono::Local;
use cron::Schedule;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::matrix::Bot;

#[derive(Clone)]
pub struct BruTimeJob {
    bot: Bot,
    cron: String,
}

impl BruTimeJob {
    pub fn new(bot: Bot, cron: String) -> BruTimeJob {
        BruTimeJob { bot, cron }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let schedule = Schedule::from_str(&self.cron)?;

        loop {
            let next = schedule.upcoming(Local).next().unwrap();

            println!("next run: {:#?}", next);

            loop {
                if next < Local::now() {
                    break;
                }

                tokio::time::sleep(Duration::from_secs(5)).await;
            }

            let mut members = self.bot.get_members().await?;

            members.shuffle(&mut thread_rng());

            let mut message = "bru time!\n\n".to_owned();

            let mut count = 1;

            while members.len() > 0 {
                match members.len() {
                    1 => {
                        let first = members.pop().unwrap();

                        message.push_str(&format!("{}. {}", count, first));
                    }
                    3 => {
                        let first = members.pop().unwrap();
                        let second = members.pop().unwrap();
                        let third = members.pop().unwrap();

                        message
                            .push_str(&format!("{}. {}, {} and {}", count, first, second, third));
                    }
                    _ => {
                        let first = members.pop().unwrap();
                        let second = members.pop().unwrap();

                        message.push_str(&format!("{}. {} and {}", count, first, second));
                    }
                }

                count += 1;
            }

            self.bot.send_message(message).await?;
        }
    }
}
