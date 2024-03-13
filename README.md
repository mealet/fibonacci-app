# 😵‍💫 Fibonacci App
Fibonacci App - is simple app for getting **fibonacci number** by writing index in input.</br>
I've created this app for learning tauri and gui on rust.<br/>
Web-page styled by Bootstrap Presets.<br/>
Main language is _Russian_.<br/><br/>
![image](https://github.com/mealet/fibonacci-app/assets/110933288/6279ca48-a1c0-4096-9512-de07da30bd1c)

## 👀 Installation
### Easy way
1. Go to the side of this repository
2. Open "Releases"
3. Download `.exe` file and start it

### Build way
1. Download code and go to its directory
2. Install tauri-cli
```
cargo install tauri-cli
```
3. Build app
```
cargo tauri build
```
4. **OR** run in developer mode
```
cargo tauri dev
```

## 🧐 Files & Folders
- `ui` folder contains web-app with bootstrap css file. <br/>
  - `index.html` is main html file.
  - `style.css` is main styling file.
  - `bootstrap` folder contains bootstrap css presets.
- `src-tauri` folder contains app configuration file and 2 rust scripts.
  - `main.rs` is main file which starting web-page like tauri app
  - `math.rs` is a simple module with fibonacci recursive function.

## 🔗 Links
- Tauri - https://tauri.app/
- Rust - https://www.rust-lang.org/
- Bootstrap - https://getbootstrap.com/
