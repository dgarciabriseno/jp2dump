# jp2dump

This program all boxes from a jpeg2000 file to stdout.
This program was written as a learning exercise for
understanding the format of jpeg2000 files.

## Usage
```
cargo run path/to/jpeg2000.jp2
```

Sample output:
```
GenericBox { length: 12, box_type: "jP  ", contents: [13, 10, 135, 10] }
FTypeBox { brand: "jpx ", minor_version: 0, cl: ["jpx ", "jp2 ", "jpxb"] }
ReaderRequirements { mask_length: 3, fully_understand_aspects: [255, 192, 0], display_contents_mask: [0, 63, 240], flags: [StandardFlag { flag: 5, mask: [128, 0, 0] }, StandardFlag { flag: 46, mask: [64, 0, 0] }, StandardFlag { flag: 2, mask: [32, 0, 0] }, StandardFlag { flag: 18, mask: [16, 0, 0] }, StandardFlag { flag: 19, mask: [8, 0, 0] }, StandardFlag { flag: 1, mask: [4, 0, 0] }, StandardFlag { flag: 8, mask: [2, 0, 0] }, StandardFlag { flag: 12, mask: [1, 0, 0] }, StandardFlag { flag: 31, mask: [0, 128, 0] }, StandardFlag { flag: 20, mask: [0, 64, 0] }], vendor_features: [] }
GenericBox { length: 45, box_type: "jp2h", contents: [0, 0, 0, 22, 105, 104, 100, 114, 0, 0, 16, 0, 0, 0, 16, 0, 0, 1, 7, 7, 1, 0, 0, 0, 0, 15, 99, 111, 108, 114, 1, 0, 0, 0, 0, 0, 17] }
GenericBox { length: 8, box_type: "jpch", contents: [] }
GenericBox { length: 8, box_type: "jplh", contents: [] }
Fragment Table:
FragmentListBox { nf: 1, offset: 6581, len: 1071559, index: 1 }
...
Error occurred while reading jpeg2000 file: failed to fill whole buffer
```

The error at the end is expected when the whole file is read.

## Developer Documentation

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
