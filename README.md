# Sort Visualization

Kleines Rust-Tool, das Sortieralgorithmen in einer Terminal-UI animiert.

## Starten

Mit Einstellungsmenue starten:

```bash
cargo run -q
```

Im Menue kannst du Algorithmus, Datenmodus, Groesse und Geschwindigkeit einstellen und danach die Visualisierung starten.

Direkt mit Algorithmus:

```bash
cargo run -q -- bubble
cargo run -q -- selection
cargo run -q -- insertion
cargo run -q -- quick
```

## Optionen

```bash
cargo run -q -- quick --size 30 --delay 40 --mode random
```

- `--size`, `-s`: Anzahl der Werte, 5 bis 60
- `--delay`, `-d`: Pause pro Animationsschritt in Millisekunden, 0 bis 2000
- `--mode`, `-m`: `random`, `reversed` oder `nearly`

## Beispiele

```bash
cargo run -q -- bubble -s 20 -d 80
cargo run -q -- quick --mode reversed --delay 30
cargo run -q -- insertion --mode nearly --size 35
```

## Tasten

- `↑` / `↓`: Menuepunkt auswaehlen
- `←` / `→`: Einstellung aendern
- `Enter`: Visualisierung starten
- `q` oder `Esc`: in der Visualisierung zurueck ins Menue
- `q` oder `Esc`: im Menue beenden
- `Space`: pausieren/fortsetzen
- `r`: neu starten
- `←` / `→`: Algorithmus wechseln
- `1` bis `4`: Algorithmus direkt waehlen
- `m`: Datenmodus wechseln
- `+` / `-`: Anzahl der Werte aendern
- `[` / `]`: Animation schneller/langsamer machen
