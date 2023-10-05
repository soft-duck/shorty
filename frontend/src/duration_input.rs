use std::fmt::{Display, Formatter};
use std::mem;

use strum::FromRepr;
use web_sys::{DragEvent, Event, FocusEvent, HtmlInputElement, KeyboardEvent, MouseEvent};
use yew::{AttrValue, Callback, Component, Context, Html, html, NodeRef, Properties};

pub struct DurationInput {
    duration: Duration,
    selection: Option<Selection>,
    backspace: bool,
}

#[derive(FromRepr, Copy, Clone, PartialEq)]
enum Selection {
    Hours = 0,
    Minutes = 1,
    Seconds = 2,
}

impl Selection {
    fn select(&self, input: &HtmlInputElement) {
        let start = (*self as usize * 3) as u32;
        input.set_selection_start(Some(start)).unwrap();
        input.set_selection_end(Some(start + 2)).unwrap();
    }

    fn from_cursor(cursor: u32) -> Self {
        if cursor > 8 {
            panic!();
        }

        Self::from_repr(cursor as usize / 3).unwrap()
    }

    fn left(&self) -> Self {
        if *self == Self::Hours {
            return *self;
        }

        Self::from_repr(*self as usize - 1).unwrap()
    }

    fn right(&self) -> Self {
        if *self == Self::Seconds {
            return *self;
        }

        Self::from_repr(*self as usize + 1).unwrap()
    }

    fn part_with(&self, n: i32) -> Parts {
        let mut parts = Parts {
            hours: 0,
            minutes: 0,
            seconds: 0,
        };
        
        match self {
            Selection::Hours => parts.hours = n,
            Selection::Minutes => parts.minutes = n,
            Selection::Seconds => parts.seconds = n,
        }

        parts
    }
}

struct Duration {
    seconds: u32,
    sub_cursor: bool,
}

#[derive(Copy, Clone)]
struct Parts {
    hours: i32,
    minutes: i32,
    seconds: i32,
}

impl Parts {
    const SECONDS_HOUR: i32 = 60 * 60;
    const SECONDS_MINUTES: i32 = 60;

    fn zero_selection(&mut self, selection: Selection) {
        match selection {
            Selection::Hours => self.hours = 0,
            Selection::Minutes => self.minutes = 0,
            Selection::Seconds => self.seconds = 0,
        }
    }

    fn to_seconds(&self) -> u32 {
        (self.hours * Self::SECONDS_HOUR + self.minutes * Self::SECONDS_MINUTES + self.seconds) as u32
    }

    fn valid(&self) -> bool {
        self.seconds < 60 && self.minutes < 60 && self.hours < 100
    }
}

impl Duration {
    const MAX_SECONDS: i32 = 99 * 60 * 60 + 59 * 60 + 59;

    fn to_parts(&self) -> Parts {
        let hours = self.seconds as i32 / Parts::SECONDS_HOUR;
        let reminder = self.seconds as i32 % Parts::SECONDS_HOUR;
        let minutes = reminder / Parts::SECONDS_MINUTES;
        let seconds = reminder % Parts::SECONDS_MINUTES;

        Parts {
            hours,
            minutes,
            seconds,
        }
    }

    fn from_parts(parts: Parts) -> Self {
        Self {
            seconds: parts.to_seconds(),
            sub_cursor: false,
        }
    }

    fn add_parts(&mut self, parts: Parts) {
        self.sub_cursor = false;

        let seconds = self.seconds as i32 + parts.seconds +
            parts.minutes * Parts::SECONDS_MINUTES +
            parts.hours * Parts::SECONDS_HOUR;

        if seconds > Self::MAX_SECONDS || seconds < 0 {
            return;
        }

        self.seconds = seconds as u32;
    }

    fn reset(&mut self) {
        self.seconds = 0;
        self.sub_cursor = false;
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parts = self.to_parts();

        write!(f, "{:02}:{:02}:{:02}", parts.hours, parts.minutes, parts.seconds)
    }
}

#[derive(Properties, PartialEq)]
pub struct DurationInputProps {
    pub name: Option<AttrValue>,
    pub input_ref: NodeRef,
}

fn cursor_location(input: &HtmlInputElement) -> u32 {
    let direction = input.selection_direction().unwrap().unwrap();

    let mut start = input.selection_start().unwrap().unwrap();
    let mut end = input.selection_end().unwrap().unwrap();

    if direction == "forward" {
        mem::swap(&mut start, &mut end);
    }

    start
}

// fn unselect(input: &HtmlInputElement) -> u32 {
//     let direction = input.selection_direction().unwrap().unwrap();
//
//     let mut start = input.selection_start().unwrap().unwrap();
//     let mut end = input.selection_end().unwrap().unwrap();
//
//     if direction == "forward" {
//         input.set_selection_end(Some(start)).unwrap();
//         mem::swap(&mut start, &mut end);
//     } else {
//         input.set_selection_start(Some(end)).unwrap();
//     };
//
//     start
// }

#[derive(PartialEq)]
pub enum DurationInputMessage {
    Key(Key),
    MouseUp,
    Focus(bool)
}

#[derive(PartialEq)]
enum Key {
    Right,
    Left,
    Up,
    Down,
    Digit(i32),
    Backspace,
}

impl Component for DurationInput {
    type Message = DurationInputMessage;
    type Properties = DurationInputProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            duration: Duration {
                seconds: 0,
                sub_cursor: false
            },
            selection: None,
            backspace: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let input = ctx.props().input_ref.cast::<HtmlInputElement>().unwrap();

        if msg != DurationInputMessage::Key(Key::Backspace) {
            self.backspace = false;
        }

        match msg {
            DurationInputMessage::MouseUp => {
                let cursor = cursor_location(&input);
                let selection = Selection::from_cursor(cursor);
                self.selection = Some(selection);
                self.duration.sub_cursor = false;
            },
            DurationInputMessage::Key(key) if self.selection.is_some() => match key {
                Key::Right => {
                    self.selection = Some(self.selection.unwrap().right());
                    self.selection.unwrap().select(&input);
                },
                Key::Left => {
                    self.selection = Some(self.selection.unwrap().left());
                    self.selection.unwrap().select(&input);
                },
                Key::Up => {
                    let parts = self.selection.unwrap().part_with(1);
                    self.duration.add_parts(parts);
                },
                Key::Down => {
                    let parts = self.selection.unwrap().part_with(-1);
                    self.duration.add_parts(parts);
                },
                Key::Digit(mut n) => {
                    let sub_cursor = self.duration.sub_cursor;

                    if !sub_cursor {
                        n *= 10;
                    }

                    let parts = self.selection.unwrap().part_with(n);

                    if parts.valid() {
                        if !sub_cursor {
                            let mut zero_parts = self.duration.to_parts();
                            zero_parts.zero_selection(self.selection.unwrap());
                            self.duration.seconds = zero_parts.to_seconds();
                        }

                        self.duration.add_parts(parts);

                        if sub_cursor {
                            self.selection = Some(self.selection.unwrap().right());
                        } else {
                            self.duration.sub_cursor = true;
                        }
                    }
                },
                Key::Backspace => {
                    self.duration.sub_cursor = false;

                    if self.backspace {
                        self.backspace = false;

                        self.duration.reset();
                    } else {
                        self.backspace = true;

                        let mut parts = self.duration.to_parts();
                        parts.zero_selection(self.selection.unwrap());
                        self.duration.seconds = parts.to_seconds();
                    }
                },
            },
            DurationInputMessage::Focus(focus) => {
                if focus {
                    if self.selection.is_none() {
                        let cursor = cursor_location(&input);
                        let selection = Selection::from_cursor(cursor);
                        self.selection = Some(selection);
                    }
                } else {
                    self.selection = None;
                }
            },
            _ => (),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        /*
            TODO
                - make tab focus better
                - maybe find a way to overwrite onmousedown
                - prevent dragging stuff out
                - add up/down buttons
        */

        let send_key = ctx.link().callback(|key| {
            DurationInputMessage::Key(key)
        });

        // TODO allow numpad and other numbers
        let onkeydown = Callback::from(move |event: KeyboardEvent| {
            match event.code().as_str() {
                "ArrowLeft" => send_key.emit(Key::Left),
                "ArrowRight" => send_key.emit(Key::Right),
                "ArrowUp" => send_key.emit(Key::Up),
                "ArrowDown" => send_key.emit(Key::Down),
                d if d.starts_with("Digit") => {
                    let n = d.chars().last().unwrap().to_digit(10).unwrap();
                    send_key.emit(Key::Digit(n as i32));
                },
                "Backspace" => send_key.emit(Key::Backspace),
                _ => (),
            }

            event.prevent_default();
        });

        let input_ref = ctx.props().input_ref.clone();
        let onmouseup = ctx.link().callback(|event: MouseEvent| {
            event.prevent_default();
            DurationInputMessage::MouseUp
        });

        let onpaste = Callback::from(|event: Event| {
            event.prevent_default();
        });

        let onfocus = ctx.link().callback(|event: FocusEvent| {
            DurationInputMessage::Focus(true)
        });

        let onblur = ctx.link().callback(|event: FocusEvent| {
            DurationInputMessage::Focus(false)
        });

        let ondrop = Callback::from(|event: DragEvent| {
            event.prevent_default();
        });

        html! {
            <>
                <input
                    { onblur }
                    { onfocus }
                    { ondrop }
                    { onpaste }
                    { onkeydown }
                    { onmouseup }
                    ref={ ctx.props().input_ref.clone() }
                    name={ ctx.props().name.clone() }
                    pattern="^[0-9]{1,2}:[0-5][0-9]:[0-5][0-9]$"
                    style="text-align: right;"
                    type="text"
                    value={ format!("{}", self.duration) }/>
            </>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let input = ctx.props().input_ref.cast::<HtmlInputElement>().unwrap();

        if let Some(selection) = self.selection {
            selection.select(&input);
        }
    }
}