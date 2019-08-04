extern crate gtk;

use gtk::*;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Predefined messages that will be used by the UI upon certain conditions.
const MESSAGES: [&str; 3] = ["Ouch! You hit me!", "...", "Thanks!"];

#[repr(u8)]
enum Message {
    Hit,
    Dead,
    Heal,
}
pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Content {
    pub container: Box,
    pub health: Label,
    pub message: Label,
}

pub struct Header {
    pub container: HeaderBar,
    pub hit: Button,
    pub heal: Button,
}
pub struct HealthComponent(AtomicUsize);

impl HealthComponent {
    fn new(initial: usize) -> HealthComponent {
        HealthComponent(AtomicUsize::new(initial))
    }

    fn health(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }

    fn subtract(&self, value: usize) -> usize {
        let current = self.0.load(Ordering::SeqCst);
        let new = if current < value { 0 } else { current - value };
        self.0.store(new, Ordering::SeqCst);
        new
    }

    fn heal(&self, value: usize) -> usize {
        let original = self.0.fetch_add(value, Ordering::SeqCst);
        original + value
    }
}
impl App {
    fn new(health: &HealthComponent) -> App {
        let window = Window::new(WindowType::Toplevel);

        let header = Header::new();

        let content = Content::new(health);

        window.set_titlebar(Some(&header.container));

        window.set_title("App Name");

        window.set_wmclass("app-name", "App Name");

        Window::set_default_icon_name("iconname");

        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            content,
        }
    }
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title(Some("App Name"));

        container.set_show_close_button(true);

        let hit = Button::new_with_label("Hit!");
        let heal = Button::new_with_label("Heal!");

        hit.get_style_context().add_class("destructive-action");
        heal.get_style_context().add_class("suggested-action");

        container.pack_start(&hit);
        container.pack_end(&heal);

        Header {
            container,
            hit,
            heal,
        }
    }
}

impl Content {
    fn new(health: &HealthComponent) -> Content {
        let container = Box::new(Orientation::Vertical, 0);
        let health_info = Box::new(Orientation::Horizontal, 0);
        let health_label = Label::new(Some("Current Health:"));
        let health = Label::new(Some(health.health().to_string().as_str()));

        health_info.set_halign(Align::Center);
        health_label.set_halign(Align::Start);

        health.set_halign(Align::Start);

        health_info.pack_start(&health_label, false, false, 5);
        health_info.pack_start(&health, true, true, 5);

        let message = Label::new(Some("Hello"));

        container.pack_start(&health_info, true, false, 0);
        container.pack_start(&Separator::new(Orientation::Horizontal), false, false, 0);
        container.pack_start(&message, true, false, 0);

        Content {
            container,
            health,
            message,
        }
    }
}
fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK");
        process::exit(1);
    }
    let health = Arc::new(HealthComponent::new(10));
    let app = App::new(&health);

    app.window.show_all();

    // Program the Hit button to subtract health.
    let current_health = health.clone();
    let message = app.content.message.clone();
    let info = app.content.health.clone();

    app.header.hit.clone().connect_clicked(move |_| {
        let new_health = current_health.subtract(1);
        let action = if new_health == 0 {
            Message::Dead
        } else {
            Message::Hit
        };
        message.set_label(MESSAGES[action as usize]);
        info.set_label(new_health.to_string().as_str());
    });

    let other_health = health.clone();
    let other_message = app.content.message.clone();
    let other_info = app.content.health.clone();

    app.header.heal.clone().connect_clicked(move |_| {
        let new_health = other_health.heal(2);
        other_message.set_label(MESSAGES[Message::Heal as usize]);
        other_info.set_label(new_health.to_string().as_str());
    });
    gtk::main();
}
