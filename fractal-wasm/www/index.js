const fractal_descriptions = [
  {
    id: "barnsleyfern",
    name: "Barnsley Fern Fractal",
    config: [],
    get_animation: (canvas, fractal_mod) => event => {
      return fractal_mod.animated_barnsleyfern(canvas);
    }
  },
  {
    id: "burningmandel",
    name: "Burning Mandel Fractal",
    config: [
      { name: "Max Iterations", id: "max-iterations", default: 100 },
      { name: "Power", id: "power", default: 2 }
    ],
    get_animation: (canvas, fractal_mod) => event => {
      let max_iterations = parseInt(
        document.querySelector("#burningmandel-max-iterations").value
      );
      let power = parseInt(
        document.querySelector("#burningmandel-power").value
      );
      return fractal_mod.animated_burningmandel(canvas, max_iterations, power);
    }
  },
  {
    id: "burningship",
    name: "Burning Ship Fractal",
    config: [
      { name: "Max Iterations", id: "max-iterations", default: 100 },
      { name: "Power", id: "power", default: 2 }
    ],
    get_animation: (canvas, fractal_mod) => event => {
      let max_iterations = parseInt(
        document.querySelector("#burningship-max-iterations").value
      );
      let power = parseInt(document.querySelector("#burningship-power").value);
      return fractal_mod.animated_burningship(canvas, max_iterations, power);
    }
  },
  {
    id: "cesaro",
    name: "Cesáro Fractal",
    config: [{ name: "Iterations", id: "iterations" }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = parseInt(
        document.querySelector("#cesaro-iterations").value
      );
      return fractal_mod.animated_cesaro(canvas, iterations);
    }
  },
  {
    id: "cesarotri",
    name: "Triangle Cesáro Fractal",
    config: [{ name: "Iterations", id: "iterations" }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = parseInt(
        document.querySelector("#cesarotri-iterations").value
      );
      return fractal_mod.animated_cesarotri(canvas, iterations);
    }
  },
  {
    id: "dragon",
    name: "Dragon Fractal",
    config: [{ name: "Iterations", id: "iterations" }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = parseInt(
        document.querySelector("#dragon-iterations").value
      );
      return fractal_mod.animated_dragon(canvas, iterations);
    }
  },
  {
    id: "kochcurve",
    name: "Koch Curve",
    config: [{ name: "Iterations", id: "iterations" }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = parseInt(
        document.querySelector("#kochcurve-iterations").value
      );
      return fractal_mod.animated_kochcurve(canvas, iterations);
    }
  },
  {
    id: "levyccurve",
    name: "Levy C Curve",
    config: [{ name: "Iterations", id: "iterations" }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = parseInt(
        document.querySelector("#levyccurve-iterations").value
      );
      return fractal_mod.animated_levyccurve(canvas, iterations);
    }
  },
  {
    id: "mandelbrot",
    name: "Mandelbrot Fractal",
    config: [
      { name: "Max Iterations", id: "max-iterations", default: 100 },
      { name: "Power", id: "power", default: 2 }
    ],
    get_animation: (canvas, fractal_mod) => event => {
      let max_iterations = parseInt(
        document.querySelector("#mandelbrot-max-iterations").value
      );
      let power = parseInt(document.querySelector("#mandelbrot-power").value);
      return fractal_mod.animated_mandelbrot(canvas, max_iterations, power);
    }
  },
  {
    id: "roadrunner",
    name: "Roadrunner Fractal (burningship variation)",
    config: [
      { name: "Max Iterations", id: "max-iterations", default: 100 },
      { name: "Power", id: "power", default: 2 }
    ],
    get_animation: (canvas, fractal_mod) => event => {
      let max_iterations = parseInt(
        document.querySelector("#roadrunner-max-iterations").value
      );
      let power = parseInt(document.querySelector("#roadrunner-power").value);
      return fractal_mod.animated_roadrunner(canvas, max_iterations, power);
    }
  },
  {
    id: "sierpinski",
    name: "Sierpinski Triangle",
    config: [],
    get_animation: (canvas, fractal_mod) => event => {
      return fractal_mod.animated_sierpinski(canvas);
    }
  },
  {
    id: "terdragon",
    name: "Terdragon Fractal",
    config: [{ name: "Iterations", id: "iterations" }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = parseInt(
        document.querySelector("#terdragon-iterations").value
      );
      return fractal_mod.animated_terdragon(canvas, iterations);
    }
  }
];

/**********************************************************
 * Config
 **********************************************************/

/**
 * Upddates which configuration element is shown --- assumes that the
 * configuration elements for each fractals have already been created.
 */
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

/**
 * Returns an event handler that updates the coordinates under the cursor,
 * using the canvas and the most recently/currently rendered fractal.
 */
const update_coords = canvas => event => {
  let x = event.clientX - canvas.offsetLeft;
  let y = event.clientY - canvas.offsetTop;

  document.querySelector("#coords").innerText =
    "Canvas coords: X: " + x + ", Y: " + y;

  if (window.current_animation) {
    let othercoords = window.current_animation.pixel_to_coordinate(x, y);
    document.querySelector("#fractal-coords").innerText =
      "Fractal coords: X: " + othercoords[0] + ", Y: " + othercoords[1];
  } else {
    document.querySelector("#fractal-coords").innerText =
      "Fractal coords: No fractal being rendered";
  }
};

/**
 * Builds the configuration UI for all of the fractals, and sets up the event
 * handlers.
 *
 * It fills out the entries/options in the #fractal-type dropdown, and
 * constructs the configuration options for each fractal.
 */
function setup_configs(canvas, fractal) {
  // Build the #fractal-type dropdown.
  let fractal_picker = document.querySelector("#fractal-type");
  for (desc of fractal_descriptions) {
    let option = document.createElement("option");
    option.value = desc.id;
    option.appendChild(document.createTextNode(desc.name));
    fractal_picker.appendChild(option);
  }

  // Whenever the selection changes, update which configs are visible.
  fractal_picker.addEventListener("input", event => {
    let choice = event.target.selectedOptions[0];
    let selected_fractal = choice.value;

    set_visible_config(selected_fractal);
  });

  let config_container = document.querySelector("#configs");
  for (cdesc of fractal_descriptions) {
    let desc = cdesc; // actually bind the desc to the scope >.<
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
      if (config_option.default) {
        config_input.value = config_option.default;
      }
      fractal_config.appendChild(config_input);
    }

    // Add a button to start/restart the animation
    let button = document.createElement("button");
    button.title = "Run";
    button.textContent = "Run";
    fractal_config.appendChild(button);

    // Listen for changes to start/restart the animation
    button.addEventListener("click", event => {
      // Stop any ongoing animation
      if (window.current_frame) {
        window.cancelAnimationFrame(window.current_frame);
      }

      // Fetch the new animation, and if we actually got one, run the animation.
      // If we don't have one, don't try to start it, allowing us to stop one
      // by not returning anything.
      let animation = desc.get_animation(canvas, fractal)(event);
      if (animation) {
        // Store a global reference to the current animation -- we need it to
        // calculate the current coordinates.
        window.current_animation = animation;

        let draw = ts => {
          if (animation.draw_one_frame()) {
            window.current_frame = window.requestAnimationFrame(draw);
          }
        };
        window.current_frame = window.requestAnimationFrame(draw);
      }
    });

    // Add it to the page
    config_container.appendChild(fractal_config);
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
    canvas.addEventListener("pointermove", update_coords(canvas));

    setup_configs(canvas, fractal);
  })
  .catch(console.error);
