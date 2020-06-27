import init, { greet } from "./sketchbook_ng.js";

async function main() {
  const module = await init();

  console.log(module);

  greet();
}

main();
