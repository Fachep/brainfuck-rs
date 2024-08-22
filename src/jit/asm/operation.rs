#[repr(u8)]
pub enum Operation {
    Mov = 0b1,
}

/*
48 89 0c 25 01 00 00 00    mov    %rcx,0x1
89 0c 25 01 00 00 00       mov    %ecx,0x1
66 89 0c 25 01 00 00 00    mov    %cx,0x1
88 0c 25 01 00 00 00       mov    %cl,0x1
88 2c 25 01 00 00 00       mov    %ch,0x1
48 89 c8                   mov    %rcx,%rax
89 c8                      mov    %ecx,%eax
66 89 c8                   mov    %cx,%ax
88 c8                      mov    %cl,%al
88 ec                      mov    %ch,%ah
ff 34 25 01 00 00 00       push   0x1
51                         push   %rcx
66 51                      push   %cx
59                         pop    %rcx
66 59                      pop    %cx
8f 04 25 01 00 00 00       pop    0x1
*/
