import { readFileSync, writeFileSync } from "fs";
import { dirname } from "path";
import { fileURLToPath } from "url";
import Delatin from "delatin";

const __dirname = dirname(fileURLToPath(import.meta.url));

function generateDelatinOutput(infile, outfile) {
  const heights = new Float64Array(
    JSON.parse(readFileSync(`${__dirname}/${infile}`))
  );

  const tin = new Delatin(heights, 512, 512);
  tin.run(0.2);

  const out = {
    coords: tin.coords,
    triangles: tin.triangles,
  };
  writeFileSync(`${__dirname}/${outfile}`, JSON.stringify(out));
}

function main() {
  generateDelatinOutput("delatin_rs.json", "delatin_rs_tin.json");
}

main();
