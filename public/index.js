import init, * as module from "./sketchbook_ng.js";

main({
  "initial": {
    title: "Initial",
    createdAt: 1593328376159,
    text: "",
  }
});

function parseCommand(command) {
  const WORD = `(\\\S+)`;
  const STRING_LITERAL = `'((?:\\\\(?=')'|.)+?)'|"((?:\\\\(?=")"|.)+?)"`;
  const PARAMETER = `-(\\\\w+)(?:=(?:${STRING_LITERAL}))?`;
  const TERM = `${STRING_LITERAL}|${PARAMETER}|${WORD}`;
  const re = new RegExp(TERM);
  return re.exec(command);
}

async function main(sketches) {
  await init();

  const command = decodeURIComponent(location.hash.slice(1));
  const argv = parseCommand(command);
  const program = module[argv[0]];
  const sketch = sketches[argv[0]];

  document.title = `${sketch.title}.rs @ `;

  program(command);
}
