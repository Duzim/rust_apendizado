fn main() {
    handle_http_resp(HttpResponse::JsonData {
        status: 200,
        body: String::from("{ \"aaa\": \"aaaa\"}"),
    })
}

enum HttpResponse {
    OK,
    Redirect(String),
    ErrorClient(u16, String),
    JsonData { status: u16, body: String },
}

fn handle_http_resp(res: HttpResponse) {
    match res {
        HttpResponse::OK => {
            println!("OK");
        }
        HttpResponse::Redirect(url) => {
            println!("{:?}", url);
        }
        HttpResponse::ErrorClient(status, err) => {
            println!("stts: {}, err: {}", status, err);
        }
        HttpResponse::JsonData { status, body } => {
            println!("stts: {:?}, body: {:?}", status, body);
        }
    }
}
