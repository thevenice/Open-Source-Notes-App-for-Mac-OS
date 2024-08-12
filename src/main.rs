use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Application, Color, Command, Element, Length, Settings, Theme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum NoteColor {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
}

impl NoteColor {
    fn to_color(&self) -> Color {
        match self {
            NoteColor::Red => Color::from_rgb(1.0, 0.8, 0.8),
            NoteColor::Green => Color::from_rgb(0.8, 1.0, 0.8),
            NoteColor::Blue => Color::from_rgb(0.8, 0.8, 1.0),
            NoteColor::Yellow => Color::from_rgb(1.0, 1.0, 0.8),
            NoteColor::Orange => Color::from_rgb(1.0, 0.9, 0.8),
        }
    }
}

struct NoteButtonStyle {
    color: Color,
}

impl iced::widget::button::StyleSheet for NoteButtonStyle {
    type Style = Theme; // This should match the `Button`'s expected style type

    fn active(&self, _: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(self.color)),
            border_radius: 5.0,
            ..Default::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> iced::widget::button::Appearance {
        let lighten = |value: f32| -> f32 {
            (value + 0.1).min(1.0)
        };

        let new_color = Color::from_rgb(
            lighten(self.color.r),
            lighten(self.color.g),
            lighten(self.color.b),
        );

        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(new_color)),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

struct NotesApp {
    notes: HashMap<String, Note>,
    current_note: Option<String>,
    error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Note {
    id: String,
    title: String,
    content: String,
    color: NoteColor,
}

#[derive(Debug, Clone)]
enum Message {
    CreateNote,
    SelectNote(String),
    UpdateNoteTitle(String),
    UpdateNoteContent(String),
    ChangeNoteColor(NoteColor),
    ImportNotes,
    ExportNotes,
    ClearError,
}

impl Application for NotesApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                notes: HashMap::new(),
                current_note: None,
                error: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Multi-Notes App")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CreateNote => {
                let id = uuid::Uuid::new_v4().to_string();
                let note = Note {
                    id: id.clone(),
                    title: "New Note".to_string(),
                    content: String::new(),
                    color: NoteColor::Yellow,
                };
                self.notes.insert(id.clone(), note);
                self.current_note = Some(id);
            }
            Message::SelectNote(id) => {
                self.current_note = Some(id);
            }
            Message::UpdateNoteTitle(title) => {
                if let Some(id) = &self.current_note {
                    if let Some(note) = self.notes.get_mut(id) {
                        note.title = title;
                    }
                }
            }
            Message::UpdateNoteContent(content) => {
                if let Some(id) = &self.current_note {
                    if let Some(note) = self.notes.get_mut(id) {
                        note.content = content;
                    }
                }
            }
            Message::ChangeNoteColor(color) => {
                if let Some(id) = &self.current_note {
                    if let Some(note) = self.notes.get_mut(id) {
                        note.color = color;
                    }
                }
            }
            Message::ImportNotes => {
                match self.import_notes() {
                    Ok(_) => self.error = None,
                    Err(e) => self.error = Some(e.to_string()),
                }
            }
            Message::ExportNotes => {
                match self.export_notes() {
                    Ok(_) => self.error = None,
                    Err(e) => self.error = Some(e.to_string()),
                }
            }
            Message::ClearError => {
                self.error = None;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let notes_list = self.notes.values().fold(
            column![].spacing(5),
            |column, note| {
                column.push(
                    button(text(&note.title).size(16))
                        .on_press(Message::SelectNote(note.id.clone()))
                        .style(NoteButtonStyle {
                            color: note.color.to_color(),
                        })
                        .padding(10),
                )
            },
        );

        let notes_list = scrollable(notes_list).height(Length::Fill);

        let note_editor = if let Some(id) = &self.current_note {
            if let Some(note) = self.notes.get(id) {
                column![
                    text_input("Title", &note.title)
                        .on_input(Message::UpdateNoteTitle)
                        .padding(10),
                    text_input("Content", &note.content)
                        .on_input(Message::UpdateNoteContent)
                        .padding(10),
                    row![
                        button("Red").on_press(Message::ChangeNoteColor(NoteColor::Red)),
                        button("Green").on_press(Message::ChangeNoteColor(NoteColor::Green)),
                        button("Blue").on_press(Message::ChangeNoteColor(NoteColor::Blue)),
                        button("Yellow").on_press(Message::ChangeNoteColor(NoteColor::Yellow)),
                        button("Orange").on_press(Message::ChangeNoteColor(NoteColor::Orange)),
                    ]
                    .spacing(5),
                ]
                .spacing(10)
            } else {
                column![text("Note not found")]
            }
        } else {
            column![text("Select a note to edit")]
        };

        let content = row![
            notes_list.width(Length::FillPortion(1)),
            note_editor.width(Length::FillPortion(3)),
        ]
        .spacing(20);

        let controls = row![
            button("New Note").on_press(Message::CreateNote),
            button("Import").on_press(Message::ImportNotes),
            button("Export").on_press(Message::ExportNotes),
        ]
        .spacing(10);

        let mut layout = column![content, controls].padding(20).spacing(20);

        if let Some(error) = &self.error {
            layout = layout.push(
                container(text(error).style(iced::theme::Text::Color(Color::from_rgb(0.8, 0.0, 0.0))))
                    .padding(10),
            );
        }

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}

impl NotesApp {
    fn import_notes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let json = fs::read_to_string("notes.json")?;
        self.notes = serde_json::from_str(&json)?;
        Ok(())
    }

    fn export_notes(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self.notes)?;
        fs::write("notes.json", json)?;
        Ok(())
    }
}


fn main() -> iced::Result {
    NotesApp::run(Settings::default())
}
