mod task;
use iced::{
    widget::{column, container, row, scrollable, text},
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

#[derive(Debug, Clone, Copy)]
enum Message {
    TaskMessage(usize, TaskMessage),
}

struct TodoApplication {
    task_list: Vec<(usize, Task)>,
}

impl iced::Sandbox for TodoApplication {
    type Message = Message;

    fn new() -> Self {
        TodoApplication {
            task_list: vec![
                (0, Task::new("This is task".to_string())),
                (1, Task::new("This is also a task".to_string())),
            ],
        }
    }

    fn title(&self) -> String {
        "Todo Application".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TaskMessage(i, TaskMessage::Delete) => {
                self.task_list.remove(i);
            }

            Message::TaskMessage(i, message) => {
                // If the index doesn't exist, just ingore the message
                if let Some(task) = self.task_list.get_mut(i) {
                    task.1.update(message);
                };
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let title = text("Todos").size(80);

        let todo_display =
            self.get_task_display(|elem| elem.1.task_status == TaskStatus::Todo, "To Do");
        let in_progress_display = self.get_task_display(
            |elem| elem.1.task_status == TaskStatus::InProgress,
            "In Progress",
        );
        let done_display =
            self.get_task_display(|elem| elem.1.task_status == TaskStatus::Done, "Done");

        let tasks_display = row![todo_display, in_progress_display, done_display].spacing(30);

        let content = column![title, tasks_display].spacing(20);

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
    fn get_task_display<F>(&self, filter_fn: F, heading_text: &str) -> iced::Element<Message>
    where
        F: FnMut(&&(usize, Task)) -> bool,
    {
        column![
            text(heading_text).size(40),
            column(
                self.task_list
                    .iter()
                    .filter(filter_fn)
                    .map(|elem| {
                        elem.1
                            .view(elem.0)
                            .map(move |message| Message::TaskMessage(elem.0, message))
                    })
                    .collect(),
            )
            .spacing(20)
        ]
        .width(Length::Fill)
        .into()
    }
}
