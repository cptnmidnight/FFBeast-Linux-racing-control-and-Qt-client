# Project Guidelines & Standards

This document outlines the mandatory rules, technical standards, and UI direction for the FFBeast Linux project.

## 📌 Project Rules (Mandatory)

To maintain code quality and scalability, all developers must follow these strict rules:

- **Strict Modularization**: Each `struct`, `enum`, or `trait` must reside in its own file. Avoid monolithic files.
- **KISS Principle (Keep It Simple, Stupid)**: Classes, methods, and files must be simple and focused. Prioritize componentization over complexity.
- **Offline First Assets**: No external fonts, images, or icons (CDN). All assets must be bundled within the application.
- **Full Abstraction**: Use traits to separate hardware logic from data storage (persistence). This ensures the system can be tested and extended without hardware dependencies.
- **Test-Driven Development (TDD)**: Comprehensive tests for backends and the game runner must be maintained and updated with every change.
- **Code in English**: All code (variables, functions, classes), comments, and technical documentation must be strictly in English for international standardization.
- **Internationalization (i18n)**: The application must provide dynamic support for **English (EN)**, **Spanish (ES)** and **Portuguese (PT-BR)**.
- **Structured Logging**: Use structured logging (via `tracing` or similar) for all operations to facilitate debugging and monitoring.

---

## 🎨 Visual Identity & UI Standards

The FFBeast Linux client aims for a focused, high-clarity desktop hardware tool.

### Core Visuals
- **Primary Theme**: Functional desktop UI optimized for configurators and telemetry.
- **Color Palette**: High-contrast industrial UI colors are preferred over decorative theming.
- **Aesthetics**: Clear controls, stable layouts, readable numeric state, and low-friction maintenance workflows.

### User Experience (UX)
- **Performance**: High-reactivity UI for hardware monitoring and settings updates.
- **Animations**: Minimal. Prefer responsiveness and clarity over ornamental motion.
- **Responsive Layout**: The interface must adapt to typical Linux desktop window sizes without sacrificing readability.

---

## 🛠️ Technical Stack

- **Backend**: Rust (Performance, Safety, Concurrency).
- **Backend Service**: Rust stdio JSON process (`apps/ffbeast-service`).
- **Frontend**: Qt client via Python/PySide6 (`apps/ffbeast-qt`).
- **Profiles**: Local file-based persistence for now.

---

## 📁 Directory Structure

- `apps/ffbeast-service`: Backend process.
- `apps/ffbeast-qt`: Qt desktop client.
- `libs/controller`: Core hardware abstraction and communication logic.
- `libs/backend_api`: Shared frontend/backend protocol contract.
- `docs`: Technical specifications and project documentation.

---
*Last Updated: 2026-01-18*
