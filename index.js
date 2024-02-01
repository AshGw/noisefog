import { NoiseMap } from "noisefog";
import { lerp, invlerp } from "noisefog";

import * as dat from 'dat.gui';

const CELL_SIZE = 1;

var NoiseMapJS = function() {
  this.width = 500;
  this.height = 500;
  this.scale = 180;
  this.octaves = 6;
  this.lacunarity = 2;
  this.persistence = 0.5;
  this.grayscale = false;
  this.reshape = true;
};

const gui = new dat.GUI({ autoPlace: false });
document.getElementById("controls").appendChild(gui.domElement);
const tmp = new NoiseMapJS();
const widthControl = gui.add(tmp, 'width').min(0);
const heightControl = gui.add(tmp, 'height').min(0);
const scaleControl = gui.add(tmp, 'scale').min(0).step(1);
const octavesControl = gui.add(tmp, 'octaves', 1, 10, 1);
const lacunarityControl = gui.add(tmp, 'lacunarity').min(0).step(0.1);
const persistenceControl = gui.add(tmp, 'persistence').min(0).step(0.01);
const grayscaleControl = gui.add(tmp, 'grayscale');
const reshapeControl = gui.add(tmp, 'reshape');

var ElevationColor = function(color, height) {
  this.color = color;
  this.height = height;
};

var colors = [ new ElevationColor([0,79,163], 0.3)
             , new ElevationColor([66,135,245], 0.4)
             , new ElevationColor([34,201,101], 0.65)
             , new ElevationColor([105,55,6], 0.73)
             , new ElevationColor([74,37,1], 0.9)
             , new ElevationColor([255,255,255], 1.0)
             ];

const colorControllers = [];
const colorsGui = new dat.GUI({ autoPlace: false });
document.getElementById("colors").appendChild(colorsGui.domElement);
colors.forEach((elevationColor, idx) => {
  colorControllers.push(colorsGui.addColor(elevationColor, 'color'));
  colorControllers.push(colorsGui.add(elevationColor, 'height'));
});

const regenMap = () => {
  const map = NoiseMap.new(widthControl.getValue(),
    heightControl.getValue(),
    scaleControl.getValue(),
    octavesControl.getValue(),
    lacunarityControl.getValue(),
    persistenceControl.getValue(),
    reshapeControl.getValue(),
  );
  const width = map.width();
  const height = map.height();
  const maxNoiseVal = map.max_value();
  const minNoiseVal = map.min_value();
  const _memory = new WebAssembly.Memory({ initial: 256 }); 
  const noise = new Float64Array(_memory.buffer, map.noise_map(), width * height);

  const canvas = document.getElementById("noise-map-canvas");
  canvas.width = width*CELL_SIZE;
  canvas.height = height*CELL_SIZE;
  const ctx = canvas.getContext('2d');

  const getCoord = (idx) => {
    return [Math.floor(idx / width), idx % width];
  };

  const render = () => {
    noise.forEach((noiseValue, idx) => {
      const [y, x] = getCoord(idx);

      const amount = invlerp(minNoiseVal, maxNoiseVal, noiseValue);
      if (grayscaleControl.getValue() == true) {
        const grayScale = lerp(0, 255, amount);
        ctx.fillStyle = `rgb(${grayScale}, ${grayScale}, ${grayScale})`;
      } else {
        var cellColor = colors.slice().reverse()
          .reduce((accum, next) => amount <= next.height ? next : accum);
        var red = cellColor.color[0], green = cellColor.color[1], blue = cellColor.color[2];
        ctx.fillStyle = `rgb(${red},${green},${blue})`;
      }
      ctx.fillRect(x*CELL_SIZE, y*CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });
  };

  render();
}

regenMap();

widthControl.onFinishChange(regenMap);
heightControl.onFinishChange(regenMap);
scaleControl.onFinishChange(regenMap);
octavesControl.onFinishChange(regenMap);
lacunarityControl.onFinishChange(regenMap);
persistenceControl.onFinishChange(regenMap);
grayscaleControl.onFinishChange(regenMap);
reshapeControl.onFinishChange(regenMap);

colorControllers.forEach((controller, idx) => controller.onFinishChange(regenMap));