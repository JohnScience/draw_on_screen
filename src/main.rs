extern crate windows;

use windows::Win32::{
    Foundation::{
        // https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror
        GetLastError,
        HINSTANCE,
    },
    Graphics::Gdi::{
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatiblebitmap
        CreateCompatibleBitmap,
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject
        SelectObject,
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc
        GetDC,
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-bitblt
        BitBlt,
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc
        ReleaseDC,
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deletedc
        DeleteDC,
        // https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject
        DeleteObject,
        SRCCOPY,
        HBITMAP,
        HDC, CreateCompatibleDC
    },
    UI::WindowsAndMessaging::{
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagea
        LoadImageA,
        IMAGE_BITMAP,
        LR_LOADFROMFILE, 
        LR_CREATEDIBSECTION,
        LR_DEFAULTSIZE,
    },
};

// Convenience macros

macro_rules! println_val {
    ($id:ident) => {
        println!("{} = {}", stringify!($id), $id);
    };
}

macro_rules! try_call_win_api {
    ($call:stmt) => {
        println!();
        $call
        let last_err = GetLastError();
        println_val!(last_err);   
    };
}

// Largely based on
// https://stackoverflow.com/questions/67765151/my-windows-rs-script-doesnt-render-bitmap-or-doesnt-create-one-but-doesnt-c
// https://stackoverflow.com/questions/33669344/bitblt-captures-only-partial-screen
fn main() {
    unsafe {
        try_call_win_api!{
            let bmp: HBITMAP = LoadImageA(
                None, // equivalent to NULL
                "C:\\Users\\demen\\Desktop\\blackbuck.bmp", // LoadImageA converts &str to PSTR
                IMAGE_BITMAP,
                0,
                0,
                LR_LOADFROMFILE | LR_CREATEDIBSECTION | LR_DEFAULTSIZE,
            )
            // Returns HANDLE as nested type (isize). HBITMAP is a typedef for isize.
            //
            // Check the issue on newtypes vs type aliases in windows crate
            // https://github.com/microsoft/windows-rs/issues/1393
            .0
        };
        println_val!(bmp);

        try_call_win_api!{
            let dc_src = CreateCompatibleDC(None)
        };
        println_val!(dc_src);


        try_call_win_api!{
            let bmp_prev = SelectObject(dc_src, bmp)
        };
        println_val!(bmp_prev);
        
        try_call_win_api!{
            let dc_dst = GetDC(None)
        };
        println_val!(dc_dst);

        try_call_win_api!{
            let is_success = BitBlt(
                dc_dst,
                0, 
                0, 
                512, 
                512,
                dc_src,
                0, 
                0, 
                SRCCOPY,
            ).as_bool()
        };
        println_val!(is_success);

        ReleaseDC(None, dc_dst);
        SelectObject(dc_src, bmp_prev);
        DeleteDC(dc_src);
        DeleteObject(bmp);
    }
}