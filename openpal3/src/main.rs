use opengb::{application::OpenGbApplication, config::OpenGbConfig};

fn main() {
    let config = OpenGbConfig::load("openpal3", "OPENPAL3");
    unsafe {
        let mut app = OpenGbApplication::create(&config, "OpenPAL3");
        app.initialize();
        app.run();
        app.run_event_loop();
    }
}
