use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS {
    allowed_origins: Vec<String>
}

impl CORS {
    pub fn init (origins: &Vec<&str>) -> CORS {
        CORS {allowed_origins: origins.iter().map(|s| s.to_string()).collect()}
    }
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        match request.headers().get_one("Origin") {
            Some(origin) => {
                match self.allowed_origins.iter().find(|s| s.to_owned() == &origin.to_string()) {
                    Some(o) => {
                        response.set_header(Header::new("Access-Control-Allow-Origin", o.to_owned()));
                    }
                    None=>{}
                }
            }
            None =>{}
        }
        
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
    }
}