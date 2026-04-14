import { tuple,string } from "../../../package/index.ts";

Deno.test("tuple()", () => {
    const tupleArr = tuple([string()])
})