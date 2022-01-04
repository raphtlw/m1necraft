mod config {
    use serde::{Deserialize, Serialize};
    use std::{fs, io};

    #[derive(Serialize, Deserialize, Default)]
    pub struct Config {
        pub minecraft_creds: MinecraftCredentials,
        pub setup_complete: bool,
    }

    impl Config {
        pub fn write(config: Self) -> Result<(), io::Error> {
            fs::write(
                crate::CONFIG_PATH.get().unwrap(),
                serde_json::to_string_pretty(&config).unwrap(),
            )?;

            Ok(())
        }

        pub fn read() -> Result<Self, Box<dyn std::error::Error>> {
            let config_file = fs::read_to_string(crate::CONFIG_PATH.get().unwrap())?;
            let config: Self = serde_json::from_str(&config_file)?;

            Ok(config)
        }
    }

    #[derive(Serialize, Deserialize, Default)]
    pub struct MinecraftCredentials {
        pub username: String,
        pub password: String,
    }
}

mod launcher {
    use std::error::Error;
    use std::io::{Write, self};
    use std::ops::Deref;
    use std::path::PathBuf;
    use std::process::Stdio;
    use std::{env, fs, process};

    use reqwest::IntoUrl;
    use serde_json::{Value, json};
    use indoc::formatdoc;

    use crate::{MC_LIBS_PATH, APP_DATA_DIR};
    use crate::{Message};
    use crate::config::{MinecraftCredentials, Config};

    pub async fn download_file<T>(url: T, custom_file_name: Option<&str>) -> Result<(), Box<dyn Error>>
    where
        T: IntoUrl + Clone + Copy,
    {
        log::info!("Downloading from URL: {}", url.as_str());
        
        let file_blob = reqwest::get(url).await?.bytes().await?;
        let file_name = if let Some(value) = custom_file_name {
            value
        } else {
            url.as_str().split("/").last().unwrap()
        };
        
        let mut file_out_path = env::current_dir()?;
        file_out_path.push(file_name);
        
        log::info!("Writing file at path: {:#?}", file_out_path);

        fs::write(file_out_path, file_blob)?;

        Ok(())
    }

    /// Downloads the libraries needed for Minecraft to run and downloads
    /// Minecraft itself.
    /// 
    /// During execution, it will move into the folder that contains the
    /// minecraft "libraries" folder.
    pub async fn download_mc_libraries() -> Result<(), Box<dyn Error>> {
        let cwd = env::current_dir()?;
        env::set_current_dir({
            let mut p = crate::MC_LIBS_PATH.get().unwrap().clone();
            p.push("libraries");
            p
        })?;
        
        Message::dispatch(Message::SetSetupProgress(Some(String::from("Downloading Minecraft libraries")), 0.1));
        
        if let Err(_) = futures::try_join!(
                // Specify library urls here
                
                download_file("https://launcher.mojang.com/v1/objects/1952d94a0784e7abda230aae6a1e8fc0522dba99/client.jar", Some("minecraft-1.16.4-client.jar")),
                download_file("https://libraries.minecraft.net/com/mojang/patchy/1.1/patchy-1.1.jar", None),
                download_file("https://libraries.minecraft.net/oshi-project/oshi-core/1.1/oshi-core-1.1.jar", None),
                download_file("https://libraries.minecraft.net/net/java/dev/jna/jna/4.4.0/jna-4.4.0.jar", None),
                download_file("https://libraries.minecraft.net/net/java/dev/jna/platform/3.4.0/platform-3.4.0.jar", None),
                download_file("https://libraries.minecraft.net/com/ibm/icu/icu4j/66.1/icu4j-66.1.jar", None),
                download_file("https://libraries.minecraft.net/com/mojang/javabridge/1.0.22/javabridge-1.0.22.jar", None),
                download_file("https://libraries.minecraft.net/net/sf/jopt-simple/jopt-simple/5.0.3/jopt-simple-5.0.3.jar", None),
                download_file("https://libraries.minecraft.net/io/netty/netty-all/4.1.25.Final/netty-all-4.1.25.Final.jar", None),
                download_file("https://libraries.minecraft.net/com/google/guava/guava/21.0/guava-21.0.jar", None),
                download_file("https://libraries.minecraft.net/org/apache/commons/commons-lang3/3.5/commons-lang3-3.5.jar", None),
                download_file("https://libraries.minecraft.net/commons-io/commons-io/2.5/commons-io-2.5.jar", None),
                download_file("https://libraries.minecraft.net/commons-codec/commons-codec/1.10/commons-codec-1.10.jar", None),
                download_file("https://libraries.minecraft.net/com/mojang/brigadier/1.0.17/brigadier-1.0.17.jar", None),
                download_file("https://libraries.minecraft.net/com/mojang/datafixerupper/4.0.26/datafixerupper-4.0.26.jar", None),
                download_file("https://libraries.minecraft.net/com/google/code/gson/gson/2.8.0/gson-2.8.0.jar", None),
                download_file("https://libraries.minecraft.net/com/mojang/authlib/2.0.27/authlib-2.0.27.jar", None),
                download_file("https://libraries.minecraft.net/org/apache/commons/commons-compress/1.8.1/commons-compress-1.8.1.jar", None),
                download_file("https://libraries.minecraft.net/org/apache/httpcomponents/httpclient/4.3.3/httpclient-4.3.3.jar", None),
                download_file("https://libraries.minecraft.net/commons-logging/commons-logging/1.1.3/commons-logging-1.1.3.jar", None),
                download_file("https://libraries.minecraft.net/org/apache/httpcomponents/httpcore/4.3.2/httpcore-4.3.2.jar", None),
                download_file("https://libraries.minecraft.net/it/unimi/dsi/fastutil/8.2.1/fastutil-8.2.1.jar", None),
                download_file("https://libraries.minecraft.net/org/apache/logging/log4j/log4j-api/2.8.1/log4j-api-2.8.1.jar", None),
                download_file("https://libraries.minecraft.net/org/apache/logging/log4j/log4j-core/2.8.1/log4j-core-2.8.1.jar", None),
                download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3.jar", None),
                download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3.jar", None),
                download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-natives-linux.jar", None),
                download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-natives-windows.jar", None),
                download_file("https://libraries.minecraft.net/com/mojang/text2speech/1.11.3/text2speech-1.11.3-sources.jar", None),
                download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0.jar", None),
                download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0-javadoc.jar", None),
                download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0-natives-osx.jar", None),
                download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0-sources.jar", None),
                download_file("https://libraries.minecraft.net/ca/weblite/java-objc-bridge/1.0.0/java-objc-bridge-1.0.0.jar", None),
                download_file("https://launcher.mojang.com/v1/objects/1952d94a0784e7abda230aae6a1e8fc0522dba99/client.jar", None),
            )
        {
            log::error!("Downloading libraries failed");
            process::exit(1);
        }
        
        Message::dispatch(Message::SetSetupProgress(None, 1.0));
        
        env::set_current_dir(cwd)?;

        Ok(())
    }

    #[derive(Debug)]
    pub enum AssetObjectVecType {
        Num(usize),
        Object(Value),
    }

    /// Downloads all assets required for Minecraft to run, like objects, textures,
    /// etc.
    pub async fn download_mc_assets() -> Result<(), Box<dyn Error>> {
        let cwd = env::current_dir().unwrap();
        env::set_current_dir(crate::MC_LIBS_PATH.get().unwrap().clone()).unwrap();
        
        Message::dispatch(Message::SetSetupProgress(Some(String::from("Downloading Minecraft assets")), 0.1));
        
        let mut index_path = env::current_dir()?;
        index_path.push("assets");
        index_path.push("indexes");
        index_path.push("1.16.json");
        let index = reqwest::get("https://launchermeta.mojang.com/v1/packages/f8e11ca03b475dd655755b945334c7a0ac2c3b43/1.16.json").await?.bytes().await?;
        fs::write(index_path, &index)?;
        
        Message::dispatch(Message::SetSetupProgress(None, 0.3));
        
        let assets: Value = serde_json::from_str(&String::from_utf8_lossy(index.deref()))?;
        let obj = &assets["objects"];
        let mut o: Vec<Vec<AssetObjectVecType>> = Vec::new();
        use self::AssetObjectVecType::*;
        for (num, key) in obj.as_object().unwrap().keys().enumerate() {
            o.push(vec![Num(num), Object(obj.get(key).unwrap().to_owned())]);
        };
        
        for asset_obj_info in o {
            log::info!("o[0]: {:#?}", asset_obj_info[0]);
            
            match &asset_obj_info[1] {
                Object(obj) => {
                    let h = obj.get("hash").unwrap();
                    let filename = format!("{}/{}", &h.as_str().unwrap()[..2], &h);
                    let dirname = format!("assets/objects/{}", filename);
                    let url = format!("https://resources.download.minecraft.net/{}", filename);
                    fs::create_dir_all(PathBuf::from(&dirname).parent().unwrap())?;
                    fs::write(&dirname, reqwest::get(url).await?.bytes().await?)?;
                }
                _ => panic!("Unable to download assets")
            }
        }
        
        Message::dispatch(Message::SetSetupProgress(None, 1.0));
        
        env::set_current_dir(cwd).unwrap();
        
        Ok(())
    }

    /// Launches Minecraft. Only works for version 1.16.4 for now.
    /// Should be called only after login credentials have been saved,
    /// because this function will authenticate with Minecraft servers.
    /// 
    /// TODO: This probably does not work, should fix.
    pub async fn launch() -> Result<(), Box<dyn Error>> {
        let _cwd = env::current_dir()?;
        env::set_current_dir(MC_LIBS_PATH.get().unwrap().clone())?;
        
        let mc_libs_dir = MC_LIBS_PATH.get().unwrap().clone();
        let config = Config::read().unwrap();
        let auth_data = authenticate(config.minecraft_creds).await?;
        
        let launch_script = formatdoc! {r#"
            mainClass net.minecraft.client.main.Main
            param --version
            param MultiMC5
            param --assetIndex
            param 1.16
            param --userType
            param mojang
            param --versionType
            param release
            windowTitle MultiMC: Working
            windowParams 854x480
            traits XR:Initial
            traits FirstThreadOnMacOS
            launcher onesix
            param --gameDir
            param {game_dir}
            param --assetsDir
            param {assets_dir}
            param --accessToken
            param {access_token}
            sessionId token:{access_token}
            param --username
            param {username}
            userName {username}
            param --uuid
            param {uuid}
            cp {libraries_dir}/lwjgljars.jar
            cp {libraries_dir}/patchy-1.1.jar
            cp {libraries_dir}/project/oshi-core/1.1/oshi-core-1.1.jar
            cp {libraries_dir}/jna-4.4.0.jar
            cp {libraries_dir}/platform-3.4.0.jar
            cp {libraries_dir}/icu4j-66.1.jar
            cp {libraries_dir}/javabridge-1.0.22.jar
            cp {libraries_dir}/jopt-simple-5.0.3.jar
            cp {libraries_dir}/netty-all-4.1.25.Final.jar
            cp {libraries_dir}/guava-21.0.jar
            cp {libraries_dir}/commons-lang3-3.5.jar
            cp {libraries_dir}/commons-io-2.5.jar
            cp {libraries_dir}/commons-codec-1.10.jar
            cp {libraries_dir}/brigadier-1.0.17.jar
            cp {libraries_dir}/datafixerupper-4.0.26.jar
            cp {libraries_dir}/gson-2.8.0.jar
            cp {libraries_dir}/authlib-2.0.27.jar
            cp {libraries_dir}/commons-compress-1.8.1.jar
            cp {libraries_dir}/httpclient-4.3.3.jar
            cp {libraries_dir}/commons-logging-1.1.3.jar
            cp {libraries_dir}/httpcore-4.3.2.jar
            cp {libraries_dir}/fastutil-8.2.1.jar
            cp {libraries_dir}/log4j-api-2.8.1.jar
            cp {libraries_dir}/log4j-core-2.8.1.jar
            cp {libraries_dir}/text2speech-1.11.3.jar
            cp {libraries_dir}/java-objc-bridge-1.0.0.jar
            cp {libraries_dir}/minecraft-1.16.4-client.jar
            ext {libraries_dir}/java-objc-bridge-1.0.0-natives-osx.jar
            natives NO_NATIVES
            launch
            "#,
            game_dir = mc_libs_dir.join("minecraft").to_string_lossy(),
            assets_dir = mc_libs_dir.join("assets").to_string_lossy(),
            access_token = auth_data.auth_token,
            username = auth_data.username,
            uuid = auth_data.uuid,
            libraries_dir = mc_libs_dir.join("libraries").to_string_lossy(),
        };
        
        let java_args = formatdoc! {r#"
            -Dorg.lwjgl.librarypath={wd}/lwjglnatives
            -Xdock:icon=icon.png
            -Xdock:name=AppleSiliconMinecraft
            -XstartOnFirstThread
            -Xms409m
            -Xmx2048m
            -Duser.language=en
            -cp {wd}/NewLaunch.jar:{wd}/libraries/lwjglfat.jar:{wd}/libraries/patchy-1.1.jar:{wd}/libraries/project/oshi-core/1.1/oshi-core-1.1.jar:{wd}/libraries/jna-4.4.0.jar:{wd}/libraries/platform-3.4.0.jar:{wd}/libraries/icu4j-66.1.jar:{wd}/libraries/javabridge-1.0.22.jar:{wd}/libraries/jopt-simple-5.0.3.jar:{wd}/libraries/netty-all-4.1.25.Final.jar:{wd}/libraries/guava-21.0.jar:{wd}/libraries/commons-lang3-3.5.jar:{wd}/libraries/commons-io-2.5.jar:{wd}/libraries/commons-codec-1.10.jar:{wd}/libraries/brigadier-1.0.17.jar:{wd}/libraries/datafixerupper-4.0.26.jar:{wd}/libraries/gson-2.8.0.jar:{wd}/libraries/authlib-2.0.27.jar:{wd}/libraries/commons-compress-1.8.1.jar:{wd}/libraries/httpclient-4.3.3.jar:{wd}/libraries/commons-logging-1.1.3.jar:{wd}/libraries/httpcore-4.3.2.jar:{wd}/libraries/fastutil-8.2.1.jar:{wd}/libraries/log4j-api-2.8.1.jar:{wd}/libraries/log4j-core-2.8.1.jar:{wd}/libraries/text2speech-1.11.3.jar:{wd}/libraries/java-objc-bridge-1.0.0.jar:{wd}/libraries/minecraft-1.16.4-client.jar:{wd}/libraries/java-objc-bridge-1.0.0-natives-osx.jar
            org.multimc.EntryPoint
            "#,
            wd = MC_LIBS_PATH.get().unwrap().clone().to_string_lossy()
        }.replace("\n", " ");
        let launcher_process = process::Command::new(MC_LIBS_PATH.get().unwrap().clone().join("zulu-11.jdk/Contents/Home/bin/java")).arg(java_args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Launcher command failed to start");
        let mut launcher_process_stdin = launcher_process.stdin.unwrap();
        launcher_process_stdin.write_all(launch_script.as_bytes())?;
        let mut mclog_file = fs::File::create(APP_DATA_DIR.get().unwrap().clone().join("mclog.log"))?;
        io::copy(&mut launcher_process.stdout.unwrap(), &mut mclog_file)?;
        
        Ok(())
    }

    #[derive(Clone)]
    pub struct AuthResult {
        uuid: String,
        username: String,
        auth_token: String,
    }

    pub async fn authenticate(creds: MinecraftCredentials) -> Result<AuthResult, Box<dyn Error>> {
        let auth_json = json!({
            "agent": {
                "name": "Minecraft",
                "version": 1
            },
            "clientToken": "client identifier",
            "requestUser": true,
            "username": creds.username,
            "password": creds.password,
        });
        
        let req_client = reqwest::Client::new();
        let resp = req_client.post("https://authserver.mojang.com/authenticate").body(auth_json.to_string()).send().await?.text().await?;
        let resp: Value = serde_json::from_str(&resp)?;
        
        let result = AuthResult {
            uuid: resp.get("selectedProfile").unwrap().get("id").unwrap().as_str().unwrap().to_string(),
            username: resp.get("selectedProfile").unwrap().get("name").unwrap().as_str().unwrap().to_string(),
            auth_token: resp.get("accessToken").unwrap().as_str().unwrap().to_string()
        };
        
        log::info!("uuid: {}", result.uuid);
        log::info!("username: {}", result.username);
        log::info!("auth_token: {}", result.auth_token);
        
        Ok(result)
    }
}

use cacao::{
    button::Button,
    input::TextField,
    layout::{Layout, LayoutConstraint},
    macos::{
        menu::{Menu, MenuItem},
        window::{Window, WindowConfig, WindowDelegate},
        App, AppDelegate,
    },
    notification_center::Dispatcher,
    progress::{ProgressIndicator, ProgressIndicatorStyle},
    text::{Font, Label},
    utils::async_main_thread,
    view::{View, ViewDelegate},
};
use config::{Config, MinecraftCredentials};
use launcher::*;
use once_cell::sync::OnceCell;
use std::{fs, path::PathBuf, thread};
use tempfile::tempdir;
use unzpack::Unzpack;

const APP_BUNDLE_ID: &str = "raphtlw.apps.M1necraft";
static MC_LIBS_PATH: OnceCell<PathBuf> = OnceCell::new();
static APP_DATA_DIR: OnceCell<PathBuf> = OnceCell::new();
static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();

#[derive(Debug)]
struct MainApp {
    window: Window<AppWindow>,
}

impl AppDelegate for MainApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![
            Menu::new(
                "",
                vec![
                    MenuItem::Services,
                    MenuItem::Separator,
                    MenuItem::Hide,
                    MenuItem::HideOthers,
                    MenuItem::ShowAll,
                    MenuItem::Separator,
                    MenuItem::Quit,
                ],
            ),
            Menu::new("File", vec![MenuItem::CloseWindow]),
            Menu::new(
                "Edit",
                vec![
                    MenuItem::Undo,
                    MenuItem::Redo,
                    MenuItem::Separator,
                    MenuItem::Cut,
                    MenuItem::Copy,
                    MenuItem::Paste,
                    MenuItem::Separator,
                    MenuItem::SelectAll,
                ],
            ),
            // Sidebar option is 11.0+ only.
            Menu::new("View", vec![MenuItem::EnterFullScreen]),
            Menu::new(
                "Window",
                vec![
                    MenuItem::Minimize,
                    MenuItem::Zoom,
                    MenuItem::Separator,
                    MenuItem::new("Bring All to Front"),
                ],
            ),
            Menu::new("Help", vec![]),
        ]);

        App::activate();
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

/// Handle all the UI related messages here, like
/// User input, button presses, etc.
impl Dispatcher for MainApp {
    type Message = Message;

    fn on_ui_message(&self, message: Self::Message) {
        match message {
            Message::InitializationCompleted => {
                log::debug!("App initialization complete");

                // Stop initialization progress indicator
                self.window
                    .delegate
                    .as_ref()
                    .unwrap()
                    .login_view
                    .delegate
                    .as_ref()
                    .unwrap()
                    .initialization_indicator
                    .set_hidden(true);

                // Check if setup has been done before
                let config = Config::read().unwrap();
                if config.setup_complete {
                    self.window
                        .delegate
                        .as_ref()
                        .unwrap()
                        .launcher_view
                        .delegate
                        .as_ref()
                        .unwrap()
                        .progress_bar
                        .set_hidden(true);
                } else {
                    self.window
                        .delegate
                        .as_ref()
                        .unwrap()
                        .launcher_view
                        .delegate
                        .as_ref()
                        .unwrap()
                        .button_launch_game
                        .set_hidden(true);
                    Message::dispatch(Message::SetSetupProgress(
                        Some(String::from("Initializing Minecraft files")),
                        0.0,
                    ))
                }

                async_main_thread(|| {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(setup_libs()).unwrap();
                });
            }
            Message::LaunchGame => async_main_thread(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(launch()).expect("Failed to launch game");
            }),
            Message::LoginCallback => {
                let login_view = self
                    .window
                    .delegate
                    .as_ref()
                    .unwrap()
                    .login_view
                    .delegate
                    .as_ref()
                    .unwrap();
                let username = login_view.input_username.get_value();
                let password = login_view.input_password.get_value();

                // Closure is, according to the language specification,
                // able to be called more than once, which means
                // you cannot move variables outside of the closure to
                // the inside of the closure without cloning the variable
                // first.
                async_main_thread(move || {
                    // Store username and password in config file
                    let mut config = Config::read().unwrap();
                    config.minecraft_creds = MinecraftCredentials {
                        username: username.clone(),
                        password: password.clone(),
                    };
                    Config::write(config).unwrap();
                });
            }
            Message::SetSetupProgress(status, progress) => {
                let progress_view = self
                    .window
                    .delegate
                    .as_ref()
                    .unwrap()
                    .launcher_view
                    .delegate
                    .as_ref()
                    .unwrap()
                    .progress_bar
                    .delegate
                    .as_ref()
                    .unwrap();
                if let Some(status) = status {
                    progress_view.label.set_text(status)
                };
                progress_view.progress.set_value(progress);
            }
        }
    }
}

#[derive(Debug)]
struct AppWindow {
    login_view: View<LoginView>,
    launcher_view: View<LauncherView>,
}

impl AppWindow {
    fn new() -> Self {
        Self {
            login_view: View::with(LoginView::new()),
            launcher_view: View::with(LauncherView::new()),
        }
    }
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("M1necraft");
        window.set_minimum_content_size(300.0, 300.0);
        window.set_content_size(600.0, 400.0);

        if let Ok(config) = Config::read() {
            if config.minecraft_creds.username.len() > 0 {
                window.set_content_view(&self.launcher_view);
            }
        } else {
            window.set_content_view(&self.login_view);
        }
    }
}

/// Should be run as soon as app initialization is completed.
/// Does not depend on authentication credentials.
pub async fn setup_libs() -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Starting mc_libs setup");

    download_mc_libraries().await.unwrap();
    download_mc_assets().await.unwrap();

    Ok(())
}

#[derive(Debug)]
struct LoginView {
    label_title: Label,
    label_username: Label,
    label_password: Label,
    input_username: TextField,
    input_password: TextField,
    button_no_account: Button,
    button_submit: Button,
    initialization_indicator: ProgressIndicator,
}

impl LoginView {
    pub fn new() -> Self {
        Self {
            label_title: Label::new(),
            label_username: Label::new(),
            label_password: Label::new(),
            input_username: TextField::new(),
            input_password: TextField::new(),
            button_no_account: Button::new("Continue without account"),
            button_submit: Button::new("Next"),
            initialization_indicator: ProgressIndicator::new(),
        }
    }
}

impl ViewDelegate for LoginView {
    const NAME: &'static str = "LoginView";

    fn did_load(&mut self, view: View) {
        self.label_title.set_text("Log in");
        self.label_title.set_font(Font::bold_system(28.));
        self.label_username.set_text("Username:");
        self.label_password.set_text("Password:");

        self.button_no_account.set_bordered(false);
        self.button_submit.set_key_equivalent("\r");

        self.button_no_account.set_action(|| {
            log::debug!("button_no_account clicked");

            todo!("Handle no account, ask for player name");
        });
        self.button_submit.set_action(|| {
            log::debug!("button_submit clicked");

            Message::dispatch(Message::LoginCallback);
        });

        self.initialization_indicator.start_animation();
        self.initialization_indicator
            .set_style(ProgressIndicatorStyle::Spinner);

        view.add_subview(&self.label_title);
        view.add_subview(&self.label_username);
        view.add_subview(&self.label_password);
        view.add_subview(&self.input_username);
        view.add_subview(&self.input_password);
        view.add_subview(&self.button_no_account);
        view.add_subview(&self.initialization_indicator);
        view.add_subview(&self.button_submit);

        LayoutConstraint::activate(&[
            self.label_title
                .top
                .constraint_greater_than_or_equal_to(&view.top)
                .offset(100.),
            self.label_title
                .center_x
                .constraint_equal_to(&view.center_x),
            //
            self.label_username
                .trailing
                .constraint_equal_to(&self.input_username.leading)
                .offset(-14.),
            self.label_username
                .center_y
                .constraint_equal_to(&self.input_username.center_y),
            //
            self.input_username
                .center_x
                .constraint_equal_to(&view.center_x)
                .offset(28.),
            self.input_username
                .top
                .constraint_greater_than_or_equal_to(&view.top),
            self.input_username
                .bottom
                .constraint_equal_to(&self.input_password.top)
                .offset(-16.),
            self.input_username.width.constraint_equal_to_constant(280.),
            //
            self.label_password
                .trailing
                .constraint_equal_to(&self.input_password.leading)
                .offset(-14.),
            self.label_password
                .center_y
                .constraint_equal_to(&self.input_password.center_y),
            //
            self.input_password
                .center_x
                .constraint_equal_to(&view.center_x)
                .offset(28.),
            self.input_password
                .center_y
                .constraint_greater_than_or_equal_to(&view.center_y)
                .offset(16.),
            self.input_password.width.constraint_equal_to_constant(280.),
            //
            self.button_no_account
                .top
                .constraint_equal_to(&self.input_password.bottom)
                .offset(24.),
            self.button_no_account
                .center_x
                .constraint_equal_to(&view.center_x),
            //
            self.initialization_indicator
                .center_y
                .constraint_equal_to(&self.button_submit.center_y),
            self.initialization_indicator
                .trailing
                .constraint_equal_to(&self.button_submit.leading)
                .offset(-8.),
            self.initialization_indicator
                .height
                .constraint_equal_to(&self.button_submit.height),
            //
            self.button_submit
                .bottom
                .constraint_equal_to(&view.bottom)
                .offset(-24.),
            self.button_submit
                .trailing
                .constraint_equal_to(&view.trailing)
                .offset(-24.),
        ]);
    }
}

#[derive(Debug)]
struct LauncherView {
    label_title: Label,
    button_launch_game: Button,
    progress_bar: View<SetupProgressView>,
    initialization_indicator: ProgressIndicator,
}

impl LauncherView {
    pub fn new() -> Self {
        Self {
            label_title: Label::new(),
            button_launch_game: Button::new("Launch Minecraft 1.16.4"),
            progress_bar: View::with(SetupProgressView::new()),
            initialization_indicator: ProgressIndicator::new(),
        }
    }
}

impl ViewDelegate for LauncherView {
    const NAME: &'static str = "LauncherView";

    fn did_load(&mut self, view: View) {
        self.label_title.set_text("Minecraft");
        self.label_title.set_font(Font::bold_system(28.));

        view.add_subview(&self.label_title);
        view.add_subview(&self.button_launch_game);
        view.add_subview(&self.progress_bar);
        view.add_subview(&self.initialization_indicator);

        LayoutConstraint::activate(&[
            self.label_title
                .center_x
                .constraint_equal_to(&view.center_x),
            self.label_title
                .top
                .constraint_equal_to(&view.top)
                .offset(20.),
            //
            self.button_launch_game
                .center_x
                .constraint_equal_to(&view.center_x),
            self.button_launch_game
                .center_y
                .constraint_equal_to(&view.center_y),
            //
            self.progress_bar
                .center_x
                .constraint_equal_to(&view.center_x),
            self.progress_bar
                .top
                .constraint_equal_to(&self.button_launch_game.bottom),
            //
            self.initialization_indicator
                .center_x
                .constraint_equal_to(&view.center_x),
            self.initialization_indicator
                .top
                .constraint_equal_to(&self.progress_bar.bottom),
        ]);
    }
}

#[derive(Debug)]
struct SetupProgressView {
    pub label: Label,
    pub progress: ProgressIndicator,
}

impl SetupProgressView {
    pub fn new() -> Self {
        Self {
            label: Label::new(),
            progress: ProgressIndicator::new(),
        }
    }
}

impl ViewDelegate for SetupProgressView {
    const NAME: &'static str = "SetupProgressView";

    fn did_load(&mut self, view: View) {
        self.label.set_text("Downloading libraries");
        self.progress.set_style(ProgressIndicatorStyle::Bar);

        view.add_subview(&self.label);
        view.add_subview(&self.progress);

        LayoutConstraint::activate(&[
            self.label
                .bottom
                .constraint_equal_to(&self.progress.top)
                .offset(100.),
            self.label
                .top
                .constraint_greater_than_or_equal_to(&view.top),
            self.label.left.constraint_equal_to(&self.progress.left),
            self.progress
                .center_x
                .constraint_greater_than_or_equal_to(&view.center_x),
            self.progress
                .center_y
                .constraint_greater_than_or_equal_to(&view.center_y),
        ]);
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    InitializationCompleted,
    LoginCallback,
    LaunchGame,
    SetSetupProgress(Option<String>, f64),
}

impl Message {
    pub fn dispatch(message: Message) {
        log::debug!("Dispatching UI message: {:?}", message);
        App::<MainApp, Message>::dispatch_main(message);
    }
}

fn main() {
    pretty_env_logger::init();

    // Initialize Paths
    APP_DATA_DIR
        .set(dirs::data_local_dir().unwrap().join(APP_BUNDLE_ID))
        .unwrap();
    if !APP_DATA_DIR.get().unwrap().exists() {
        log::debug!("App data directory does not exist, creating...");
        fs::create_dir(APP_DATA_DIR.get().unwrap().clone())
            .expect("Failed to create app data directory");
    }
    log::debug!("APP_DATA_DIR: {:#?}", APP_DATA_DIR.get().unwrap());

    CONFIG_PATH
        .set(APP_DATA_DIR.get().unwrap().clone().join("config.json"))
        .unwrap();
    log::debug!("CONFIG_PATH: {:#?}", CONFIG_PATH.get().unwrap());

    MC_LIBS_PATH
        .set(PathBuf::from(
            APP_DATA_DIR.get().unwrap().clone().join("mc_libs"),
        ))
        .unwrap();
    log::debug!("MC_LIBS_PATH: {:#?}", MC_LIBS_PATH.get().unwrap());

    let app_init_thread = thread::spawn(|| {
        log::debug!("app_init_thread started");

        // Setup mc_libs stuff
        // Checks if mc_libs has been fully downloaded & extracted previously
        if !MC_LIBS_PATH.get().unwrap().clone().exists() {
            let temp_dir = tempdir().unwrap();
            let mc_libs_zip_path = temp_dir.path().join("mc_libs.zip");
            log::debug!("mc_libs.zip path: {:#?}", &mc_libs_zip_path);
            let mut mc_libs_zip_file = fs::File::create(&mc_libs_zip_path).unwrap();
            log::debug!("Downloading mc_libs.zip from GitHub...");
            reqwest::blocking::get(
                "https://github.com/raphtlw/m1necraft/releases/download/resources/mc_libs.zip",
            )
            .expect("Failed to download mc_libs.zip")
            .copy_to(&mut mc_libs_zip_file)
            .expect("Failed to write mc_libs.zip as bytes");

            log::debug!("Extracting mc_libs.zip");
            // Unpack Minecraft working directory archive
            Unzpack::extract(&mc_libs_zip_path, &APP_DATA_DIR.get().unwrap().clone())
                .expect("Failed to unpack resources");
            temp_dir
                .close()
                .expect("Failed to delete temporary directory");

            // Only after unpacking the resources should the config file
            // (AKA, the first launch indicator) be created.
            Config::write(Config::default()).expect("Failed to create config file");
        }

        // Let UI thread know that app initialization has completed
        Message::dispatch(Message::InitializationCompleted);
    });

    App::new(
        APP_BUNDLE_ID,
        MainApp {
            window: Window::with(WindowConfig::default(), AppWindow::new()),
        },
    )
    .run();

    app_init_thread
        .join()
        .expect("Initialization thread failed to stop");
}
