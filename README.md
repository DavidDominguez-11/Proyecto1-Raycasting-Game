

# Proyecto 1: Raycasting Game

Juego 3D estilo raycasting construido en Rust usando `raylib`. El objetivo es explorar un laberinto, encontrar la llave y llegar a la salida antes de que el tiempo se agote. Incluye minimapa, sprites (llave y baterías animadas), efectos de linterna y audio de fondo y SFX.

## Características

- Raycasting 3D por columnas con texturizado de paredes.
- Sprites billboard para llave, meta y baterías animadas.
- Minimap persistente y mapa 2D alternativo.
- Efecto de linterna (gradiente radial) y oscurecimiento general.
- Tiempo de vida descendente (tipo “timer”).
- Sonidos: música de fondo y efectos (pasos y recolección).
- Soporte para múltiples niveles basados en archivos `maze*.txt`.

# Enlace al Video de YouTube

[Mira este video tutorial](https://youtu.be/OSI6EssYeH4) donde se muestra el funcionamiento con las siguientes características:

## Funcionalidades Implementadas

| Puntos | Característica |
|--------|----------------|
| 15 | Efecto linterna |
| 20 | Rotar con mouse |
| 10 | Minimapa esquina |
| 5 | Música de fondo |
| 10 | Agregar efecto sonido |
| 20 | Animar sprite |
| 5 | Pantalla de bienvenida |
| 10 | Seleccionar nivel |
| 10 | Pantalla de éxito |
| 5 | Control de vida |
| **110** | **TOTAL** |


## Estructura del proyecto

```
RayTracer/
├─ assets/
│  ├─ sounds/
│  │  ├─ battery_pickup.mp3
│  │  ├─ game_music.mp3
│  │  └─ step.mp3
│  └─ textures/
│     ├─ battery1.png
│     ├─ battery2.png
│     ├─ battery3.png
│     ├─ key.png
│     ├─ wall1.png
│     ├─ wall2.png
│     ├─ wall3.png
│     ├─ wall4.png
│     └─ wall5.png
├─ maze.txt
├─ maze1.txt
├─ maze2.txt
├─ maze3.txt
├─ src/
│  ├─ audio.rs             // Reproductor de audio (música/SFX) con rodio
│  ├─ caster.rs            // Ray casting y cálculo de impactos
│  ├─ framebuffer.rs       // Framebuffer basado en raylib::Image
│  ├─ key.rs               // Structs Key y Battery (sprites)
│  ├─ main.rs              // Bucle principal, estados, render y lógica de juego
│  ├─ maze.rs              // Carga de laberintos desde archivos de texto
│  ├─ player.rs            // Jugador, entrada y movimiento con colisiones
│  ├─ text.rs              // Fuente bitmap minimalista y dibujado de texto
│  └─ textures.rs          // Gestor de Texturas/Imágenes por carácter
├─ Cargo.toml
└─ Cargo.lock
```

## Instalación

1. Instala Rust:
   - https://www.rust-lang.org/tools/install

2. Clona o copia el proyecto.
   - git clone https://github.com/DavidDominguez-11/Proyecto1-Raycasting-Game.git

3. Verifica que [Cargo.toml](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/Cargo.toml:0:0-0:0) contiene:
   ```toml
   [package]
   name = "RayTracer"
   version = "0.1.0"
   edition = "2021"

   [profile.dev]
   opt-level = 3
   debug = false

   [dependencies]
   raylib = "5.5.1"
   rodio = "0.17.1"
   rand = "0.8"
   ```

## Ejecución

- Modo depuración (rápido para iterar):
  ```
  cd RayTracer\
  cargo run
  ```
## Controles

- Movimiento:
  - W / Flecha arriba: avanzar
  - S / Flecha abajo: retroceder
- Rotación:
  - Ratón
  - A/D o Flechas izquierda/derecha
- Linterna:
  - E: alternar linterna on/off
- Mapas:
  - M (mantenido): mostrar mapa 2D completo
  - Minimap: se muestra automáticamente en pantalla
- Menú:
  - Flechas arriba/abajo: seleccionar nivel
  - Enter: iniciar
  - Esc: volver al menú (desde el juego) o salir de pantallas de victoria/derrota
- Otros:
  - El cursor se oculta automáticamente al jugar y se muestra en menús o pantallas de fin.

## Mecánicas de juego

- Tienes un tiempo limitado para encontrar la llave y llegar a la salida (casilla ‘g’).
- El tiempo se muestra como barra y texto en la UI.
- Las baterías (sprites animados) otorgan tiempo extra al ser recogidas.
- La meta (casilla ‘g’) se dibuja como sprite y requiere tener la llave para ganar.

## Diseño de niveles

Los niveles están definidos por archivos de texto `maze*.txt`. Cada carácter representa una celda:

- ` ` (espacio): espacio libre (transitable)
- `#`, `+`, `-`, `|`: paredes (no transitables, diferentes texturas)
- `k`: celda con llave (se representa como sprite)
- `g`: meta/salida (renderizada como sprite, necesita llave)

Consejos:
- Asegúrate de que el laberinto esté cerrado por paredes en el perímetro para evitar rayos fuera de rango.
- Usa dimensiones regulares (rectangulares) y consistentes por fila.

## Texturas y mapeo de caracteres

En [src/textures.rs](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/src/textures.rs:0:0-0:0) se asignan imágenes a caracteres:

- Paredes: `|` → [wall1.png](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/assets/textures/wall1.png:0:0-0:0), `-` → [wall2.png](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/assets/textures/wall2.png:0:0-0:0), `+` → [wall4.png](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/assets/textures/wall4.png:0:0-0:0), `#` → [wall3.png](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/assets/textures/wall3.png:0:0-0:0)
- Meta: `g` → [wall5.png](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/assets/textures/wall5.png:0:0-0:0) (usada también para sprite de meta)
- Llave: `k` → [key.png](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/assets/textures/key.png:0:0-0:0)
- Baterías: `b/c/d` → `battery1/2/3.png` (animación por frames)

Puedes extender el `texture_files` para nuevos tipos de celdas/sprites.

## Audio

Música y efectos gestionados en [src/audio.rs](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/src/audio.rs:0:0-0:0) (rodio):

- Música de fondo en loop: `assets/sounds/game_music.mp3`
- SFX de pasos: `assets/sounds/step.mp3`
- SFX recolección de batería: `assets/sounds/battery_pickup.mp3`

Volumen y reproducción se controlan desde [main.rs](cci:7://file:///c:/dev/Proyecto1-Raycasting/Proyecto1-Raycasting-Game/RayTracer/src/main.rs:0:0-0:0). Asegúrate de que los archivos existan en las rutas indicadas.
