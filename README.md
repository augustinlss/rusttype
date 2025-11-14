```
                              ________  ________                             
                             /        |/        |                            
  ______   __    __   _______$$$$$$$$/ $$$$$$$$/__    __   ______    ______  
 /      \ /  |  /  | /       |  $$ |      $$ | /  |  /  | /      \  /      \ 
/$$$$$$  |$$ |  $$ |/$$$$$$$/   $$ |      $$ | $$ |  $$ |/$$$$$$  |/$$$$$$  |
$$ |  $$/ $$ |  $$ |$$      \   $$ |      $$ | $$ |  $$ |$$ |  $$ |$$    $$ |
$$ |      $$ \__$$ | $$$$$$  |  $$ |      $$ | $$ \__$$ |$$ |__$$ |$$$$$$$$/ 
$$ |      $$    $$/ /     $$/   $$ |      $$ | $$    $$ |$$    $$/ $$       |
$$/        $$$$$$/  $$$$$$$/    $$/       $$/   $$$$$$$ |$$$$$$$/   $$$$$$$/ 
                                               /  \__$$ |$$ |                
                                               $$    $$/ $$ |                
                                                $$$$$$/  $$/                 
```
# 🦀 rusTType  
*A fast, terminal-based typing test written in Rust.*

rusTType is a lightweight, MonkeyType-inspired typing program built entirely for the terminal.  
It focuses on speed, clarity, and minimalism.

The project aims to be a fully keyboard-driven CLI app with a clean TUI display, colored feedback, and accurate WPM/accuracy calculations.

---

## ✨ Features (Current & Planned)

### ✅ Current
- Embedded English word list  
- Randomized test generation  
- CLI flag to choose number of words  
- Basic terminal setup (alternate screen, raw mode)  
- Structure for game state handling  

### 🚧 In Development
- Real-time colored typing feedback (green/red)  
- Smooth rendering without flicker  
- Accurate WPM + accuracy calculation  
- Configurable test formats (word, time, quote)  

---

## 🧐 Project Idea

rusTType is a terminal application that allows you to run a typing test directly from the command line using:


```
tt -w 15
```

The goal is to replicate the core experience of MonkeyType, but inside a clean Rust-powered TUI:

- Words appear in dimmed text  
- Test starts on first keypress  
- Characters turn green/red immediately  
- Test ends when the target words are completed  
- WPM and accuracy are displayed at the end  

This project also serves as a learning sandbox for terminal rendering, raw mode input, and state-driven game loops in Rust.

---

## 📋 To-Do List

These are the major items left to implement.

### 🗂️ Menu & Settings
- [ ] Create a selection menu to choose test type  
    - [ ] Word test  
    - [ ] Time test (e.g., 15/30/60 sec)  
    - [ ] Quote test  
- [ ] Save menu selection to a config file  
- [ ] Load saved config on startup  
- [ ] Skip menu if saved config exists  

### 🖥️ Rendering & UI
- [ ] Display target text inside alternate screen  
- [ ] Show dimmed words before typing starts  
- [ ] Highlight current character  
- [ ] Display correct characters in green  
- [ ] Display incorrect characters in red  

### 🎮 Typing Logic
- [ ] Start test on first key  
- [ ] Prevent flickering (efficient rerendering)  
- [ ] Detect word boundaries and mistakes  
- [ ] End test on final word  

### 🧮 Statistics
- [ ] Implement WPM calculation  
- [ ] Implement raw WPM vs adjusted WPM  
- [ ] Calculate accuracy  
- [ ] Display final results summary  

---

## ▶️ Running the Program

Development:
```
cargo run -- -w 15
```



---

## 🤝 Contributions

PRs, suggestions, and issues are welcome.  
This is a learning-oriented project — feedback of any level is appreciated!



