mod task;
use std::fmt::Display;

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input},
    Font, Length, Sandbox,
};
use task::{Task, TaskMessage};

use crate::task::TaskStatus;

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.otf"),
};

const RIGHT_ARROW_ICON: &str = "\u{f061}";
const LEFT_ARROW_ICON: &str = "\u{f060}";
const TRASH_CAN_ICON: &str = "\u{f2ed}";

fn main() -> Result<(), iced::Error> {
    TodoApplication::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    TaskMessage(usize, TaskMessage),
    TextFieldChanged(String),
    CreateTask,
}

struct TodoApplication {
    task_list: Vec<Task>,
    text_input: String,
}

impl iced::Sandbox for TodoApplication {
    type Message = Message;

    fn new() -> Self {
        TodoApplication {
            task_list: Vec::new(),
            text_input: "".to_string(),
        }
    }

    fn title(&self) -> String {
        "KanBan Board Application".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TaskMessage(i, TaskMessage::Delete) => {
                self.task_list.remove(i);
            }

            Message::TaskMessage(i, message) => {
                // If the index doesn't exist, just ingore the message
                if let Some(task) = self.task_list.get_mut(i) {
                    task.update(message);
                };
            }
            Message::TextFieldChanged(s) => self.text_input = s,
            Message::CreateTask => {
                self.add_task(self.text_input.clone().trim());
                self.text_input = String::new();
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // TODO seperate this into different functions, this is getting too long

        let title = text("Kanban Board").size(80);

        let text_input = text_input(
            "Enter new task",
            &self.text_input,
            Message::TextFieldChanged,
        );
        let new_task_button = button("New Task").on_press(Message::CreateTask);

        let new_task_display = row![text_input, new_task_button];

        let todo_display =
            self.get_task_display(|elem| elem.task_status == TaskStatus::Todo, "To Do");
        let in_progress_display = self.get_task_display(
            |elem| elem.task_status == TaskStatus::InProgress,
            "In Progress",
        );
        let done_display =
            self.get_task_display(|elem| elem.task_status == TaskStatus::Done, "Done");

        let tasks_display = row![todo_display, in_progress_display, done_display].spacing(30);

        let content = column![title, new_task_display, tasks_display].spacing(20);

        scrollable(
            container(content)
                .width(iced::Length::Fill)
                .padding(40)
                .center_x(),
        )
        .into()
    }
}

impl TodoApplication {
    /// Creates a display for task using a iterator-filter like closure
    fn get_task_display<F>(&self, mut filter_fn: F, heading_text: &str) -> iced::Element<Message>
    where
        F: FnMut(&&Task) -> bool,
    {
        column![
            text(heading_text).size(40),
            column(
                self.task_list
                    .iter()
                    .enumerate()
                    .filter(|(_, elem)| filter_fn(elem))
                    .map(|(i, elem)| {
                        elem.view(i)
                            .map(move |message| Message::TaskMessage(i, message))
                    })
                    .collect(),
            )
            .spacing(20)
        ]
        .width(Length::Fill)
        .into()
    }

    /// Creates and adds a new task to the application
    fn add_task(&mut self, title: impl Display) {
        self.task_list.push(Task::new(title.to_string()));
    }
}
