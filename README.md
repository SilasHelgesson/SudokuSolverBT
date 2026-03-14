# README
**English version below** 🇺🇸

## Deutsch 🇩🇪

### Projektbeschreibung
Dieses kleine Rust-Projekt war eine persönliche Übung, um mich mit der Sprache vertraut zu machen. Es implementiert einen Sudoku-Löser mithilfe eines Backtracking-Algorithmus. Das Programm geht das 9×9-Feld Zelle für Zelle durch, setzt gültige Ziffern ein und verwirft Entscheidungen, die zu einem Widerspruch führen, bis eine vollständige Lösung gefunden wird.

### Voraussetzungen
Stellen Sie sicher, dass Sie folgendes installiert haben:
- [Rust](https://www.rust-lang.org/) (stable, über `rustup` empfohlen)

### Installation und Ausführung
1. Klonen Sie das Repository auf Ihren lokalen Rechner.
```bash
   git clone https://github.com/SilasHelgesson/SudokuSolverBT
   cd SudokuSolverBT
```
2. Bauen und starten Sie das Programm mit Cargo.
```bash
   cargo run
```
Das zu lösende Sudoku-Feld ist direkt in `src/main.rs` in der `main`-Funktion als zweidimensionales Array definiert. Leere Felder werden durch `0` dargestellt.

---

## English 🇺🇸

### Project Description
This was a small personal project and a Rust exercise to get more familiar with the language. It implements a Sudoku solver using a backtracking algorithm. The program iterates over the 9×9 board cell by cell, placing valid digits and backtracking whenever a contradiction is reached, until a complete solution is found.

### Prerequisites
Make sure you have the following installed:
- [Rust](https://www.rust-lang.org/) (stable, via `rustup` recommended)

### Installation and Execution
1. Clone the repository to your local machine.
```bash
   git clone https://github.com/SilasHelgesson/SudokuSolverBT
   cd SudokuSolverBT
```
2. Build and run the program using Cargo.
```bash
   cargo run
```
The puzzle to be solved is defined directly in `src/main.rs` inside the `main` function as a 2D array. Empty cells are represented by `0`.

### License
This project is licensed under the GPL-3.0 License.
