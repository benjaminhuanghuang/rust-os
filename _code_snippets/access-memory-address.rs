// cast 0xb8000 as an mutable raw poiniter
// covert it to a mutalbe reference by dereferening it through *
// borrow it through &mut
buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
