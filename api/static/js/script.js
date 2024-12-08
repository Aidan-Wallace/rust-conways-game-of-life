const size = 20;
const sleepTime = 400;

const boardEl = document.getElementById("board");
const presetsEl = document.getElementById("presets");
const startStopBtnEl = document.getElementById("start-stop-button");
const speedEl = document.getElementById("speed");
const useToroidalEl = document.getElementById("use-toroidal");
const stepBtnEl = document.getElementById("step-button");
const clearBtnEl = document.getElementById("clear-button");
const iterationNumberEl = document.getElementById("iteration-number");
const loadingNotesEl = document.getElementById("additional-notes");

let matrix = [];
let lastMatrix = [];
let presets = [];
let isRunning = false;
let speed = 400;
let intervalId;
let useToroidal = false;
let iteration = 0;
let serverPingWasSuccessful = false;

window.addEventListener("load", () => {
    const loadingScreen = document.getElementById("loadingScreen");
    function checkServer() {
        if (healthCheck() === true) {
            loadingScreen.style.display = "none";
        } else {
            loadingNotesEl.innerText = "Cannot connect to server";
            setTimeout(checkServer, 100);
        }
    }

    checkServer();
});

startStopBtnEl.addEventListener("click", (e) => {
    console.debug("start stop button clicked");
    e.preventDefault();
    isRunning = !isRunning;
});

speedEl.addEventListener("input", (e) => {
    console.debug(`update speed changed to ${e.target.value}`);
    clearInterval(intervalId);
    speed = e.target.value;
    startLoop();
});

useToroidalEl.addEventListener("change", (e) => {
    console.debug("use-toroidal button clicked");
    useToroidal = e.target.checked;
});

stepBtnEl.addEventListener("click", (e) => {
    console.debug("step button clicked");
    e.preventDefault();
    update();
});

clearBtnEl.addEventListener("click", (e) => {
    console.debug("clear button clicked");
    e.preventDefault();
    clearMatrix();
    draw();
});

presetsEl.addEventListener("change", () => {
    if (presetsEl.value == "random") {
        ApiServices.generateRandom()
            .then((y) => {
                matrix = y;
                draw();
            })
            .catch((e) => console.error(e));

        return;
    }

    let target = presets.find((x) => x.id == presetsEl.value).matrix;

    const startX = Math.floor((matrix.length - target.length) / 2);
    const startY = Math.floor((matrix[0].length - target[0].length) / 2);

    clearMatrix();

    for (let i = 0; i < target.length; i++)
        for (let j = 0; j < target[i].length; j++)
            matrix[startX + i][startY + j] = target[i][j];

    draw();
});

function update() {
    ApiServices.check(matrix, useToroidal)
        .then((y) => {
            lastMatrix = [...matrix];
            matrix = y;

            draw();
            increaseIteration();


            if (matrixesAreSame(matrix, lastMatrix)) {
                isRunning = false;
            }
        })
        .catch((e) => console.error(e));
}

function configure() {
    ApiServices.getPresets()
        .then((y) => {
            presets = y;

            for (let i = 0; i < presets.length; i++) {
                let el = document.createElement("option");

                el.value = presets[i].id;
                el.innerText = presets[i].displayName;

                presetsEl.appendChild(el);
            }
        })
        .catch((e) => console.error(e));

    ApiServices.generateRandom()
        .then((y) => {
            matrix = y;
            draw();
        })
        .catch((e) => console.error(e));
}

function draw() {
    boardEl.innerHTML = "";
    matrix.forEach((row, i) => {
        const rowEl = createRow(row, i);
        boardEl.appendChild(rowEl);
    });
}

function createRow(row, rowIndex) {
    const rowEl = document.createElement("div");
    rowEl.classList.add("row");

    row.forEach((cell, j) => {
        const cellEl = createCell(cell, rowIndex, j);
        rowEl.appendChild(cellEl);
    });

    return rowEl;
}

function createCell(cell, rowIndex, colIndex) {
    const cellEl = document.createElement("div");
    cellEl.classList.add("cell");

    if (cell === 1) cellEl.classList.add("alive");

    cellEl.addEventListener('click', () => toggleCell(cellEl, rowIndex, colIndex));
    return cellEl;
}

function toggleCell(cellEl, rowIndex, colIndex) {
    matrix[rowIndex][colIndex] = matrix[rowIndex][colIndex] === 1 ? 0 : 1;

    if (matrix[rowIndex][colIndex] === 1) {
        cellEl.classList.add("alive");
    } else {
        cellEl.classList.remove("alive");
    }
}

function startLoop() {
    intervalId = setInterval(() => {
        if (isRunning) update();
    }, speed);
}

function matrixesAreSame(m1, m2) {
    if (m1.length !== m2.length)
        return false;

    for (let i = 0; i < m1.length; i++) {
        if (m1[i].length !== m2[i].length)
            return false;

        for (let j = 0; j < m1[i].length; j++)
            if (m1[i][j] !== m2[i][j])
                return false;
    }

    return true;
}

function clearMatrix() {
    matrix.forEach((row, i) => row.forEach((_, j) => matrix[i][j] = 0));

    lastMatrix = [];
    iteration = 0;
    isRunning = false;
}

function increaseIteration() {
    iteration++;
    iterationNumberEl.innerText = iteration;
}

function healthCheck() {
    ApiServices.healthCheck()
        .then(() => {
            serverPingWasSuccessful = true;
        })
        .catch((e) => {
            console.error(e);
            serverPingWasSuccessful = false;
        });

    return serverPingWasSuccessful;
}

class ApiServices {
    static serverAddress = "/api";
    static postHeaders = {
        "Content-Type": "application/json",
    };

    static check = async (matrix, useToroidal = false) => {
        const route = `${this.serverAddress}/check?use_toroidal=${useToroidal}`;
        try {
            const response = await fetch(route, {
                method: "POST",
                headers: this.postHeaders,
                body: JSON.stringify(matrix),
            });
            return await response.json();
        } catch (error) {
            console.error("Error in check:", error);
            throw error;
        }
    };

    static generateRandom = async (w = 20, h = 20) => {
        const route = `${this.serverAddress}/generate-random?width=${w}&height=${h}`;
        try {
            const response = await fetch(route);
            return await response.json();
        } catch (error) {
            console.error("Error in generateRandom:", error);
            throw error;
        }
    };

    static getPresets = async () => {
        const route = `${this.serverAddress}/get-presets`;
        try {
            const response = await fetch(route);
            return await response.json();
        } catch (error) {
            console.error("Error in getPresets:", error);
            throw error;
        }
    };

    static healthCheck = async () => {
        const route = `${this.serverAddress}/healthz`;
        try {
            const response = await fetch(route);
            return response.text();
        } catch (error) {
            console.error("Error in healthCheck:", error);
            throw error;
        }
    };
}

configure();
startLoop();
