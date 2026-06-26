use futures_util::stream::StreamExt;
use swayipc_async::{Connection, EventType, Fallible, Event, WindowChange};
use serde::Deserialize;
use directories::ProjectDirs;
use std::fs;


#[derive(Deserialize)]
struct Config {
    default_opacity: f32,
    target_opacity: f32
}


#[tokio::main]
async fn main() -> Fallible<()> {

    let mut default_opacity: f32 = 0.85;
    let mut target_opacity: f32 = 1.0;

    if let Some(project_dirs) = ProjectDirs::from("", "", "swayopacity") {
        let config_dir = project_dirs.config_dir();
        let config_path = config_dir.join("config.toml");

        if config_path.exists() {
            if let Ok(config_text) = fs::read_to_string(&config_path) {
                if let Ok(config)= toml::from_str::<Config>(&config_text) {
                    default_opacity = config.default_opacity;
                    target_opacity = config.target_opacity;
                }
            }
        } else {
            let default_toml = format!(
            "default_opacity = {}\ntarget_opacity = {}",
            default_opacity, target_opacity
            );
            let _ = fs::create_dir_all(config_dir);
            let _ = fs::write(config_path, default_toml);
        }
    }

    let subs = [
        EventType::Window,
    ];

    let mut events = Connection::new().await?.subscribe(subs).await?;
    let mut fader = Connection::new().await?;

    while let Some(event) = events.next().await {

        if let Event::Window(window_event) = event? {
            if window_event.change == WindowChange::Focus {
                 let container_id = window_event.container.id;
                fader.run_command(format!(r#"[app_id=".*"] opacity {}; [con_id={}] opacity {}"#, default_opacity, container_id, target_opacity)).await?;
            }
        }
    }
    Ok(())
}
