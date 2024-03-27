# jp2dump

This program all boxes from a jpeg2000 file to stdout.
This program was written as a learning exercise for
understanding the format of jpeg2000 files.

## Code Layout

The most important driving function `boxes.rs:print_box`.
The program's main will call this function in a loop until the program
ends because there are no more boxes.

Within `print_box`, the `GenericBox` struct is used to read a box structure
from the jpeg2000 file. Once a box is read, the box type, a 4 letter string,
is matched to the appropriate box parser, which will parse the contents
of the box into a human readable format.

All the box parsers live in `src/boxes/<box_name>.rs`.
If you plan to implement a new box parser, create a new source file,
add the module to `boxes.rs`, and update the `match` control structure
with the new box.

All box parsers must implement a function named `from_buffer` which accepts
a `Vec<u8>` which is the contents of the box. From there, the parser should
read all the binary fields from the vec into a struct.

Optionally, you can also implement the `Display` trait for better printing,
but often just using the built-in `#[derive(Debug)]` on your struct is sufficient.
