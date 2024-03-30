import { verifyReceipt } from "../bindings/verifier";
import receipt from "../../../testing.receipt?url";

export function run(element: HTMLButtonElement) {
  async function verify() {
    const bin = new Uint8Array(await (await fetch(receipt)).arrayBuffer())
    console.log(bin);

    let res = verifyReceipt(bin);
    element.innerHTML = `Result = ${res}`;
  }
  element.addEventListener("click", () => verify());
}
