// Copyright (c) 2015-2019 William (B.J.) Snow Orvis
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

const fractal_descriptions = [
  {
    id: "barnsleyfern",
    name: "Barnsley Fern",
    category: "Chaos Games",
    config: [],
    get_animation: (canvas, fractal_mod) => event => {
      return fractal_mod.animated_barnsleyfern(canvas);
    }
  },
  {
    id: "burningmandel",
    name: "Burning Mandel",
    category: "Escape Time Fractals",
    config: [
      { name: "Max Iterations", id: "max-iterations", default: 100, min: 1 },
      { name: "Power", id: "power", default: 2, min: 1 }
    ],
    get_animation: (canvas, fractal_mod) => event => {
      let max_iterations = get_int("#burningmandel-max-iterations");
      let power = get_int("#burningmandel-power");
      return fractal_mod.animated_burningmandel(canvas, max_iterations, power);
    }
  },
  {
    id: "burningship",
    name: "Burning Ship",
    category: "Escape Time Fractals",
    config: [
      { name: "Max Iterations", id: "max-iterations", default: 100, min: 1 },
      { name: "Power", id: "power", default: 2, min: 1 }
    ],
    get_animation: (canvas, fractal_mod) => event => {
      let max_iterations = get_int("#burningship-max-iterations");
      let power = get_int("#burningship-power");
      return fractal_mod.animated_burningship(canvas, max_iterations, power);
    }
  },
  {
    id: "cesaro",
    name: "Cesáro Curve",
    category: "Lindenmayer Curves",
    config: [{ name: "Iterations", id: "iterations", min: 0 }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = get_int("#cesaro-iterations");
      return fractal_mod.animated_cesaro(canvas, iterations);
    }
  },
  {
    id: "cesarotri",
    name: "Triangle Cesáro Curve",
    category: "Lindenmayer Curves",
    config: [{ name: "Iterations", id: "iterations", min: 0 }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = get_int("#cesarotri-iterations");
      return fractal_mod.animated_cesarotri(canvas, iterations);
    }
  },
  {
    id: "dragon",
    name: "Dragon",
    category: "Other Curves",
    config: [{ name: "Iterations", id: "iterations", min: 0 }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = get_int("#dragon-iterations");
      return fractal_mod.animated_dragon(canvas, iterations);
    }
  },
  {
    id: "kochcurve",
    name: "Koch Curve",
    category: "Lindenmayer Curves",
    config: [{ name: "Iterations", id: "iterations", min: 0 }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = get_int("#kochcurve-iterations");
      return fractal_mod.animated_kochcurve(canvas, iterations);
    }
  },
  {
    id: "levyccurve",
    name: "Levy C Curve",
    category: "Lindenmayer Curves",
    config: [{ name: "Iterations", id: "iterations", min: 0 }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = get_int("#levyccurve-iterations");
      return fractal_mod.animated_levyccurve(canvas, iterations);
    }
  },
  {
    id: "mandelbrot",
    name: "Mandelbrot",
    category: "Escape Time Fractals",
    config: [
      { name: "Max Iterations", id: "max-iterations", default: 100, min: 1 },
      { name: "Power", id: "power", default: 2, min: 1 }
    ],
    get_animation: (canvas, fractal_mod) => event => {
      let max_iterations = get_int("#mandelbrot-max-iterations");
      let power = get_int("#mandelbrot-power");
      return fractal_mod.animated_mandelbrot(canvas, max_iterations, power);
    }
  },
  {
    id: "roadrunner",
    name: "Roadrunner",
    category: "Escape Time Fractals",
    config: [
      { name: "Max Iterations", id: "max-iterations", default: 100, min: 1 },
      { name: "Power", id: "power", default: 2, min: 1 }
    ],
    get_animation: (canvas, fractal_mod) => event => {
      let max_iterations = get_int("#roadrunner-max-iterations");
      let power = get_int("#roadrunner-power");
      return fractal_mod.animated_roadrunner(canvas, max_iterations, power);
    }
  },
  {
    id: "sierpinski",
    name: "Sierpinski Triangle",
    category: "Chaos Games",
    config: [],
    get_animation: (canvas, fractal_mod) => event => {
      return fractal_mod.animated_sierpinski(canvas);
    }
  },
  {
    id: "terdragon",
    name: "Terdragon",
    category: "Lindenmayer Curves",
    config: [{ name: "Iterations", id: "iterations", min: 0 }],
    get_animation: (canvas, fractal_mod) => event => {
      let iterations = get_int("#terdragon-iterations");
      return fractal_mod.animated_terdragon(canvas, iterations);
    }
  }
];

/**********************************************************
 * Config
 **********************************************************/

function get_int(selector) {
  const input = document.querySelector(selector);
  if (input.checkValidity()) {
    return parseInt(input.value);
  } else {
    throw `${selector} isn't valid`;
  }
}

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

  // Group the descriptions by category, to build a fancier dropdown with option groups
  const fractal_descriptions_by_category = fractal_descriptions.reduce(
    (acc, desc) => {
      if (acc.has(desc.category)) {
        acc.get(desc.category).push(desc);
      } else {
        acc.set(desc.category, [desc]);
      }
      return acc;
    },
    new Map()
  );

  // Build the dropdown
  for (category of [...fractal_descriptions_by_category.keys()].sort()) {
    console.log(category);
    let optgroup = document.createElement("optgroup");
    optgroup.label = category;

    for (desc of fractal_descriptions_by_category.get(category)) {
      let option = document.createElement("option");
      option.value = desc.id;
      option.appendChild(document.createTextNode(desc.name));
      optgroup.appendChild(option);
    }

    fractal_picker.appendChild(optgroup);
  }

  // Whenever the selection changes, update which configs are visible.
  fractal_picker.addEventListener("input", event => {
    let choice = event.target.selectedOptions[0];
    let selected_fractal = choice.value;

    set_visible_config(selected_fractal);
  });

  // Build the UI for each fractal
  let config_container = document.querySelector("#configs");
  for (cdesc of fractal_descriptions) {
    let desc = cdesc; // actually bind the desc to the scope >.<
    // Build the config section for the fractal
    let fractal_config = document.createElement("div");
    fractal_config.className = "config";
    fractal_config.id = desc.id + "-config";
    for (config_option of desc.config) {
      let config_div = document.createElement("div");
      // Add a label
      let config_label = document.createElement("label");
      config_label.htmlFor = desc.id + "-" + config_option.id;
      config_label.appendChild(document.createTextNode(config_option.name));
      config_div.appendChild(config_label);

      // Add an Input
      let config_input = document.createElement("input");
      config_input.id = desc.id + "-" + config_option.id;
      config_input.type = "number";
      config_input.required = true;
      if (config_option.default !== undefined) {
        config_input.value = config_option.default;
      }
      if (config_option.min !== undefined) {
        config_input.min = config_option.min;
      }
      if (config_option.max !== undefined) {
        config_input.max = config_option.max;
      }
      config_div.appendChild(config_input);
      fractal_config.appendChild(config_div);
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
