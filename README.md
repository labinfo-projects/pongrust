# Rust Pong 🦀🏓

Un'implementazione didattica in **Rust** del classico gioco arcade **Pong**, sviluppata con la libreria grafica [Macroquad](https://macroquad.rs/).

Questo progetto nasce con l'intento di esplorare i concetti fondamentali dello sviluppo di giochi 2D in Rust, la gestione della fisica dei vettori, la riproduzione audio asincrona e il comportamento delle IA per giochi arcade.

---

## 📸 Anteprima e Gameplay

Il gioco offre un'esperienza arcade classica arricchita da una fisica dinamica ("effetto valanga") e da diverse modalità di controllo, inclusa una simulazione **Computer vs Computer**.

---

## ✨ Caratteristiche Principali

* **3 Modalità di Controllo**:
  * **Mouse**: Movimento fluido della racchetta sinistra basato sul puntatore.
  * **Tastiera**: Controllo tradizionale tramite frecce direzionali ($\uparrow$ / $\downarrow$).
  * **Computer vs Computer**: Entrambe le racchette sono guidate dall'IA per testare l'equilibrio di gioco.
* **Sistema di Difficoltà a 3 Livelli**:
  * **Facile**, **Media**, **Difficile** (con parametri di velocità adattivi per bilanciare la sfida).
* **Fisica Dinamica & "Effetto Valanga"**:
  * Ad ogni impatto con le racchette, la velocità della pallina viene incrementata casualmente per creare scambi sempre più veloci e spettacolari.
  * Aggiunta di micro-variazioni angolari casuali sui rimbalzi a parete per evitare loop geometrici ripetitivi.
* **Audio Asincrono**: Gestione di musica di sottofondo ed effetti sonori per impatti e punteggi tramite `macroquad::audio`.
* **Interfaccia e Grafica**: Menu di selezione interattivi, conteggio dei punti e campo da gioco scalabile.

---

## 🛠️ Aspetti Tecnici e Didattici

Questo repository è ideale per chi vuole comprendere la struttura base di un gioco 2D scritto in Rust. Alcuni dei concetti chiave usati nel codice includono:

### 1. Game Loop Asincrono
Utilizzo della macro `#[macroquad::main]` e del ciclo `loop` asincrono con `next_frame().await` per il rendering continuo a frame-rate fluido.

### 2. Fisica 2D Vettoriale e Collisioni
* **Tracciamento della Pallina**: Gestione delle posizioni $P(x, y)$ e delle velocità $V(v_x, v_y)$ tramite il tipo `Vec2`.
* **Collisione Cerchio-Rettangolo (AABB)**: Implementata nella funzione `circle_rect_collision`, che calcola il punto più vicino sul rettangolo (racchetta) rispetto al centro del cerchio (pallina) per verificare l'impatto:
  $$d^2 = (x_{circle} - x_{closest})^2 + (y_{circle} - y_{closest})^2 \le r^2$$

### 3. Logica dell'Intelligenza Artificiale (AI)
Le racchette controllate dal computer utilizzano un sistema di tracciamento con **zona morta** (`dead_zone`):
* Evita le oscillazioni ad alta frequenza (*jittering*) quando la racchetta si trova vicina al bersaglio $Y$.
* La racchetta si muove verso la pallina solo se l'errore $|y_{target} - y_{paddle}|$ supera la soglia prefissata.

### 4. Gestione della Casualità (`rand` crate)
Per evitare dipendenze interne incoerenti tra la gestione casuale di Macroquad e il crate esterno `rand`, il progetto fa uso esplicito del crate std `::rand::Rng` per la generazione di numeri casuali float ad alta precisione nei rimbalzi:
* `rumore()`: Moltiplicatore casuale $[1.00, 1.25)$ per l'accelerazione orizzontale.
* `rumore_angolo()`: Fattore casuale $[0.95, 1.05)$ sull'angolo di rimbalzo verticale.

---

## 🚀 Come Eseguire il Progetto

### Prerequisiti
Assicurati di avere installato **Rust** e **Cargo**. In caso contrario, puoi installarli da [rustup.rs](https://rustup.rs/).

### Struttura della Cartella
Per riprodurre correttamente l'audio, assicurati che i file sonori siano presenti nella cartella `assets/`:
```text
.
├── Cargo.toml
├── assets/
│   ├── gameplay.ogg
│   ├── ping.wav
│   ├── pong.wav
│   └── vitapersa.wav
└── src/
    └── main.rs
```

### Compilazione ed Esecuzione
Clona il repository ed esegui il progetto tramite Cargo:

```bash
git clone https://github.com/labinfo-projects/pongrust.git
cd pongrust\pongrust
cargo build --release
```
Accertarsi di spostare l'eseguibile ottenuto, presente nella cartella target\release, nella cartella base che contiene la cartella assets
---

## 🎮 Comandi di Gioco

* **M**: Seleziona controllo con Mouse.
* **K**: Seleziona controllo da Tastiera (Frecce Up/Down).
* **A**: Attiva la modalità Auto (Computer vs Computer).
* **1 / 2 / 3**: Selezione Difficoltà (Facile, Media, Difficile).
* **R**: Reset del punteggio corrente.
* **Q / Esc**: Uscita dal gioco.

---

## 👤 Autore

**Giorgio Ruffa** — *(c) 2026*  
*Sviluppato con passione per l'apprendimento di Rust! 🦀*
