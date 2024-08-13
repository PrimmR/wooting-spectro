use crate::audio;
use crate::options::*;
use crate::themes::ThemeChoice;

use std::str::FromStr;
use std::sync::RwLock;
use std::sync::{mpsc::SyncSender, Arc};

use strum::IntoEnumIterator;
use tray_icon::{menu::*, TrayIconBuilder};

use winit::event_loop::{EventLoop, EventLoopProxy};
#[cfg(windows)]
use winit::platform::windows::EventLoopBuilderExtWindows as _;

const TICK: &str = "•";
const NO_TICK: &str = "";

/// Stores the potential messages that can be sent by the tray
#[derive(Debug)]
pub enum TrayMessage {
    ThemeReload,
    Refresh,
    Quit,
}

#[derive(Debug)]
enum EventLoopMessage {
    Redraw(Arc<RwLock<Options>>),
}

struct Loop {
    menu: Menu,
}

impl winit::application::ApplicationHandler<EventLoopMessage> for Loop {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: EventLoopMessage) {
        match event {
            EventLoopMessage::Redraw(opt) => draw_menu(& self.menu, &opt),
        }
    }
}

/// Creates an application within the system tray that can send [TrayMessage]s to the main thread
pub fn spawn_tray(opt: Arc<RwLock<Options>>, tx: SyncSender<TrayMessage>) {
    let tray_menu = Menu::new();

    draw_menu(&tray_menu, &opt);

    // Since winit doesn't use gtk on Linux, and we need gtk for
    // the tray icon to show up, we need to spawn a thread
    // where we initialize gtk and create the tray_icon
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        use tray_icon::menu::Menu;

        let icon = get_icon();

        gtk::init().unwrap();
        let _tray_icon = TrayIconBuilder::new().with_menu(Box::new(Menu::new())).with_icon(icon).build().unwrap();

        gtk::main();
    });

    #[cfg(not(target_os = "linux"))]
    let icon = get_icon();
    #[cfg(not(target_os = "linux"))]
    let _tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu.clone()))
            .with_title("Wooting Spectro")
            .with_tooltip("Wooting Spectro - Running")
            .with_menu_on_left_click(true)
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    // #[cfg(windows)]
    let event_loop = EventLoop::<EventLoopMessage>::with_user_event().with_any_thread(true).build().unwrap();

    let proxy: EventLoopProxy<EventLoopMessage> = event_loop.create_proxy();

    MenuEvent::set_event_handler(Some(move |e| handle_event(e, &tx, &opt, &proxy)));

    let _ = event_loop.run_app(&mut Loop { menu: tray_menu });
}

fn empty_menu(menu: &Menu) {
    while menu.remove_at(0).is_some() {}
}

fn draw_menu(menu: &Menu, opt: &Arc<RwLock<Options>>) {
    empty_menu(menu);

    let about = PredefinedMenuItem::about(
        None,
        Some(AboutMetadata {
            name: Some(String::from("Wooting Spectro")),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
            comments: Some(env!("CARGO_PKG_DESCRIPTION").to_string()),
            license: Some(env!("CARGO_PKG_LICENSE").to_string()),
            authors: Some(env!("CARGO_PKG_AUTHORS").split(';').map(|a| a.trim().to_string()).collect::<Vec<_>>()),
            ..Default::default()
        }),
    );

    let devices = SubmenuBuilder::new().text("Devices").enabled(true).build().unwrap();
    let selected = matches!(opt.read().unwrap().device, ActiveDevice::Default);
    devices
        .append(&MenuItem::with_id(
            "Devices:Default",
            format!("{} Default", if selected { TICK } else { NO_TICK }),
            true,
            None,
        ))
        .unwrap();
    for name in audio::get_devices().unwrap() {
        let selected = matches!(&opt.read().unwrap().device,ActiveDevice::Named(s) if s == &name);
        devices
            .append(&MenuItem::with_id(
                format!("Devices:{name}"),
                format!("{} {name}", if selected { TICK } else { NO_TICK }),
                true,
                None,
            ))
            .unwrap();
    }

    let themes = SubmenuBuilder::new().text("Themes").enabled(true).build().unwrap();
    for theme in ThemeChoice::iter() {
        let selected = theme == opt.read().unwrap().theme;
        themes
            .append(&MenuItem::with_id(
                format!("Themes:{theme}"),
                format!("{} {theme}", if selected { TICK } else { NO_TICK }),
                true,
                None,
            ))
            .unwrap();
    }

    let caps_display_name =
        format!("{} Caps Lock Indicator", if opt.read().unwrap().caps_active { TICK } else { NO_TICK });
    let toggle_caps = MenuItem::with_id("Caps:", caps_display_name, true, None);

    let refresh = MenuItem::with_id("Refresh:", "Refresh", true, None);
    let quit = MenuItem::with_id("Quit:", "Quit", true, None);

    menu.append_items(&[
        &about,
        &PredefinedMenuItem::separator(),
        &devices,
        &themes,
        &PredefinedMenuItem::separator(),
        &toggle_caps,
        &PredefinedMenuItem::separator(),
        &refresh,
        &quit,
    ])
    .unwrap();
}

fn handle_event(
    event: MenuEvent,
    tx: &SyncSender<TrayMessage>,
    opt: &Arc<RwLock<Options>>,
    proxy: &EventLoopProxy<EventLoopMessage>,
) {
    let id = &event.id().0;
    let mut tree = id.split(':');
    if let Some(base) = tree.next() {
        if let Err(e) = match base {
            "Devices" => {
                opt.write().unwrap().device = ActiveDevice::from_string(tree.next().unwrap().to_string());
                tx.send(TrayMessage::Refresh)
            }
            "Themes" => {
                opt.write().unwrap().theme = ThemeChoice::from_str(tree.next().unwrap()).unwrap();
                tx.send(TrayMessage::ThemeReload)
            }
            "Caps" => {
                opt.write().unwrap().caps_active ^= true;
                Ok(())
            }
            "Refresh" => tx.send(TrayMessage::Refresh),
            "Quit" => tx.send(TrayMessage::Quit),
            "QuickQuit" => std::process::exit(0),
            _ => Ok(()),
        } {
            eprintln!("{e}")
        }
    }

    proxy.send_event(EventLoopMessage::Redraw(opt.clone())).unwrap();
}

fn get_icon() -> tray_icon::Icon {
    let rgba = include_bytes!(concat!(env!("OUT_DIR"), "/icon.rgba8"));
    let width = u32::from_be_bytes(*include_bytes!(concat!(env!("OUT_DIR"), "/icon-w")));
    let height = u32::from_be_bytes(*include_bytes!(concat!(env!("OUT_DIR"), "/icon-h")));

    tray_icon::Icon::from_rgba(rgba.to_vec(), width, height).expect("Failed to load icon")
}
