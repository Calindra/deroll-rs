import { is_real } from "wallet";
import { strict as assert } from "node:assert";

async function main() {
  assert.equal(add(2 + 2), 4);
}

main().catch(console.error);
