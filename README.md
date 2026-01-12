# TimeEdit Tongs

TimeEdit Tongs är en webbapp för att skapa kalenderprenumerationslänkar efter kurser och studentgrupper på Linköpings universitet. Allt sker direkt i webbläsaren. Ingen server.

**Funktioner:**

- (Någorlunda) snabb och smart sökning med fuzzy matchning.
- Generera `.ics`-länkar som kan importeras i de flesta kalenderprogram.

**Teknologier:**

- Rust och Dioxus för frontend.
- `skim`\-baserad fuzzy matching för sökningar.
- Hårdkodat databas över kurser/studentgrupper som läses direkt i webbläsaren, vilket gör appen (någorlunda) snabb och offline-kompatibel.
