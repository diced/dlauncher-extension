# Dlauncher Example Extension
In this repo you can find an example extension that should work for Dlauncher. 
Extensions as of now are pretty weird and jank, yet they do work.

The code is full of comments which should be helpful in guiding you to creating your own extension later.

# Building
Dlauncher will need a `.so` file to run your extension, so you will need to have the following in your Cargo.toml
```toml
[lib]
crate-type = ["cdylib"]
```
This is already included in this example repository.

Go ahead and clone the repository and build it.
```bash
git clone https://github.com/diced/dlauncher-extension
cargo build --release
```

# Running
After this you can find the file at `target/release/libdlauncher_zws.so`

You will need to modify your `~/.config/dlauncher/dlauncher.toml`'s `main.extensions` property to have `libdlauncher_zws.so` inside of its array.
```toml
[main]
# ...
extensions = ["libdlauncher_zws.so"]
# ...
```

Now copy the .so to `~/.config/dlauncher/extensions/` and restart Dlauncher.
```shell
cp target/release/libdlauncher_zws.so ~/.config/dlauncher/extensions/
pkill dlauncher
dlauncher
```

Now you can type "zero" and you can see it pop up at the bottom.

# Extra Stuff
In the extension I added some stuff for prefixes and such, which aren't used in this extension, yet it is used as an example for extensions that want to use a prefix.

I encourage you to explore the dlauncher documentation to see what is possible, since you can do pretty much anything you want.

Docs: [https://docs.rs/dlauncher](https://docs.rs/dlauncher)