import("../pkg/fractal_wasm")
  .then(fractal => {
    document.fractal = fractal;
    let canvas = document.querySelector("#fractal-canvas");

    // Show coordinates within the canvas
    canvas.addEventListener("pointermove", event => {
      document.querySelector("#coords").innerText =
        "Canvas coords: X: " + event.clientX + ", Y: " + event.clientY;

      let othercoords = fractal.screen_to_turtle(
        canvas,
        event.clientX,
        event.clientY
      );
      document.querySelector("#turtle-coords").innerText =
        "Turtle coords: X: " + othercoords[0] + ", Y: " + othercoords[1];
    });

    fractal.render_dragon(canvas, 4);
  })
  .catch(console.error);
