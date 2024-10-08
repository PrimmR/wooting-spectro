# Wooting Spectro

Turn your Wooting keyboard into an audio spectrum analyser!

This application lives in the system tray and displays a frequency spectrum onto the keyboard's LED matrix. There are various colour themes available to choose from.

This application is currently only available for Windows, but Linux support may come eventually if some dependencies are updated.

*Disclaimer: Although I've not encountered any issues when using this program with my own keyboard, I cannot guarantee that this is the case for all setups - please use at your own risk.*

## Compatibility

|Wooting Device|Support|
|---|---|  
|One|Untested|
|Two HE|Working|
|60 HE|Working|
|UwU|Unsupported|

## Usage
Firstly, download the latest release from [Releases](https://github.com/PrimmR/wooting-spectro/releases/) or [build it from source](#build) yourself.  

You can also install this app using cargo:  
`cargo install --git https://github.com/PrimmR/wooting-spectro --features cli`  
Then you can run it by calling `wooting-spectro` from the command line.

The executable is portable but does create one other file to store user preferences, so store it in whichever directory you see fit.

Then make sure your Wooting keyboard is plugged in and simply run the application, and it should appear in your system tray. On Windows, it's likely to be hidden by default. You should know that it's working if your keyboard's LEDs are all off. They should now light up when you play audio from your machine.

If you right click on the icon, you will enter the menu, where you can change settings such as the colour theme being displayed and the device for the keyboard to 'listen' to. If a device doesn't show up when it's just been plugged in, press the *Refresh* button to update the list. All of these options are saved when the application is exited, so you don't need to change these settings every time.

When you've finished, make sure to quit using the tray icon menu (which can be accessed with a right-click), to return your keyboard to its original theme.

## Build

Because the [SDK](https://github.com/WootingKb/wooting-rgb-sdk) is written in C, `libclang` needs to be installed to build this project. You can install it on Windows with the following command:  
`winget install LLVM.LLVM`

You will also need [`cargo`](https://www.rust-lang.org/tools/install), to build the main Rust application.

Then you can just download this repository and run `cargo build --release` in the wooting-spectro directory. The executable should be in `target/release`, where it can be moved and executed wherever you wish. 

## Issues
Any bug reports are greatly appreciated, however the application doesn't propagate many errors to the user on release builds. If you can, please try to recreate any error under a `dev` build, but just giving steps to recreate the issue would also be just as useful.

## Gallery
**Showcase Video:**
[![Showcase Video](https://raw.githubusercontent.com/PrimmR/wooting-spectro/main/gallery/ShowcaseThumb.png)](https://raw.githubusercontent.com/PrimmR/wooting-spectro/main/gallery/Showcase.mp4)
