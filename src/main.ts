import Notepad from "./components/notepad.svelte"

declare global {
  interface Window {
    instanceId: string
  }
}

document.addEventListener("DOMContentLoaded", function (event) {
  new Notepad({
    target: document.getElementById("notepad"), // entry point in ../public/index.html
  })
})
