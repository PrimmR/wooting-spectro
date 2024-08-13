use strum::IntoEnumIterator;

use crate::options::Options;
use crate::themes::ThemeChoice;
use crate::tray::TrayMessage;

use std::sync::mpsc::SyncSender;
use std::sync::{Arc, RwLock};
use std::thread::{sleep, spawn};
use std::time::Duration;

const DELAY: Duration = Duration::from_secs(5);

fn next_theme(current: &ThemeChoice) -> ThemeChoice {
    let mut choices = ThemeChoice::iter().cycle();
    choices.find(|x| x == current);
    choices.next().unwrap()
}

pub fn cycle(tx: SyncSender<TrayMessage>, opt: Arc<RwLock<Options>>) {
    spawn(move || loop {
        let current = next_theme(&opt.read().unwrap().theme);

        opt.write().unwrap().theme = current;
        tx.send(TrayMessage::ThemeReload).unwrap();

        sleep(DELAY)
    });
}
