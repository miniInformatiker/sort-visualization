function startApp() {
  const canvas = document.getElementById("app");
  if (!(canvas instanceof HTMLCanvasElement)) {
    throw new Error("Canvas #app not found");
  }

  const bindings = window.wasmBindings;
  if (!bindings?.WebHandle) {
    throw new Error("WASM bindings are not available");
  }

  const handle = new bindings.WebHandle();
  window.sortVisualization = handle;
  return handle.start(canvas);
}

function main() {
  if (window.wasmBindings) {
    startApp().catch(showError);
    return;
  }

  window.addEventListener(
    "TrunkApplicationStarted",
    () => {
      startApp().catch(showError);
    },
    { once: true },
  );
}

function showError(error) {
  console.error("Failed to start Sort Visualization:", error);
  const message = document.createElement("pre");
  message.textContent = `Failed to start Sort Visualization:\n${error}`;
  message.style.position = "fixed";
  message.style.inset = "24px";
  message.style.zIndex = "10";
  message.style.margin = "0";
  message.style.padding = "16px";
  message.style.border = "1px solid #e57373";
  message.style.borderRadius = "8px";
  message.style.background = "#1f1215";
  message.style.color = "#ffd6d6";
  message.style.whiteSpace = "pre-wrap";
  document.body.appendChild(message);
}

main();
