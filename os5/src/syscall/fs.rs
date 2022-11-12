use crate::{
    mm::page_table,
    sbi,
    task::{self, Processor},
};

const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers =
                page_table::translated_byte_buffer(Processor::current_user_satp(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

/// 功能：从文件中读取一段内容到缓冲区。
///
/// 参数：fd 是待读取文件的文件描述符，切片 buffer 则给出缓冲区。
///
/// 返回值：如果出现了错误则返回 -1，否则返回实际读到的字节数。
///
/// syscall ID：63
///
/// 目前而言只能从 stdin 读入 1 个 byte
pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            assert_eq!(len, 1, "Only support len=1 in sys_read!");
            let c = loop {
                let c = sbi::console_getchar() as u8;
                if c == 0 {
                    task::suspend_current_and_run_next();
                } else {
                    break c;
                }
            };
            let mut buffers =
                page_table::translated_byte_buffer(Processor::current_user_satp(), buf, len);
            unsafe { buffers[0].as_mut_ptr().write_volatile(c) }
            1
        }
        _ => {
            panic!("Unsupported fd in sys_read");
        }
    }
}
