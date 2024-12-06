# Conways Game of Life

Conways Game of Life in Rust

## API

The API can be run using:

```bash
cargo run --bin api
```

### Environment Variables

| Variable          | Default                      | Description                         |
| ----------------- | ---------------------------- | ----------------------------------- |
| `HOST`            | `127.0.0.1`                  | The server's host URL.              |
| `DEFAULT_PORT`    | `8080`                       | The server's host port.             |
| `HTML_FILE_FP`    | `./api/templates/index.html` | Path to the HTML index file.        |
| `STATIC_DIR_FP`   | `./api/static`               | Path to the static files directory. |
| `PRESETS_FILE_FP` | `./presets.json`             | Path to the presets file.           |

### Routes

#### `/`

**Method:** `GET`  
Returns an HTML page to play Conway's Game of Life.

#### `/api/check`

**Method:** `POST`  
Updates a Game of Life board to the next generation. The board is provided in the request body.

#### `/api/get-presets`

**Method:** `GET`  
Returns preset Game of Life boards.

**Available Presets:**

- Glider
- Pulsar
- Spaceship
- Circle of Fire
- Quadpole
- B29
- Fireship
- Spider

Additional presets can be added in the [`presets.json`](./presets.json) file.

#### `/api/generate-random`

**Method:** `GET`  
Generates a random Game of Life board.

**Query Parameters:**

- `width` (integer) - The width of the board.
- `height` (integer) - The height of the board.

#### `/api/healthz`

**Method:** `GET`  
Health check endpoint.

---

## Console App

The console application can be run using:

```bash
cargo run --bin console
```

### Environment Variables

| Variable           | Default | Description                                                                                                                  |
| ------------------ | ------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `SOURCE_GRID_PATH` | `null`  | Path to a source grid file. A sample grid is located at `sourceGrid.json`. If not provided, a random grid will be generated. |
| `USE_TOROIDAL`     | `null`  | Enables a toroidal effect if set to `true`.                                                                                  |
| `SLEEP_TIME_MS`    | `600`   | Time in milliseconds to sleep between generations.                                                                           |

---

## Rules

<ul>
    <li>
        <strong>Survival:</strong> A live cell with
        <strong>2 or 3 live neighbors</strong> stays alive to the next
        generation.
    </li>
    <li>
        <strong>Death by Underpopulation:</strong> A live cell with
        <strong>fewer than 2 live neighbors</strong> dies in the next
        generation.
    </li>
    <li>
        <strong>Death by Overpopulation:</strong> A live cell with
        <strong>more than 3 live neighbors</strong> dies in the next
        generation.
    </li>
    <li>
        <strong>Birth:</strong> A dead cell with
        <strong>exactly 3 live neighbors</strong> becomes a live cell in the
        next generation.
    </li>
</ul>

## Development

This project includes a Rust development container for Visual Studio Code. To use it, ensure you have the Remote - Containers extension installed.

To enable hot reloading, install `cargo-watch`

```sh
# install with
cargo install cargo-watch

# run with
cargo watch -x '--bin api'
```

# References

- [Unicode characters](https://en.wikipedia.org/wiki/Box-drawing_characters) for grids
- [More unicode](https://www.compart.com/en/unicode)
