# WayQuick

WayQuick is a modern, lightweight desktop utility designed specifically for **Wayland** compositors.  
Its goal is to provide a fast, unified entry point for common desktop interactions such as wallpaper management, application launching, search, and system-level components—while staying minimal, responsive, and compositor-friendly.

Inspired by the *workflow and philosophy* of tools like **rofi**, **waybar**, and **fuzzel**, WayQuick does **not** aim to replace them directly. Instead, it focuses on offering a cohesive, extensible experience built as a single, well-integrated application.

---

## Project Goals

WayQuick aims to become a small but powerful companion for Wayland-based desktops, focusing on speed, clarity, and usability.

### 1. Wayland Wallpaper Management
- Native Wayland-friendly wallpaper handling
- Support for:
  - Static wallpapers
  - Per-output (per-monitor) configuration
- Simple and fast UI for browsing and switching wallpapers
- Designed to work across different Wayland compositors without relying on X11

### 2. Application Launcher & Search
- Keyboard-driven application launcher
- Fast fuzzy search for:
  - Installed desktop applications
  - Executable commands
- Minimal UI with instant feedback
- Optimized for quick invocation and dismissal

### 3. System Components
- Lightweight system widgets and components, such as:
  - Time and date
  - Audio and brightness status
  - Basic system indicators
- Modular design, allowing components to be enabled or disabled
- Intended to complement, not fully replace, existing status bars

---

## Design Philosophy

- **Wayland-first**: No X11 assumptions or legacy dependencies
- **Minimal but powerful**: Focus on essential workflows
- **Fast startup and low overhead**
- **Composable and extensible**: Features are built as independent modules
- **Keyboard-centric interaction**

---

## Technology Stack

WayQuick is built using a modern and robust Rust-based toolchain.

### Core Language
- **Rust**
  - Memory safety and performance
  - Strong ecosystem for systems programming
  - Ideal for responsive desktop applications

### Application Framework
- **gpui**
  - GPU-accelerated UI framework
  - Modern, reactive UI model
  - Well-suited for building fast and clean desktop interfaces

### Build & Dependency Management
- **Cargo**
  - Rust’s official build system and package manager
  - Handles compilation, dependencies, testing, and tooling

### Development Environment & Packaging
- **Nix / flake.nix**
  - Reproducible development environments
  - Declarative dependency management
  - Unified solution for:
    - Developer setup
    - CI environments
    - Application packaging and distribution

---

## Target Audience

- Wayland users who prefer minimal, keyboard-driven workflows
- Users of tiling compositors (e.g. sway, river, hyprland)
- Developers and power users looking for a cohesive desktop utility
- Anyone who wants a fast alternative to fragmented desktop tools

---

## Project Status

WayQuick is currently in **early development**.  
The API, feature set, and internal architecture may evolve as the project matures.

---

## Vision

WayQuick aims to be:
> *A fast, elegant, and Wayland-native control surface for your desktop.*

Not bloated. Not intrusive. Just quick.

---
