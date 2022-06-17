use lucian::app::App;
use lucian::hub::line::LineType;

const CALLER_NUM: u8 = 30;
const LISTEN_ADDR:&str = "127.0.0.1:1080";
const PROXY_SERVER:&str = "8.218.15.102:3389";

fn main() {
    let mut app = App::new(LISTEN_ADDR,LineType::Fox);
    app.init(CALLER_NUM,PROXY_SERVER);
    app.start();
}
