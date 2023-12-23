const { subtle } = globalThis.crypto

export type HexString = `0x${string}`

export interface EncryptedMessage {
  salt: HexString
  iv: HexString
  cipherText: HexString
}

/**
 * @dev Remove the 0x prefix from a hex string, if it exists
 * @param s hex string
 * @returns Hex string without the prefix
 */
function remove0x(s: string): string {
  return s.replace(/^0x/, "")
}

/**
 * @dev Convert a buffer to a hex string
 * @param buffer ArrayBuffer data
 * @returns The encoded hex string
 */
function buf2Hex(buffer: ArrayBuffer): HexString {
  const hex = [...new Uint8Array(buffer)]
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("")
  return `0x${hex}`
}

/**
 * @dev Convert a hex string to a bufer
 * @param hex The hex string to convert
 * @returns ArrayBuffer data
 */
function hex2Buf(hex: string): ArrayBuffer {
  return new Uint8Array(
    remove0x(hex)
      .match(/.{1,2}/g)
      .map((byte) => parseInt(byte, 16))
  ).buffer
}

/**
 * @dev Creates a key used in PBKDF2 from a password
 * @param password The password to use to generate the key
 * @returns CryptoKey for the password
 */
async function passwordKey(password: string): Promise<CryptoKey> {
  return await subtle.importKey(
    "raw",
    new TextEncoder().encode(password),
    {
      name: "PBKDF2",
    },
    false,
    ["deriveKey"]
  )
}

/**
 * @dev Derive an AES-CBC encryption key given a salt and password
 * @param salt The salt to use to generate the key
 * @param password The password to use to generate the key
 * @returns CryptoKey for data encryption
 */
async function deriveKey(
  salt: HexString,
  password: string
): Promise<CryptoKey> {
  const keyAlgo = {
    name: "PBKDF2",
    hash: "SHA-512",
    salt: hex2Buf(salt),
    iterations: 123456,
  }
  const pwKey = await passwordKey(password)
  return await subtle.deriveKey(
    keyAlgo,
    pwKey,
    {
      name: "AES-CBC",
      length: 256,
    },
    false,
    ["encrypt", "decrypt"]
  )
}

/**
 * @dev Encrypt message data using the given password
 * @param message The message to encrypt
 * @param password The password to use to generate the key
 * @returns EncryptedMessage object to store that can be used to reconstruct the message
 */
async function encrypt(
  message: string,
  password: string
): Promise<EncryptedMessage> {
  const salt = buf2Hex(globalThis.crypto.getRandomValues(new Uint8Array(16)))
  const key = await deriveKey(salt, password)
  const iv = globalThis.crypto.getRandomValues(new Uint8Array(16))
  const algo = {
    name: "AES-CBC",
    iv,
  }
  const cipherText = await subtle.encrypt(
    algo,
    key,
    new TextEncoder().encode(message)
  )

  return {
    salt: salt,
    iv: buf2Hex(iv),
    cipherText: buf2Hex(cipherText),
  }
}

/**
 * @dev Decrypts the given EncryptedMessage object using the provided message
 * @param enc The EncryptedMessage object to decrypt
 * @param password The password to use to generate the key
 * @returns The descrypted message data
 */
async function decrypt(
  enc: EncryptedMessage,
  password: string
): Promise<string> {
  const key = await deriveKey(enc.salt, password)
  const algo = {
    name: "AES-CBC",
    iv: hex2Buf(enc.iv),
  }
  const data = await subtle.decrypt(algo, key, hex2Buf(enc.cipherText))
  return new TextDecoder().decode(data)
}

export default {
  decrypt,
  encrypt,
}
