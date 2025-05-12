# GooeyGit Project Plan

## Overview
GooeyGit is a minimalist GUI git client built with Rust, GTK4, and libadwaita. The application aims to provide essential git functionality with a clean, modern GNOME-style interface.

## Architecture

### Core Components
1. **UI Layer** (`src/ui/`)
   - Main window
   - Repository view
   - Commit history view
   - Diff viewer
   - Status view

2. **Git Operations** (`src/git/`)
   - Repository management
   - Branch operations
   - Commit operations
   - Remote operations

3. **Models** (`src/models/`)
   - Repository model
   - Commit model
   - Branch model
   - Status model

## Feature Roadmap

### Phase 1: Basic Repository Viewing
- [x] Project setup
- [ ] Basic UI layout
- [ ] Repository opening
- [ ] File status display
- [ ] Simple commit viewing

### Phase 2: Basic Git Operations
- [ ] Stage/unstage files
- [ ] Create commits
- [ ] Switch branches
- [ ] Pull/push operations

### Phase 3: Advanced Features
- [ ] Diff viewing
- [ ] Interactive rebase
- [ ] Stash management
- [ ] Git config management

## Implementation Details

### Git Functionality
We'll use the `git2` crate, which provides Rust bindings for libgit2, to handle Git operations.

### UI Design
The application will follow GNOME's Human Interface Guidelines:
- Use libadwaita widgets for a modern GNOME look and feel
- Implement adaptive layouts for responsiveness
- Use symbolic icons from the GNOME icon set
- Follow GNOME's spacing and layout conventions

### Data Flow
1. Git operations are performed through the `git2` API
2. Results are converted to our model types
3. UI components observe and display these models
4. User actions trigger git operations, completing the cycle 