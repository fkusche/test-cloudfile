use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;            // contains encode_wide
use std::iter::once;
use winapi::um::fileapi;

// https://stackoverflow.com/questions/48586816/converting-raw-pointer-to-16-bit-unicode-character-to-file-path-in-rust
// https://stackoverflow.com/questions/39068718/convert-a-vecu16-or-vecwchar-to-a-str
unsafe fn u16_ptr_to_string_lossy(ptr: *const u16) -> String {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    String::from_utf16_lossy(slice)
}

fn main() {
    //let directory = r"C:\Users\xxx\OneDrive\Documents\*.*";
    //let directory = r"C:\Users\xxx\iCloudDrive\Bilder\*.*";
    let directory = r"*.*";

    let directory: Vec<u16> = OsStr::new(directory).encode_wide().chain(once(0)).collect();
    let mut fd = winapi::um::minwinbase::WIN32_FIND_DATAW::default();
    let h = unsafe {
        fileapi::FindFirstFileW( directory.as_ptr(), &mut fd )
    };

    if h == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        panic!("Could not list directory")
    }

    loop {
        let filename = unsafe { u16_ptr_to_string_lossy( &fd.cFileName as *const u16 ) };

        let pinned = fd.dwFileAttributes & winapi::um::winnt::FILE_ATTRIBUTE_PINNED != 0;
        let unpinned = fd.dwFileAttributes & winapi::um::winnt::FILE_ATTRIBUTE_UNPINNED != 0;
        let recall_open = fd.dwFileAttributes & winapi::um::winnt::FILE_ATTRIBUTE_RECALL_ON_OPEN != 0;
        let recall_access = fd.dwFileAttributes & winapi::um::winnt::FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS != 0;

        println!("{} {} {} {} {}", pinned, unpinned, recall_open, recall_access, filename);

        unsafe {
            if fileapi::FindNextFileW(h, &mut fd) == 0 {
                break;
            }
        }
    }
}
