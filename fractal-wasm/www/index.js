import("../pkg/fractal_wasm")
    .then(wasm => {
        document.wasm = wasm;
        let canvas = document.querySelector("#fractal-canvas");

        canvas.addEventListener(
            "pointermove",
            (event) => {
                document.querySelector("#coords").innerText = "X: " + event.clientX + ", Y: " + event.clientY;

                let othercoords = wasm.screen_to_turtle(canvas, event.clientX, event.clientY);
                document.querySelector("#turtle-coords").innerText = "X: " + othercoords[0] + ", Y: " + othercoords[1];
            }
        );

    wasm.render(canvas);
  })
  .catch(console.error);

