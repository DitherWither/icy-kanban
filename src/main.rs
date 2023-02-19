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
const PLUS_ICON: &str = "\u{2b}";

fn main() -> Result<(), iced::Error> {
    TodoApplication::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    TaskChanged(usize, TaskMessage),
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
        "Icy Kanban".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TaskChanged(i, TaskMessage::Delete) => {
                self.task_list.remove(i);
            }

            Message::TaskChanged(i, message) => {
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
        let title = text("Icy KanBan").size(80);

        let new_task_display = self.new_task_display();

        let tasks_display = self.current_tasks_display();

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
    fn get_filtered_task_display<F>(
        &self,
        mut filter_fn: F,
        heading_text: &str,
    ) -> iced::Element<Message>
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
                            .map(move |message| Message::TaskChanged(i, message))
                    })
                    .collect(),
            )
            .spacing(20)
        ]
        .width(Length::Fill)
        .into()
    }

    /// Returns an iced widget that can be used to create a new task
    fn new_task_display(&self) -> iced::Element<Message> {
        let text_input = text_input(
            "Enter new task",
            &self.text_input,
            Message::TextFieldChanged,
        )
        .on_submit(Message::CreateTask);
        let new_task_button = button(PLUS_ICON).on_press(Message::CreateTask);

        row![text_input, new_task_button].spacing(5).into()
    }

    /// Returns an iced widget that displays all tasks, sorted into "To do",
    /// "In Progress", and "Done"
    fn current_tasks_display(&self) -> iced::Element<Message> {
        let todo_display =
            self.get_filtered_task_display(|elem| elem.task_status == TaskStatus::Todo, "To Do");
        let in_progress_display = self.get_filtered_task_display(
            |elem| elem.task_status == TaskStatus::InProgress,
            "In Progress",
        );
        let done_display =
            self.get_filtered_task_display(|elem| elem.task_status == TaskStatus::Done, "Done");

        row![todo_display, in_progress_display, done_display]
            .spacing(30)
            .into()
    }

    /// Creates and adds a new task to the application
    fn add_task(&mut self, title: impl Display) {
        self.task_list.push(Task::new(title.to_string()));
    }
}
