// Loads the WebAssembly build of this very page, renders it again in the browser, and
// compares the result with the copy the native build wrote at deploy time.
//
// Everything below is presentation. The page you are reading was already complete before
// this file ran — that is the point of the exercise.

import init, { renderProfile } from "./pkg/winged_demo.js";

const verdict = document.getElementById("parity-verdict");
const bytesOut = document.getElementById("stat-bytes");
const timeOut = document.getElementById("stat-time");
const wasmOut = document.getElementById("stat-wasm");
const toggle = document.getElementById("toggle-markup");
const markup = document.getElementById("markup");

const numbers = new Intl.NumberFormat("pt-BR");
const encoder = new TextEncoder();

function say(state, text) {
    verdict.dataset.state = state;
    verdict.textContent = text;
}

function kilobytes(bytes) {
    return `${numbers.format(Math.round(bytes / 102.4) / 10)} KB`;
}

/** Index of the first character that differs, or -1 when the strings are identical. */
function firstDifference(a, b) {
    const shared = Math.min(a.length, b.length);
    for (let i = 0; i < shared; i += 1) {
        if (a[i] !== b[i]) {
            return i;
        }
    }
    return a.length === b.length ? -1 : shared;
}

function wireToggle(source) {
    markup.textContent = source;
    toggle.addEventListener("click", () => {
        const opening = markup.hidden;
        markup.hidden = !opening;
        toggle.setAttribute("aria-expanded", String(opening));
        toggle.textContent = opening ? "Ocultar o markup" : "Ver o markup gerado";
    });
}

async function run() {
    // Fetched by hand rather than letting init() do it, so the exact byte count of the
    // module can be shown without downloading it twice.
    const wasmBytes = await fetch(new URL("./pkg/winged_demo_bg.wasm", import.meta.url))
        .then((response) => response.arrayBuffer());
    await init({ module_or_path: wasmBytes });
    wasmOut.textContent = kilobytes(wasmBytes.byteLength);

    const started = performance.now();
    const rendered = renderProfile();
    const elapsed = performance.now() - started;

    timeOut.textContent = `${numbers.format(Math.round(elapsed * 100) / 100)} ms`;
    bytesOut.textContent = `${numbers.format(encoder.encode(rendered).length)} bytes`;
    wireToggle(rendered);

    const native = await fetch(new URL("./parity/native.txt", import.meta.url))
        .then((response) => response.text());

    const difference = firstDifference(native, rendered);
    if (difference === -1) {
        say("ok", `nativo === wasm — ${numbers.format(rendered.length)} caracteres idênticos`);
    } else {
        say("bad", `os dois resultados divergem na posição ${numbers.format(difference)}`);
    }
}

run().catch((error) => {
    const hint = location.protocol === "file:"
        ? "Sirva a pasta dist/ por HTTP — o navegador não carrega WebAssembly de file://."
        : String(error);
    say("bad", `Não foi possível rodar o módulo WebAssembly. ${hint}`);
});
