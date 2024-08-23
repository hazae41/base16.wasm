import { assert, test } from "@hazae41/phobos"
import { base16_decode_mixed, base16_encode_lower, initBundled, Memory } from "mods/index.js"

test("base16", async () => {
  await initBundled()

  const bytes = crypto.getRandomValues(new Uint8Array(256))

  const text = base16_encode_lower(new Memory(bytes))
  const text2 = Buffer.from(bytes).toString("hex")

  const bytes2 = base16_decode_mixed(text).bytes

  assert(text === text2)
  assert(Buffer.from(bytes2).equals(Buffer.from(bytes)))
})