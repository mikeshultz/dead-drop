import crypto from "crypto"

// Mock up the Web subtle crypto API
Object.defineProperty(globalThis, "crypto", {
  value: {
    subtle: crypto.webcrypto.subtle,
    getRandomValues: crypto.randomFillSync,
  },
})
