use std::env::current_dir;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;

pub fn create_assembly_project(name: &str) {
    let path: PathBuf = current_dir().unwrap().join(name);

    create_dir(&path);

    let main_file_path: PathBuf = path.clone().join("main.s");
    let make_file_path: PathBuf = path.clone().join("Makefile");

    let mut main_file: File = File::create(main_file_path).unwrap();
    let mut make_file: File = File::create(make_file_path).unwrap();

    let main_file_contents = r#"section .text
global _start

_start:
    mov rax, 1          ; system call for write
    mov rdi, 1          ; file descriptor 1 is stdout
    mov rsi, message    ; address of string to output
    mov rdx, 13         ; number of bytes
    syscall             ; invoke system call
    mov rax, 60         ; system call for exit
    xor rdi, rdi       ; exit code 0
    syscall

section .data
message db 'Hello, test!', 10 ; length in bytes"#;

    let make_file_content = r#"ASM = nasm
LD  = ld

ASMFLAGS = -f elf64
LDFLAGS  =

TARGET = main
SRC = main.s
OBJ = main.o

all: $(TARGET)

$(TARGET): $(OBJ)
    $(LD) $(LDFLAGS) -o $(TARGET) $(OBJ)

$(OBJ): $(SRC)
    $(ASM) $(ASMFLAGS) $(SRC) -o $(OBJ)

clean:
    rm -f $(OBJ) $(TARGET)
"#;

    main_file.write_all(main_file_contents.as_bytes());
    make_file.write_all(make_file_content.as_bytes());
}
