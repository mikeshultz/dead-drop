import { mount } from "svelte"

import Notepad from "./components/notepad.svelte"

declare global {
  interface Window {
    instanceId: string
  }
}

document.addEventListener("DOMContentLoaded", function (event) {
  // Ensure TLS connection if JS is enabled (required for client-side encryption)
  if (
    window.location.protocol !== "https:" &&
    window.location.hostname !== "localhost"
  ) {
    window.location.protocol = "https:"
  }

  const target = document.getElementById("notepad")
  if (target) {
    // entry point in ../public/index.html
    mount(Notepad, { target })
  } else {
    throw new Error("Target element not found")
  }
})
