# Sort Visualization

Kleines Rust-Tool, das Sortieralgorithmen in einer Terminal-UI animiert.
Zusätzlich gibt es eine gemeinsame GUI-Variante fuer Desktop und Web.

## Starten

Terminal-App mit Einstellungsmenue starten:

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

## Desktop-App

```bash
cargo run -q --bin desktop
```

## Web-App

Die Web-Variante nutzt WebAssembly. Mit `trunk` kannst du sie lokal starten:

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
trunk serve index.html --open
```

Der Web-Einstiegspunkt ist [index.html](index.html), der Rust-Binary-Target ist `web`.

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
