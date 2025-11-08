# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

PersianSubtitleFixer is a Qt5-based desktop application that converts Persian subtitle files from Windows-1256 (Arabic) encoding to UTF-8. The application provides a GUI for opening, previewing, and saving subtitle files with proper encoding.

**Current Version:** 0.3 TP1

## Build System

### Building the Application

```bash
# Create build directory
mkdir build
cd build

# Generate Makefile using qmake
qmake ..

# Compile the application
make
```

### Installation (Linux)

```bash
# Make executable
chmod +x ./persiansubtitlefixer

# Install system-wide
sudo cp persiansubtitlefixer /usr/bin/PersianSubtitleFixer
sudo chmod +x /usr/bin/PersianSubtitleFixer

# Run the application
PersianSubtitleFixer
```

### Running from Build Directory

```bash
./persiansubtitlefixer
```

The application can also accept a subtitle file as a command-line argument:

```bash
./persiansubtitlefixer /path/to/subtitle.srt
```

## Architecture

### Core Components

**Main Entry Point (`main.cpp`)**
- Handles command-line arguments for opening files directly
- Initializes the Qt application and main form
- Sets window title based on opened file

**Main Form (`form1.cpp`, `form1.h`, `form1.ui`)**
- Primary application window (inherits from QWidget)
- Handles all subtitle file operations (open, save, preview)
- Implements drag-and-drop functionality for SRT files
- Manages text encoding conversion between Windows-1256 and UTF-8
- Uses `GlobalFileName` global variable to track currently opened file

**About Dialog (`about.cpp`, `about.h`, `about.ui`)**
- Displays application information and GPL license
- Loads LICENSE text from embedded resources

### Key Features Implementation

**Text Encoding Conversion**
- Uses `QTextStream::setCodec()` to handle encoding
- "Windows Arabic - ناسالم" option: reads as `windows-1256`
- "UTF-8 - سالم" option: reads as `utf-8`
- Always saves files in UTF-8 encoding

**File Operations**
- `Form1::openFile()`: Core function for reading files with specified codec
- Supports opening via file dialog, drag-and-drop, or command-line argument
- Save operation always writes UTF-8 encoded files

**UI Elements**
- Uses system theme icons (`QIcon::fromTheme()`)
- ComboBox for codec selection
- QTextEdit for subtitle preview with customizable font
- Keyboard shortcuts: Ctrl+O (open), Ctrl+S (save), Ctrl+F (font), Ctrl+Q/Esc (quit)

### Qt-Specific Files

- **UI Files**: `form1.ui`, `about.ui` - Qt Designer form definitions
- **Resources**: `resources.qrc` - Embedded resources (LICENSE, images)
- **Generated Files** (in `.gitignore`):
  - `ui_*.h` - Auto-generated from .ui files
  - `moc_*.cpp` - Auto-generated meta-object code
  - `qrc_*.cpp` - Auto-generated resource code

## Development Notes

### Qt Framework Requirements

- **Qt Version**: Qt5
- **Required Qt Modules**: QtCore, QtGui, QtWidgets
- No `.pro` file present - project configuration needs to be created if using Qt Creator

### Global State

The application uses a global variable `GlobalFileName` (defined in `form1.cpp`) to track the currently opened file path across different functions and event handlers.

### Encoding Handling

The core functionality revolves around QTextCodec:
- Windows-1256 (Windows Arabic) is used for Persian subtitles created on Windows systems
- Files are converted to UTF-8 for proper display on modern systems
- The user selects source encoding via dropdown before opening

### License

This project is licensed under GPL v3. License text is embedded as a resource and displayed in the About dialog.
