use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::prelude::OsStrExt;
use std::ptr;
use std::{
    mem,
    thread::sleep,
    time::Duration,
};
use winapi::shared::windef::HWND__;
use winapi::um::winuser::{FindWindowW, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, SendInput, SetForegroundWindow, ShowWindow};

struct GameInfo {
    is_running: bool,
    game_process: *mut HWND__
}

// "ws://localhost:5051/ws/seeder?groupid={}"
// "ws://seeder.gametools.network:5252/ws/seeder?groupid={}"
fn main() {
    // anti afk thread, runs when game is in "joined" state
    loop {
        let game_info = is_running();
        if game_info.is_running {
            unsafe {
                // if game is not running
                SetForegroundWindow(game_info.game_process);
                ShowWindow(game_info.game_process, 9);
                sleep(Duration::from_millis(1808));
                key_enter(0x45);
                sleep(Duration::from_millis(100));
                ShowWindow(game_info.game_process, 6);
            }
        }
        sleep(Duration::from_secs(120));
    };
}

fn is_running() -> GameInfo {
    unsafe {
        let window: Vec<u16> = OsStr::new("Battlefield™ V")
            .encode_wide()
            .chain(once(0))
            .collect();
        let window_handle = FindWindowW(std::ptr::null_mut(), window.as_ptr());
        let no_game: *mut HWND__ = ptr::null_mut();
        GameInfo{ is_running: window_handle != no_game, game_process: window_handle }
    }
}

unsafe fn create_input(key_code: u16, flags: u32) -> INPUT {
    let mut input = mem::zeroed::<INPUT>();
    input.type_ = INPUT_KEYBOARD;
    let mut ki = input.u.ki_mut();
    ki.wVk = key_code;
    ki.dwFlags = flags;
    input
}

unsafe fn key_down(key_code: u16) {
    let mut input = create_input(key_code, 0);
    SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
}

unsafe fn key_up(key_code: u16) {
    let mut input = create_input(key_code, KEYEVENTF_KEYUP);
    SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
}

unsafe fn key_enter(key_code: u16) {
    key_down(key_code);
    sleep(Duration::from_millis(154));
    key_up(key_code);
}