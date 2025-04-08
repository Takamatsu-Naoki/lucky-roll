# Dice Triplet

**Test your luck with a simple dice roller. This program keeps spinning the dice until you roll a rare triplet – because sometimes, life’s all about chance.**

## How It Works

The program simulates rolling three dice repeatedly until all three dice show the same number (a triplet). It’s a fun, simple way to test your luck and pass the time.

For the **impatient**, there's an option to automatically roll the dice over and over, because sometimes waiting for fate is just too slow!

## Features
- Roll three dice until a triplet is rolled.
- Adjust the number of frames (dice rolls) and the delay between them.
- Displays a simple slot-machine-style animation for each roll.
- **Auto mode** for those who just can’t wait for their luck to unfold.

---

# Setup & Usage

You can use **Cargo** or **Nix** to set up the environment, build, and run the program.  
Choose **one** of the following methods:

## Option 1: Using Cargo

### **1. Development Environment**
Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

### **2. Building the Program**
```bash
cargo build --release
```
The compiled binary will be located at:
```
./target/release/lucky-roll
```

### **3. Running the Program**

To start the dice-rolling fun, simply run:

```bash
cargo run --release
```

If luck is on your side, you’ll witness the glorious triplet! The program will congratulate you on your rare achievement.

Too impatient to let fate do its thing? Use the `-a` flag:

```bash
cargo run --release -- -a
```

---

## Option 2: Using Nix

### **1. Development Environment**
You can set up the environment without manually installing dependencies.

#### **Method A: Automatic setup (`nix-direnv` users only)**
If you have [`nix-direnv`](https://github.com/nix-community/nix-direnv) installed, run:
```bash
direnv allow
```
After this, entering the project directory will automatically set up the environment.

#### **Method B: Manual setup**
If you are not using `nix-direnv`, manually enter the development environment:
```bash
nix develop
```

### **2. Building the Program**
```bash
nix build
```
This will generate the compiled binary at:
```
./result/bin/lucky-roll
```

### **3. Running the Program**

To start the dice-rolling fun, simply run:

```bash
nix run
```

If luck is on your side, you’ll witness the glorious triplet! The program will congratulate you on your rare achievement.

Too impatient to let fate do its thing? Use the `-a` flag:

```bash
nix run -- -a
```

---

## Configuration

You can customize the program's behavior with the following arguments:

- `-f` or `--frames`: Number of frames (dice rolls) before the final result. Default is 18.
- `-s` or `--seconds`: Seconds until the result. Default is 1.8.
- `-a` or `--auto`: Automatically roll the dice. Default is false.

Example:

```bash
cargo run --release -- -f 10 -s 1.5
```
(or with Nix: `nix run -- -f 10 -s 1.5`)

## License

This project is open-source and available under the [MIT License](LICENSE).
