import { readFileSync, writeFileSync } from "fs";
import { dirname } from "path";
import { fileURLToPath } from "url";
import Delatin from "delatin";

const __dirname = dirname(fileURLToPath(import.meta.url));

function generateDelatinOutput(infile, coordsOutfile, trianglesOutfile) {
  const heights = new Float64Array(
    JSON.parse(readFileSync(`${__dirname}/${infile}`))
  );

  const tin = new Delatin(heights, 512, 512);
  tin.run(0.2);

  const coords = Uint32Array.from(tin.coords);
  const triangles = Uint32Array.from(tin.triangles);

  writeFileSync(`${__dirname}/${coordsOutfile}`, Buffer.from(coords.buffer));
  writeFileSync(
    `${__dirname}/${trianglesOutfile}`,
    Buffer.from(triangles.buffer)
  );
}

function main() {
  generateDelatinOutput(
    "delatin_rs.json",
    "coords_from_js.bin",
    "triangles_from_js.bin"
  );
}

main();
