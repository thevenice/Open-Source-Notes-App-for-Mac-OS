use iced::widget::{pick_list, column, container, text};
use iced::{Sandbox, Element, Length, Color, Settings};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorOption {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
}

impl ColorOption {
    fn to_color(&self) -> Color {
        match self {
            ColorOption::Red => Color::from_rgb(0.8, 0.0, 0.0),
            ColorOption::Green => Color::from_rgb(0.0, 0.8, 0.0),
            ColorOption::Blue => Color::from_rgb(0.0, 0.0, 0.8),
            ColorOption::Yellow => Color::from_rgb(0.8, 0.8, 0.0),
            ColorOption::Orange => Color::from_rgb(0.8, 0.5, 0.2),
        }
    }
}

impl std::fmt::Display for ColorOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorOption::Red => write!(f, "Red"),
            ColorOption::Green => write!(f, "Green"),
            ColorOption::Blue => write!(f, "Blue"),
            ColorOption::Yellow => write!(f, "Yellow"),
            ColorOption::Orange => write!(f, "Orange"),
        }
    }
}

#[derive(Debug, Clone)]
enum AppMessage {
    SelectColor(ColorOption),
}

struct App {
    selected_color: Option<ColorOption>,
    color_options: Vec<ColorOption>,
}

impl Sandbox for App {
    type Message = AppMessage;

    fn new() -> Self {
        Self {
            selected_color: None,
            color_options: vec![
                ColorOption::Red,
                ColorOption::Green,
                ColorOption::Blue,
                ColorOption::Yellow,
                ColorOption::Orange,
            ],
        }
    }

    fn title(&self) -> String {
        String::from("Color Picker App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::SelectColor(color) => {
                self.selected_color = Some(color);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let color_pick_list = pick_list(
            &self.color_options,
            self.selected_color,
            AppMessage::SelectColor,
        );

        let content = column![
            text("Select a color:"),
            color_pick_list,
        ]
        .padding(20)
        .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}