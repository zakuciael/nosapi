const fs = require("fs");
const { decode } = require("./dist/index");

const filename = process.argv[2];
if (filename === undefined)
  throw new Error("No filename provided. Usage: pnpm run cmd:decodeblackbox <filename>");

const blackbox = fs.readFileSync(filename, { encoding: "utf8", flag: "r" });
const fingerprint = decode(blackbox);
console.log(JSON.stringify(fingerprint, null, "\t"));
