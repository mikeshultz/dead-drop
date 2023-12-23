import crypto from "./crypto"

test("encrypts and decrypts message with password", async () => {
  const message = "This is super secret information."
  const password = "secretPassword"
  const enc = await crypto.encrypt(message, password)

  expect(enc).toBeTruthy()
  expect(enc).not.toBe(message)
  expect(enc).toHaveProperty("salt")
  expect(enc).toHaveProperty("iv")
  expect(enc).toHaveProperty("cipherText")

  const dec = await crypto.decrypt(enc, password)

  expect(dec).toBe(message)
})
