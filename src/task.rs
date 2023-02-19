use std::fmt::Display;

use iced::{
    theme,
    widget::{button, row, text},
};

use crate::{ICONS, LEFT_ARROW_ICON, RIGHT_ARROW_ICON, TRASH_CAN_ICON};

pub struct Task {
    title: String,
    pub task_status: TaskStatus,
}

impl Task {
    pub fn new(title: String) -> Task {
        Task {
            title,
            task_status: TaskStatus::Todo,
        }
    }

    pub fn update(&mut self, message: TaskMessage) {
        match message {
            // The delete event should be handled by the window containing the task, not the widget.
            TaskMessage::Delete => {}
            TaskMessage::MarkAs(status) => {
                self.task_status = status;
            }
        }
    }

    pub fn view(&self, _id: usize) -> iced::Element<TaskMessage> {
        let title_display = text(&self.title);
        let delete_button = button(text(TRASH_CAN_ICON).font(ICONS))
            .on_press(TaskMessage::Delete)
            .padding(10)
            .style(theme::Button::Destructive);

        let todo_button = button(text(LEFT_ARROW_ICON).font(ICONS))
            .on_press(TaskMessage::MarkAs(TaskStatus::Todo));

        let in_progress_button_right_arrow = button(text(RIGHT_ARROW_ICON).font(ICONS))
            .on_press(TaskMessage::MarkAs(TaskStatus::InProgress));

        let in_progress_button_left_arrow = button(text(LEFT_ARROW_ICON).font(ICONS))
            .on_press(TaskMessage::MarkAs(TaskStatus::InProgress));

        let done_button = button(text(RIGHT_ARROW_ICON).font(ICONS))
            .on_press(TaskMessage::MarkAs(TaskStatus::Done));

        let task_status_buttons = match self.task_status {
            TaskStatus::Todo => {
                row![in_progress_button_right_arrow]
            }
            TaskStatus::InProgress => {
                row![todo_button, done_button]
            }
            TaskStatus::Done => {
                row![in_progress_button_left_arrow]
            }
        };

        // TODO Make an element that doesent look like crap
        row![
            title_display,
            task_status_buttons.width(80),
            delete_button.width(40)
        ]
        .spacing(20)
        .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TaskMessage {
    Delete,
    MarkAs(TaskStatus),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_string = match self {
            TaskStatus::Todo => "To do",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Done => "Done",
        };

        write!(f, "{display_string}")
    }
}
