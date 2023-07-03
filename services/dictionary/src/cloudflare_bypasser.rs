use std::time::Duration;

#[derive(Debug)]
pub enum BypassErr {
    MaxTriesErr,
    GetReqErr(reqwest::Error),
}

pub struct Bypasser {
    max_tries: u8,
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
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            } else {
                break Ok(res);
            };
        }
    }

    async fn try_get(&self, url: &str) -> Result<String, reqwest::Error> {
        let user_agent = fake_useragent::UserAgentsBuilder::new()
            .cache(false)
            .set_browsers(
                fake_useragent::Browsers::new()
                    .set_chrome()
                    .set_firefox()
                    .set_safari(),
            )
            .build()
            .random()
            .to_string();

        let res = reqwest::Client::new()
            .get(url)
            .header(reqwest::header::USER_AGENT, user_agent)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }
}

impl Default for Bypasser {
    fn default() -> Self {
        Bypasser { max_tries: 5 }
    }
}
