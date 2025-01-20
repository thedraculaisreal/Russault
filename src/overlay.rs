use windows::{ core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*,};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

pub fn create_overlay() -> HWND {
    unsafe {
	// we use none in getmodulehandle to get the handle to our window that we are creating.
	let h_instance = GetModuleHandleW(None).unwrap().into();
	let class_name = "overlay_class";
	let overlay_name = "russault_overlay";

	let wc = WNDCLASSW {
	    hInstance: h_instance,
	    // PWSTR is a windows pointer to wide string, which is a pointer mutable utf16 chars.
	    lpszClassName: PCWSTR(class_name.encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_mut_ptr()),
	    lpfnWndProc: Some(WNDPROC),
	    style: CS_HREDRAW | CS_VREDRAW,
	    // defaulting rest of params that we dont care about.
	    ..Default::default()
	};
	RegisterClassW(&wc);

	let hwnd = CreateWindowExW(
	    // we want our overlay to sit on top of everything and be transparent.
	    WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST,
	    PWSTR(class_name.encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_mut_ptr()),
	    PWSTR(overlay_name.encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_mut_ptr()),
	    WS_POPUP,
	    0,0,800,600,
	    Some(HWND(std::ptr::null_mut())),
	    Some(HMENU(std::ptr::null_mut())),
	    Some(h_instance),
	    Some(std::ptr::null_mut()),
	);
	// LWA_ALPHA sets are opacity of backgroudn to be transparent.
	SetLayeredWindowAttributes(hwnd.expect("REASON"), windows::Win32::Foundation::COLORREF(0), 255, LWA_ALPHA);
	ShowWindow(hwnd.expect("REASON"), SW_SHOW);
	// returning our hwnd so we can use it in our drawing with wgpu later, once ive gotten to it.
	hwnd.expect("REASON")
    }
}
