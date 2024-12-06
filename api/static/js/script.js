const size = 20;
const sleepTime = 400;

const serverAddress = "/api";
const checkRoute = `${serverAddress}/check`;
const generateRoute = `${serverAddress}/generate-random`;
const getPresetsRoute = `${serverAddress}/get-presets`;
const healthCheckRoute = `${serverAddress}/healthz`;

const boardEl = document.getElementById("board");
const presetsEl = document.getElementById("presets");
const startStopBtnEl = document.getElementById("start-stop-button");
const speedEl = document.getElementById("speed");
const useToroidalEl = document.getElementById("use-toroidal");
const stepBtnEl = document.getElementById("step-button");
const clearBtnEl = document.getElementById("clear-button");
const iterationNumberEl = document.getElementById("iteration-number");
const loadingNotesEl = document.getElementById("additional-notes");

var matrix = [];
var lastMatrix = [];
var presets = [];
var isRunning = false;
var speed = 400;
let intervalId;
let useToroidal = false;
let iteration = 0;
var serverPingWasSuccessful = false;

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

speedEl.addEventListener("change", (e) => {
    console.debug("update speed changed");
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

function draw() {
    boardEl.innerHTML = "";

    for (var i = 0; i < matrix.length; i++) {
        var row = document.createElement("div");
        row.classList.add("row");

        for (var j = 0; j < matrix[i].length; j++) {
            var el = document.createElement("div");
            el.classList.add("cell");

            if (matrix[i][j] == 1) el.classList.add("alive");

            row.appendChild(el);
        }

        boardEl.appendChild(row);
    }
}

function healthCheck() {
    fetch(healthCheckRoute)
        .then(() => {
            serverPingWasSuccessful = true;
        })
        .catch((e) => {
            console.error(e);
            serverPingWasSuccessful = false;
        });

    return serverPingWasSuccessful;
}

function configure() {
    var route = `${generateRoute}?width=${size}&height=${size}`;

    fetch(route)
        .then((x) => x.json())
        .then((y) => {
            matrix = y;
            draw();
        })
        .catch((e) => console.error(e));

    fetch(getPresetsRoute)
        .then((x) => x.json())
        .then((y) => {
            presets = y;

            for (var i = 0; i < presets.length; i++) {
                var el = document.createElement("option");

                el.value = presets[i].id;
                el.innerText = presets[i].displayName;

                presetsEl.appendChild(el);
            }

            presetsEl.addEventListener("change", () => {
                if (presetsEl.value == "random") {
                    fetch(route)
                        .then((x) => x.json())
                        .then((y) => {
                            matrix = y;
                            draw();
                        })
                        .catch((e) => console.error(e));

                    return;
                }

                var target = presets.find((x) => x.id == presetsEl.value).matrix;

                const startX = Math.floor((matrix.length - target.length) / 2);
                const startY = Math.floor(
                    (matrix[0].length - target[0].length) / 2
                );

                clearMatrix();

                for (let i = 0; i < target.length; i++)
                    for (let j = 0; j < target[i].length; j++)
                        matrix[startX + i][startY + j] = target[i][j];

                draw();
            });
        })
        .catch((e) => console.error(e));
}

function clearMatrix() {
    for (var i = 0; i < matrix.length; i++)
        for (var j = 0; j < matrix[i].length; j++) matrix[i][j] = 0;

    lastMatrix = [];
    iteration = 0;
    isRunning = false;
}

function increaseIteration() {
    iteration++;

    iterationNumberEl.innerText = iteration;
}

function update() {
    var route = `${checkRoute}?use_toroidal=${useToroidal}`;

    fetch(route, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(matrix),
    })
        .then((x) => x.json())
        .then((y) => {
            matrix = y;

            draw();
            increaseIteration();

            lastMatrix = matrix;
        })
        .catch((e) => console.error(e));
}

function startLoop() {
    intervalId = setInterval(() => {
        if (isRunning) update();
    }, speed);
}

configure();
startLoop();