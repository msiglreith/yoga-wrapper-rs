extern crate gcc;

fn main() {
	let mut lib = gcc::Config::new();
	lib.flag("-std=c99");

	lib.file("yoga/yoga/Yoga.c")
	   .file("yoga/yoga/YGEnums.c")
	   .file("yoga/yoga/YGNodeList.c");

    lib.compile("libyoga.a");
}
