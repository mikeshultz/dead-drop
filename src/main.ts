import Notepad from "./components/notepad.svelte"

declare global {
  interface Window {
    instanceId: string
  }
}

document.addEventListener("DOMContentLoaded", function (event) {
  // Ensure TLS connection if JS is enabled (required for client-side encryption)
  if (window.location.protocol !== "https:" && window.location.hostname !== "localhost") {
    window.location.protocol = "https:"
  }

  new Notepad({
    target: document.getElementById("notepad"), // entry point in ../public/index.html
  })
})
