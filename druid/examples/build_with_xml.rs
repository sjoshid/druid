use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::mem;

use fluent_bundle::types::AnyEq;
use xml::reader::{EventReader, XmlEvent};

use druid::{AppLauncher, Data, Lens, LensExt, LocalizedString, Widget, WindowDesc};
use druid::platform_menus::win::file::new;
use druid::widget::{Flex, Label, LabelText};

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

trait XmlTag<T>
    where T: Data
{
    fn is_container(&self) -> bool;
    fn add_child(&mut self, child_tag: Box<dyn XmlTag<T>>);
    fn get_wrapped(&mut self) -> Box<dyn Widget<T>>;
}

struct FlexRowTag<T: Data> {
    widget: Option<Flex<T>>,
    children: Vec<Box<dyn XmlTag<T>>>,
    is_container: bool,
}

impl<T: Data> FlexRowTag<T> {
    fn new() -> FlexRowTag<T> {
        FlexRowTag {
            widget: Some(Flex::row()),
            children: Vec::new(),
            is_container: true,
        }
    }
}

impl<T> XmlTag<T> for FlexRowTag<T> where T: Data {
    fn is_container(&self) -> bool {
        true
    }

    fn add_child(&mut self, mut child_tag: Box<dyn XmlTag<T>>) {
        let container_widget = self.widget.as_mut().unwrap();
        let wrapped_widget = child_tag.get_wrapped();
        container_widget.add_child(wrapped_widget);
    }

    fn get_wrapped(&mut self) -> Box<dyn Widget<T>> {
        let w = mem::take(&mut self.widget);
        Box::new(w.unwrap())
    }
}

struct LabelTag<T: Data> {
    widget: Option<Label<T>>,
    is_container: bool,
}

impl<T: Data> LabelTag<T> {
    fn new(text: impl Into<LabelText<T>>) -> LabelTag<T> {
        LabelTag {
            widget: Some(Label::new(text)),
            is_container: false,
        }
    }
}

impl<T> XmlTag<T> for LabelTag<T> where T: Data {
    fn is_container(&self) -> bool {
        false
    }

    fn add_child(&mut self, mut child_tag: Box<dyn XmlTag<T>>) {
        // do nothing. Label is not a container.
    }

    fn get_wrapped(&mut self) -> Box<dyn Widget<T>> {
        let w = mem::take(&mut self.widget);
        Box::new(w.unwrap())
    }
}

fn main() {
    let mut root_tag = parse_xml_for_root::<HelloState>();
    if root_tag.len() == 1 {
        let root_tag = &mut root_tag.remove(0);
        let root_widget = root_tag.get_wrapped();

        let main_window = WindowDesc::new_app_with_boxed_root(root_widget)
            .title(WINDOW_TITLE)
            .window_size((400.0, 400.0));

        // create the initial app state
        let initial_state = HelloState {
            name: "World".into(),
        };

        // start the application
        AppLauncher::with_window(main_window)
            .launch(initial_state)
            .expect("Failed to launch application");
    }
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn parse_xml_for_root<T>() -> Vec<Box<dyn XmlTag<T>>>
    where T: Data {
    let mut depth = 0;
    let file = File::open("C:\\Users\\joshi\\OneDrive\\druid\\druid\\examples\\test.xml").unwrap();
    let file = BufReader::new(file);
    let mut widget_stack = Vec::new();
    let mut current_children = Vec::new();
    let mut parser = EventReader::new(file);

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
                let tag_widget: Box<dyn XmlTag<T>> = widget_factory(name.to_string());
                widget_stack.push(tag_widget);
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
                if let Some(mut top) = widget_stack.pop() {
                    if top.is_container() {
                        // drain all elements from current_children and push it to top.
                        for c in current_children {
                            top.add_child(c);
                        }
                        let mut new_children = Vec::new();
                        new_children.push(top);
                        current_children = new_children;
                    } else {
                        current_children.push(top);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    current_children
}

fn widget_factory<T>(widget_name: String) -> Box<dyn XmlTag<T>>
    where T: Data
{
    match widget_name.as_str() {
        "Label" => {
            Box::new(LabelTag::new(String::from("Sujit")))
        }
        "RowFlex" => {
            Box::new(FlexRowTag::new())
        }
        _ => Box::new(FlexRowTag::new())
    }
}
