mod config;
mod launcher;

use cacao::utils::async_main_thread;
use launcher::*;
use std::{fs, path::PathBuf, thread};

use cacao::button::Button;
use cacao::input::TextField;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::macos::menu::{Menu, MenuItem};
use cacao::macos::window::{Window, WindowConfig, WindowDelegate};
use cacao::macos::{App, AppDelegate};
use cacao::notification_center::Dispatcher;
use cacao::progress::{ProgressIndicator, ProgressIndicatorStyle};
use cacao::text::{Font, Label};
use cacao::view::{View, ViewDelegate};
use config::{Config, MinecraftCredentials};
use once_cell::sync::OnceCell;
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
