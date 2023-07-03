use std::time::Duration;

#[derive(Debug)]
pub enum BypassErr {
    MaxTriesErr,
    GetReqErr(reqwest::Error),
}

pub struct Bypasser {
    max_tries: u8,
    wait_time: Duration,
}

impl Bypasser {
    pub fn new() -> Self {
        Bypasser::default()
    }

    pub async fn get(&self, url: &str) -> Result<String, BypassErr> {
        let mut tries = 0;
        loop {
            if tries > self.max_tries {
                break Err(BypassErr::MaxTriesErr);
            };

            tries += 1;

            let res = self.try_get(url).await.map_err(BypassErr::GetReqErr)?;

            let challenged = regex::Regex::new(r#"id="challenge-form" action="([^"]*)""#)
                .unwrap()
                .captures(&res)
                .is_some();

            if challenged {
                tokio::time::sleep(self.wait_time).await;
                continue;
            } else {
                break Ok(res);
            };
        }
    }

    async fn try_get(&self, url: &str) -> Result<String, reqwest::Error> {
        let res = reqwest::Client::new()
            .get(url)
            .header(reqwest::header::USER_AGENT, fake_user_agent::get_rua())
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }
}

impl Default for Bypasser {
    fn default() -> Self {
        Bypasser {
            max_tries: 5,
            wait_time: Duration::from_secs(1),
        }
    }
}
