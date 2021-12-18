mod config;
mod launcher;

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
use config::Config;
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

/// Nothing to do with the app struct so we pass the messages down
impl Dispatcher for MainApp {
    type Message = Message;

    fn on_ui_message(&self, message: Self::Message) {
        self.window.delegate.as_ref().unwrap().on_message(message);
    }
}

#[derive(Debug)]
struct AppWindow {
    login_view: View<LoginView>,
}

impl AppWindow {
    fn new() -> Self {
        Self {
            login_view: View::with(LoginView::new()),
        }
    }

    fn on_message(&self, message: Message) {
        self.login_view
            .delegate
            .as_ref()
            .unwrap()
            .on_message(message);
    }
}

impl WindowDelegate for AppWindow {
    const NAME: &'static str = "WindowDelegate";

    fn did_load(&mut self, window: Window) {
        window.set_title("M1necraft");
        window.set_minimum_content_size(300.0, 300.0);
        window.set_content_size(600.0, 400.0);
        window.set_content_view(&self.login_view);
    }
}

/// Should be run after the user logs in either
/// as a guest or a normal user.
pub async fn after_login(username: String, password: String) {
    log::debug!("Starting post login setup");

    let mut config = Config::read().unwrap();
    config.minecraft_creds.username = username;
    config.minecraft_creds.password = password;

    download_mc_libraries().await.unwrap();
    download_mc_assets().await.unwrap();
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
    spinny_wheel: ProgressIndicator,
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
            spinny_wheel: ProgressIndicator::new(),
        }
    }

    pub fn on_message(&self, message: Message) {
        match message {
            Message::InitializationCompleted => {
                log::debug!("Initialization complete");
                self.spinny_wheel.set_hidden(true);
            }
            Message::LoginCallback => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(after_login(
                    self.input_username.get_value(),
                    self.input_password.get_value(),
                ));
            }
            Message::LaunchGame => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(launch()).expect("Failed to launch game");
            }
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

            // let rt = tokio::runtime::Runtime::new().unwrap();

            // rt.block_on(after_login(username, password));

            dispatch_ui(Message::LoginCallback);
        });
        self.button_submit.set_action(|| {
            log::debug!("button_submit clicked");

            // let rt = tokio::runtime::Runtime::new().unwrap();

            // rt.block_on(after_login(username, password));

            dispatch_ui(Message::LoginCallback);
        });

        self.spinny_wheel.start_animation();
        self.spinny_wheel.set_style(ProgressIndicatorStyle::Spinner);

        view.add_subview(&self.label_title);
        view.add_subview(&self.label_username);
        view.add_subview(&self.label_password);
        view.add_subview(&self.input_username);
        view.add_subview(&self.input_password);
        view.add_subview(&self.button_no_account);
        view.add_subview(&self.spinny_wheel);
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
            self.spinny_wheel
                .center_y
                .constraint_equal_to(&self.button_submit.center_y),
            self.spinny_wheel
                .trailing
                .constraint_equal_to(&self.button_submit.leading)
                .offset(-8.),
            self.spinny_wheel
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

#[derive(Clone, Debug)]
pub enum Message {
    InitializationCompleted,
    LoginCallback,
    LaunchGame,
}

pub fn dispatch_ui(message: Message) {
    log::debug!("Dispatching UI message: {:?}", message);
    App::<MainApp, Message>::dispatch_main(message);
}

fn main() {
    pretty_env_logger::init();

    let app_init_thread = thread::spawn(|| {
        log::debug!("app_init_thread started");

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

        // Setup mc_libs stuff
        // Uses CONFIG_PATH to check if app has been run before,
        // if app has not been run before, then extract the mc_libs.zip
        // file into the data directory.
        if !CONFIG_PATH.get().unwrap().clone().exists() {
            let temp_dir = tempdir().unwrap();
            let mc_libs_zip_path = temp_dir.path().join("mc_libs.zip");
            log::debug!("Downloading mc_libs.zip from GitHub...");
            let mc_libs_zip_bytes = reqwest::blocking::get(
                "https://github.com/raphtlw/m1necraft/releases/download/resources/mc_libs.zip",
            )
            .unwrap()
            .bytes()
            .unwrap();
            fs::write(&mc_libs_zip_path, mc_libs_zip_bytes)
                .expect("Failed to write mc_libs.zip as bytes");

            log::debug!("Creating Minecraft working directory...");
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
        dispatch_ui(Message::InitializationCompleted);
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
