use std::env::current_dir;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;

pub fn create_c_project(name: &str) {
    let path: PathBuf = current_dir().unwrap().join(name);

    create_dir(&path);

    let main_file_path: PathBuf = path.clone().join("main.c");
    let make_file_path: PathBuf = path.clone().join("Makefile");

    let mut main_file: File = File::create(main_file_path).unwrap();
    let mut make_file: File = File::create(make_file_path).unwrap();

    let main_file_contents = r#"
#include<stdio.h>

int main(int *argc, char **argv) {
    printf("new project");
    return 0
}
        "#;

    let make_file_content = r#"CC = gcc
CFLAGS = -Wall -Wextra -O2

TARGET = main
SRC = main.c
OBJ = main.o

all: $(TARGET)

$(TARGET): $(OBJ)
    $(CC) -o $(TARGET) $(OBJ)

$(OBJ): $(SRC)
    $(CC) $(CFLAGS) -c $(SRC) -o $(OBJ)

clean:
    rm -f $(OBJ) $(TARGET)
"#;

    main_file.write_all(main_file_contents.as_bytes());
    make_file.write_all(make_file_content.as_bytes());
}
