use std::fmt::Display;

use iced::{
    theme,
    widget::{button, row, text},
    Length,
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
        let title_display = text(&self.title).width(Length::Fill).size(25);

        let delete_button = button(text(TRASH_CAN_ICON).font(ICONS).size(20))
            .on_press(TaskMessage::Delete)
            .padding(10)
            .style(theme::Button::Destructive)
            .width(40);

        let buttons_display = row![self.status_change_buttons(), delete_button]
            .align_items(iced::Alignment::End)
            .spacing(10);

        row![title_display, buttons_display].spacing(30).into()
    }

    fn status_change_buttons(&self) -> iced::Element<TaskMessage> {
        let todo_button = button(text(LEFT_ARROW_ICON).font(ICONS).size(30))
            .on_press(TaskMessage::MarkAs(TaskStatus::Todo))
            .padding(5);

        let in_progress_button_right_arrow = button(text(RIGHT_ARROW_ICON).font(ICONS).size(30))
            .on_press(TaskMessage::MarkAs(TaskStatus::InProgress))
            .padding(5);

        let in_progress_button_left_arrow = button(text(LEFT_ARROW_ICON).font(ICONS).size(30))
            .on_press(TaskMessage::MarkAs(TaskStatus::InProgress))
            .padding(5);

        // greyed out buttons that don't do anything
        // Not showing them makes the widget look bad
        let disabled_left_arrow = button(text(LEFT_ARROW_ICON).font(ICONS).size(30)).padding(5);
        let disabled_right_arrow = button(text(RIGHT_ARROW_ICON).font(ICONS).size(30)).padding(5);

        let done_button = button(text(RIGHT_ARROW_ICON).font(ICONS).size(30))
            .on_press(TaskMessage::MarkAs(TaskStatus::Done))
            .padding(5);

        match self.task_status {
            TaskStatus::Todo => {
                row![disabled_left_arrow, in_progress_button_right_arrow]
            }
            TaskStatus::InProgress => {
                row![todo_button, done_button]
            }
            TaskStatus::Done => {
                row![in_progress_button_left_arrow, disabled_right_arrow]
            }
        }
        .width(80)
        .spacing(5)
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
