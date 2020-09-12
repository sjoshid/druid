use bitflags::bitflags;
use crate::{Event, MenuDesc, LocalizedString, MenuItem, commands, Data, Env, LifeCycle};
use crate::druid::Command;
use crate::event::Event::*;
use xi_trace::{StrCow, CategoriesT, TracePayloadT, SampleGuard};
use crate::event::LifeCycle::{HotChanged, FocusChanged, AnimFrame, Size, Internal};

bitflags! {
    pub struct FilterEvents: i32 {
        const None = 0;
        const WindowSize =  1;
        const MouseDown =  2;
        const MouseUp =  3;
        const MouseMove =  4;
        const Wheel =  5;
        const KeyDown =  6;
        const KeyUp =  7;
        const Paste =  8;
        const Zoom =  9;
        const Timer =  10;
        const Command =  11;
        const WindowConnected = 12;
        const WidgetAdded = 13;
        const HotChanged = 14;
        const Size = 15;
        const AnimFrame = 16;
        const FocusChanged = 17;
        const Internal = 18;
    }
}

#[derive(Clone, Debug)]
pub struct TraceFilter {
    filter: [bool; 19],
}

impl TraceFilter {
    pub fn create_filter() -> TraceFilter {
        TraceFilter {
            filter: [false; 19]
        }
    }

    pub fn trace_event_on_off(&mut self, filter: FilterEvents) {
        self.filter[filter.bits as usize] = !self.filter[filter.bits as usize];
    }

    pub fn filter_event_payload<'a, S, C, P>(&self, event: &Event, name: S, categories: C, payload: P) -> Option<SampleGuard<'a>>
        where
            S: Into<StrCow>,
            C: Into<CategoriesT>,
            P: Into<TracePayloadT> {
        if self.filter_event(event) {
            Some(xi_trace::trace_block_payload(name, categories, payload))
        } else {
            None
        }
    }

    pub fn filter_lifecycle_payload<'a, S, C, P>(&self, event: &LifeCycle, name: S, categories: C, payload: P) -> Option<SampleGuard<'a>>
        where
            S: Into<StrCow>,
            C: Into<CategoriesT>,
            P: Into<TracePayloadT> {
        if self.filter_lifecycle(event) {
            Some(xi_trace::trace_block_payload(name, categories, payload))
        } else {
            None
        }
    }

    pub fn filter_event(&self, event: &Event) -> bool {
        match event {
            WindowConnected => {
                self.filter[FilterEvents::WindowConnected.bits as usize]
            }
            KeyUp(_) => {
                self.filter[FilterEvents::KeyUp.bits as usize]
            },
            KeyDown(_) => {
                self.filter[FilterEvents::KeyDown.bits as usize]
            }
            WindowSize(_) => {
                self.filter[FilterEvents::WindowSize.bits as usize]
            }
            MouseDown(_) => {
                self.filter[FilterEvents::MouseDown.bits as usize]
            }
            MouseUp(_) => {
                self.filter[FilterEvents::MouseUp.bits as usize]
            }
            Wheel(_) => {
                self.filter[FilterEvents::Wheel.bits as usize]
            }
            Paste(_) => {
                self.filter[FilterEvents::Paste.bits as usize]
            }
            Zoom(_) => {
                self.filter[FilterEvents::Zoom.bits as usize]
            }
            Timer(_) => {
                self.filter[FilterEvents::Timer.bits as usize]
            }
            MouseMove(_) => {
                self.filter[FilterEvents::MouseMove.bits as usize]
            }
            Command(_) => {
                self.filter[FilterEvents::Command.bits as usize]
            }
            _ => {
                false
            }
        }
    }

    pub fn filter_lifecycle(&self, event: &LifeCycle) -> bool {
        match event {
            WidgetAdded => {
                self.filter[FilterEvents::WidgetAdded.bits as usize]
            }
            HotChanged(_) => {
                self.filter[FilterEvents::HotChanged.bits as usize]
            },
            Size(_) => {
                self.filter[FilterEvents::Size.bits as usize]
            }
            AnimFrame(_) => {
                self.filter[FilterEvents::AnimFrame.bits as usize]
            }
            FocusChanged(_) => {
                self.filter[FilterEvents::FocusChanged.bits as usize]
            }
            Internal(_) => {
                self.filter[FilterEvents::Internal.bits as usize]
            }
            _ => {
                false
            }
        }
    }

    fn flip(&self, filter_type: FilterEvents) -> bool {
        self.filter[filter_type.bits as usize]
    }

    pub fn generate_menu<T: Data>(&self) -> MenuDesc<T> {
        MenuDesc::new(LocalizedString::new("Debug")).append(MenuItem::new(
            LocalizedString::new("KeyDown event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::KeyDown)
        ).selected_if(||
            self.filter[FilterEvents::KeyDown.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("WindowSize event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::WindowSize)
        ).selected_if(||
            self.filter[FilterEvents::WindowSize.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("MouseDown event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::MouseDown)
        ).selected_if(||
            self.filter[FilterEvents::MouseDown.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("MouseUp event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::MouseUp)
        ).selected_if(||
            self.filter[FilterEvents::MouseUp.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("Wheel event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::Wheel)
        ).selected_if(||
            self.filter[FilterEvents::Wheel.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("KeyUp event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::KeyUp)
        ).selected_if(||
            self.filter[FilterEvents::KeyUp.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("Paste event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::Paste)
        ).selected_if(||
            self.filter[FilterEvents::Paste.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("Zoom event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::Zoom)
        ).selected_if(||
            self.filter[FilterEvents::Zoom.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("Timer event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::Timer)
        ).selected_if(||
            self.filter[FilterEvents::Timer.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("Command event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::Command)
        ).selected_if(||
            self.filter[FilterEvents::Command.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("WindowConnected event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::WindowConnected)
        ).selected_if(||
            self.filter[FilterEvents::WindowConnected.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("MouseMove event"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::MouseMove)
        ).selected_if(||
            self.filter[FilterEvents::MouseMove.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("WidgetAdded"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::WidgetAdded)
        ).selected_if(||
            self.filter[FilterEvents::WidgetAdded.bits as usize]
        )).append(MenuItem::new(
        LocalizedString::new("HotChanged"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::HotChanged)
        ).selected_if(||
            self.filter[FilterEvents::HotChanged.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("Size"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::Size)
        ).selected_if(||
            self.filter[FilterEvents::Size.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("AnimFrame"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::AnimFrame)
        ).selected_if(||
            self.filter[FilterEvents::AnimFrame.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("FocusChanged"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::FocusChanged)
        ).selected_if(||
            self.filter[FilterEvents::FocusChanged.bits as usize]
        )).append(MenuItem::new(
            LocalizedString::new("Internal"), Command::new(commands::GENERIC_TRACE_COMMAND, FilterEvents::Internal)
        ).selected_if(||
            self.filter[FilterEvents::Internal.bits as usize]
        ))
    }
}
