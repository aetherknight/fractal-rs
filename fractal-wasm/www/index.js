const fractal_descriptions = [
  {
    id: "cesaro",
    name: "Cesáro Fractal",
    config: [{ name: "Iterations", id: "iterations" }],
    run_config: (canvas, fractal) => event => {
      let iterations = parseInt(event.target.value);
      fractal.render_cesaro(canvas, iterations);
    }
  },
  {
    id: "cesarotri",
    name: "Triangle Cesáro Fractal",
    config: [{ name: "Iterations", id: "iterations" }],
    run_config: (canvas, fractal) => event => {
      let iterations = parseInt(event.target.value);
      fractal.render_cesarotri(canvas, iterations);
    }
  },
  {
    id: "dragon",
    name: "Dragon Fractal",
    config: [{ name: "Iterations", id: "iterations" }],
    run_config: (canvas, fractal) => event => {
      let iterations = parseInt(event.target.value);
      fractal.render_dragon(canvas, iterations);
    }
  },
  {
    id: "terdragon",
    name: "Terdragon Fractal",
    config: [{ name: "Iterations", id: "iterations" }],
    run_config: (canvas, fractal) => event => {
      let iterations = parseInt(event.target.value);
      fractal.render_terdragon(canvas, iterations);
    }
  }
];

/**********************************************************
 * Config
 **********************************************************/

function set_visible_config(selected_fractal) {
  console.log("Displaying config for " + selected_fractal);
  let config_panels = document.querySelectorAll(".config");
  for (panel of config_panels) {
    if (panel.id === selected_fractal + "-config") {
      panel.style.display = "block";
    } else {
      panel.style.display = "none";
    }
  }
}

function setup_configs(canvas, fractal) {
  let fractal_picker = document.querySelector("#fractal-type");
  for (desc of fractal_descriptions) {
    let option = document.createElement("option");
    option.value = desc.id;
    option.appendChild(document.createTextNode(desc.name));
    fractal_picker.appendChild(option);
  }
  fractal_picker.addEventListener("input", event => {
    let choice = event.target.selectedOptions[0];
    let selected_fractal = choice.value;

    set_visible_config(selected_fractal);
  });

  let config_container = document.querySelector("#configs");
  for (desc of fractal_descriptions) {
    // Build the config section for the fractal
    let fractal_config = document.createElement("div");
    fractal_config.className = "config";
    fractal_config.id = desc.id + "-config";
    for (config_option of desc.config) {
      // Add a label
      let config_label = document.createElement("label");
      config_label.htmlFor = desc.id + "-" + config_option.id;
      config_label.appendChild(document.createTextNode(config_option.name));
      fractal_config.appendChild(config_label);

      // Add an Input
      let config_input = document.createElement("input");
      config_input.id = desc.id + "-" + config_option.id;
      fractal_config.appendChild(config_input);
    }

    // Add it to the page
    config_container.appendChild(fractal_config);
    // Listen for changes
    fractal_config.addEventListener("input", desc.run_config(canvas, fractal));
  }
  set_visible_config(
    document.querySelector("#fractal-type").selectedOptions[0].value
  );
}

/**********************************************************
 * Load the wasm
 **********************************************************/
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

    setup_configs(canvas, fractal);
  })
  .catch(console.error);
