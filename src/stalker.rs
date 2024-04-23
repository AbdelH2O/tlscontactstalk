use reqwest::header;
use std::{thread, time};

pub struct Stalker {
    // reqwest instance
    client: reqwest::blocking::Client,
    username: String,
    password: String,
    group: String,
    frequency: u64,
}

impl Stalker {
    pub fn new(username: String, password: String, group: String, frequency: u64) -> Stalker {
        let policy = reqwest::redirect::Policy::custom(|attempt| {
            //if attempt.previous().len() > 100 {
            //    attempt.error("too many redirects")
            //} else {
            //   attempt.follow()
            //}
            println!("Attempt number: {:?}", attempt.previous().len());
            attempt.follow()
        });
        Stalker {
            client: reqwest::blocking::Client::builder().redirect(policy).build().unwrap(),
            username,
            password,
            group,
            frequency,
        }
    }

    pub fn login(&self) -> Result<String, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
        headers.insert("Connection", "keep-alive".parse().unwrap());
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
        //headers.insert(header::COOKIE, "AUTH_SESSION_ID=e6a8ff90-c834-4711-81c2-c2dfc2a4c07e.keycloak-79777df9f4-cmpxw; AUTH_SESSION_ID_LEGACY=e6a8ff90-c834-4711-81c2-c2dfc2a4c07e.keycloak-79777df9f4-cmpxw; KC_RESTART=eyJhbGciOiJIUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJkMTM5MDAyNC0wNTc0LTRkMzMtOWYwNC0zY2Q2NTFkMDE5YjgifQ.eyJjaWQiOiJ3ZWJfYXBwIiwicHR5Ijoib3BlbmlkLWNvbm5lY3QiLCJydXJpIjoiaHR0cHM6Ly92aXNhcy1kZS50bHNjb250YWN0LmNvbS9sb2dpbi9vYXV0aDIvY29kZS9vaWRjIiwiYWN0IjoiQVVUSEVOVElDQVRFIiwibm90ZXMiOnsic2NvcGUiOiJvcGVuaWQgcm9sZXMgYXRsYXMgd2ViLW9yaWdpbnMgZW1haWwgb2ZmbGluZV9hY2Nlc3MgcHJvZmlsZSBhZGRyZXNzIHBob25lIiwiY2xpZW50X3JlcXVlc3RfcGFyYW1faXNzdWVyIjoibWFSQkEyZGUiLCJpc3MiOiJodHRwczovL2F1dGgudmlzYXMtZGUudGxzY29udGFjdC5jb20vYXV0aC9yZWFsbXMvYXRsYXMiLCJyZXNwb25zZV90eXBlIjoiY29kZSIsInJlZGlyZWN0X3VyaSI6Imh0dHBzOi8vdmlzYXMtZGUudGxzY29udGFjdC5jb20vbG9naW4vb2F1dGgyL2NvZGUvb2lkYyIsInN0YXRlIjoiQ2U1cGJoaHQ5ZExUUUl4eTRTN3drWkJIYVF4QVctcTJ0UlFLRHIxakM0ST0ifX0.Bl_XD8M2v6YDU1y9EUjcSRkb7Jt_yxS1GDp0ajdO7Mo; uid=CltkGWYaZ4x04nbgBQppAg==; osano_consentmanager_uuid=0d75a56e-1782-40b1-b1af-cdb7a9836a50; osano_consentmanager=cIPtzZkEUNnRQHkQRiMQ1cH6SSTbrKHxeWNJe-Dm_Ss8z-GuaOKjZ_3pUQ4az1IEsA_30gI5958NyGow_bUN393cYIpYIv92I6ZBiVahrcrq0Ir3Gf4D6dbqoHXS-78d4YOztuFWLDHZ-BczqbAHwb_vmmA7Z6BR351hUmYg7YYNMfsbVdIGlWL7i2PYITElvFzcn-2LXNeXloS_HCP32F8SyAW0VV_iiDykfHt7Fz0e6xLMGdcDjNqDNG7RORkl08-EakmyibAQmoTM4x9I7F9OeeYiSh8wbNvUJQ==; __cf_bm=rzf5wgnYo9Ag_xpB1GXwqs.lPO8gwk5ssLP1DW9zk7s-1713867064-1.0.1.1-dOnwS7OB3LTTBzxHf4voxux7Do5fW5owkO32RDgHeWqJOy7amLe7yLRpDbCrYfLFvIGuSFleuYZY3Fcp8kw9Wg".parse().unwrap());
        headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
        headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
        headers.insert("Sec-Fetch-User", "?1".parse().unwrap());
        headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());

        let login_url = "https://auth.visas-de.tlscontact.com/auth/realms/atlas/login-actions/authenticate?session_code=74SooVJthLYRUx6xjtugxIkI5f2Fu863ensa87PB5ag&execution=6688271c-2703-46f7-ad03-c6d1b3ea41d1&client_id=web_app&tab_id=ozb0_N24Bd8";
        let res = self.client.post(login_url)
            .headers(headers)
            .body(format!("username={}&password={}", self.username, self.password))
            .send()?
            .text()?;
        Ok(res)
    }

   

    pub fn run(&self) {
        loop {
            self.login().unwrap();
            // self.get_group().unwrap();
            thread::sleep(time::Duration::from_secs(self.frequency));
        }
    }
}
