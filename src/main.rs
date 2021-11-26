use cacao::button::Button;
use cacao::input::TextField;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::macos::menu::{Menu, MenuItem};
use cacao::macos::window::{Window, WindowConfig, WindowDelegate};
use cacao::macos::{App, AppDelegate};
use cacao::text::{Font, Label};
use cacao::view::{View, ViewDelegate};

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
}

// #[derive(Debug, Default)]
// struct UsernameInput;

// impl TextFieldDelegate for UsernameInput {
//     const NAME: &'static str = "UsernameInput";

//     fn text_should_begin_editing(&self, value: &str) -> bool {
//         println!("Should begin with value: {}", value);
//         true
//     }

//     fn text_did_change(&self, value: &str) {
//         println!("Did change to: {}", value);
//     }

//     fn text_did_end_editing(&self, value: &str) {
//         println!("Ended: {}", value);
//     }
// }

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

#[derive(Debug)]
struct LoginView {
    label_title: Label,
    label_username: Label,
    label_password: Label,
    input_username: TextField,
    input_password: TextField,
    button_no_account: Button,
    button_submit: Button,
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

        view.add_subview(&self.label_title);
        view.add_subview(&self.label_username);
        view.add_subview(&self.label_password);
        view.add_subview(&self.input_username);
        view.add_subview(&self.input_password);
        view.add_subview(&self.button_no_account);
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

fn main() {
    App::new(
        "raphtlw.apps.M1necraft",
        MainApp {
            window: Window::with(WindowConfig::default(), AppWindow::new()),
        },
    )
    .run();
}
