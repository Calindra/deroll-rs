import { writeFile } from "node:fs/promises";
import openapiTS from "openapi-typescript";

/*
This code customizes the TypeScript schema generation using openapi-typescript
Node API defined at https://openapi-ts.pages.dev/node/. The goal is to use the
viem types Hex and Address instead of simple strings for some schema properties.
*/

const inputFile =
  new URL("https://raw.githubusercontent.com/cartesi/openapi-interfaces/992c11489b50a47b5fb018298833f714ff166df7/rollup.yaml");
const outputFile = "src/schema.d.ts";
// import types from viem in generated code
const inject = "import { Address, Hex } from 'viem';\n";
console.log(`${inputFile.href} -> ${outputFile}`);
openapiTS(inputFile, {
  inject,
  transform: (schemaObject, _options) => {
    if ("format" in schemaObject && schemaObject.format === "hex") {
      // use viem.Hex if format is hex
      return schemaObject.nullable ? "Hex | null" : "Hex";
    } else if ("format" in schemaObject && schemaObject.format === "address") {
      // use viem.Address if format is address
      return schemaObject.nullable ? "Address | null" : "Address";
    }
  },
  // fs.writeFileSync(outputFile, output)
})
  .then((output) => writeFile(outputFile, output))
  .then(() => console.log("Done!"))
  .catch(console.error);
