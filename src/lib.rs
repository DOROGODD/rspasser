use std::collections::HashMap;

pub struct RsPasser {
    client: reqwest::Client,
}

impl RsPasser {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder().build().unwrap(),
        }
    }

    async fn get_recaptcha_token(&self, _type: String, params: String) -> anyhow::Result<String> {
        let url = format!("https://www.google.com/recaptcha/{_type}/anchor?{params}");
        let body = self
            .client
            .get(url)
            .header("content-type", "application/x-www-form-urlencoded")
            .send()
            .await?
            .text()
            .await?;

        Ok(body
            .split("id=\"recaptcha-token\" value=\"")
            .last()
            .unwrap()
            .split('\"')
            .next()
            .unwrap()
            .to_string())
    }

    async fn get_recaptcha_response(
        &self,
        _type: String,
        post_data: String,
        site_key: String,
    ) -> anyhow::Result<String> {
        let data = self
            .client
            .post(format!(
                "https://www.google.com/recaptcha/{_type}/reload?k={site_key}",
            ))
            .header("content-type", "application/x-www-form-urlencoded")
            .body(post_data)
            .send()
            .await?
            .text()
            .await?;

        Ok(data
            .split("resp\",\"")
            .last()
            .unwrap()
            .split('\"')
            .next()
            .unwrap()
            .to_string())
    }

    pub async fn solve_captcha(&self, anchor_url: String) -> anyhow::Result<String> {
        let type_ = if anchor_url.contains("api2") {
            "api2"
        } else {
            "enterprise"
        };

        let params = anchor_url.split("?").last().unwrap();
        let token = self
            .get_recaptcha_token(type_.to_string(), params.to_string())
            .await?;

        let params = params
            .split('&')
            .map(|param| {
                let args = param.split('=').collect::<Vec<&str>>();

                (args[0].to_string(), args[1].to_string())
            })
            .collect::<HashMap<String, String>>();
        let post_data = format!(
            "v={}&reason=q&c={}&k={}&co={}",
            params["v"], token, params["k"], params["co"]
        );

        self.get_recaptcha_response(type_.to_string(), post_data, params["k"].to_string())
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::RsPasser;

    #[tokio::test]
    async fn test_captcha_solving() {
        assert!(RsPasser::new().solve_captcha("https://www.google.com/recaptcha/api2/anchor?ar=1&k=6Leqr00oAAAAAN3ItHtrGkMpHiOtENMkG87lq2fq&co=aHR0cHM6Ly9wcmVyZWdpc3Rlci5oeXRvcGlhLmNvbTo0NDM.&hl=ru&type=image&v=Ai7lOI0zKMDPHxlv62g7oMoJ&theme=dark&size=invisible&badge=bottomright&cb=bmwunnu5dq8d".to_string()).await.unwrap().is_ascii());
    }
}
