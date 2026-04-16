import python_rust

python_rust.rust_function()

rust_class = python_rust.RustStruct("gabriel", "saiago", 27)
print(f"o(a) [{rust_class.get_fullname()}] tem [{rust_class.age}] anos")
print(rust_class)
