/*
    Working with Windows processes
    Original Article: https://friendlyuser.github.io

    Author: Irfan Ghat
*/

use std::ptr::null_mut;
#[allow(unused_imports)]
use winapi::{
    shared::minwindef::FALSE,
    um::{
        handleapi::CloseHandle,
        processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW},
        winbase::CREATE_NEW_CONSOLE,
        // Yeah...probably not going to use this
        winnt::PROCESS_ALL_ACCESS,
    },
};

use winapi::um::{
    processthreadsapi::TerminateProcess, synchapi::WaitForSingleObject, winbase::INFINITE,
};

use widestring::U16String;

#[allow(dead_code)]
// To do -> function implementation
fn terminate_process(process_info: &mut PROCESS_INFORMATION, exit_code: u32) {
    unsafe {
        TerminateProcess(process_info.hProcess, exit_code);
        WaitForSingleObject(process_info.hProcess, INFINITE);
        CloseHandle(process_info.hThread);
        CloseHandle(process_info.hProcess);
    }
}

fn main() {
    let mut startup_info: STARTUPINFOW = unsafe { std::mem::zeroed() };
    let mut process_info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    // Create new cmd process
    let command_line = U16String::from_str("cmd.exe");

    let success = unsafe {
        CreateProcessW(
            null_mut(),
            command_line.as_ptr() as *mut _,
            null_mut(),
            null_mut(),
            FALSE,
            CREATE_NEW_CONSOLE,
            null_mut(),
            null_mut(),
            &mut startup_info,
            &mut process_info,
        )
    };

    if success == FALSE {
        println!("Failed to create resources...");
        return;
    }

    println!("Process created successfully");

    unsafe {
        CloseHandle(process_info.hThread);
        CloseHandle(process_info.hProcess);
    }
}
