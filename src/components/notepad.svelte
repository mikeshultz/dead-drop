<script lang="ts">
  import { MD5 } from "jshashes"
  import { onDestroy, onMount } from "svelte"
  import crypto, { EncryptedMessage } from "../crypto"
  import Status from "./status"

  const md5 = new MD5()

  const NAME = document.location
    .toString()
    .split("/")
    .pop()
    .split("#")
    .slice(0, 1)
  const CHECK_INTERVAL = 3000
  const SAVE_COUNTDOWN_DURATION = 2000

  const instanceId = window.instanceId
  const hashKey = document.location.hash.slice(1)
  const passphrase = `${instanceId}${hashKey}`
  const shouldEncrypt = !!hashKey

  let updateTick: ReturnType<typeof setInterval> | undefined
  let countdownTick: ReturnType<typeof setInterval> | undefined
  let countdownStartTimeout: ReturnType<typeof setTimeout> | undefined

  let loaded = $state(false)
  let lastUpdated = $state(0)
  let lastUpdatedHuman = $state("")
  let percentUntilSave = $state(0)
  let value = $state("Loading....")
  let valueHash = $state("")

  console.log("instanceId:", instanceId)

  interface NoteResponse {
    body: string
    updated: number
  }

  interface UpdatedResponse {
    updated: number
  }

  /// Convert a unix timestamp to human-readable string
  function humanUnixTime(unix: number): string {
    return unix ? new Date(unix * 1000).toLocaleString() : ""
  }

  function looksLikeEncryptedMessage(body: string): boolean {
    return !!body && body.startsWith("{") && body.includes('"iv"')
  }

  function encryptMaybe(noteValue: string): Promise<string> {
    if (shouldEncrypt) {
      return crypto.encrypt(noteValue, passphrase).then(JSON.stringify)
    }

    return Promise.resolve(noteValue)
  }

  function decryptMaybe(body: string): Promise<string> {
    if (shouldEncrypt && looksLikeEncryptedMessage(body)) {
      return crypto.decrypt(JSON.parse(body), passphrase)
    }

    return Promise.resolve(body)
  }

  /// Update component note state from API response
  function update({ body, updated }: NoteResponse) {
    decryptMaybe(body)
      .then((dec) => {
        value = dec
      })
      .catch((err) => {
        console.error(err)
        value = "Error decrypting note"
      })
    valueHash = md5.hex(value)
    lastUpdated = updated
    lastUpdatedHuman = humanUnixTime(updated)
  }

  /// Load a note from the API
  function loadNote() {
    return fetch(`/${NAME}`, {
      headers: new Headers({ Accept: "application/json" }),
    }).then((res) => res.json().then(update))
  }

  /// Save the note to the API
  function saveNote() {
    encryptMaybe(value).then((body) => {
      fetch(`/${NAME}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ body }),
      }).then((res) => {
        percentUntilSave = 0
        res.json().then(update)
      })
    })
  }

  /// Check for note updates, and trigger load if BE has something newer
  function checkForUpdates(ival = CHECK_INTERVAL) {
    if (updateTick) clearInterval(updateTick)
    updateTick = setInterval(() => {
      fetch(`/${NAME}/updated`).then((res) => {
        res.json().then(({ updated }: UpdatedResponse) => {
          if (updated > lastUpdated) loadNote()
        })
      })
    }, ival)
  }

  /// Reset the save countdown
  function resetCountdownToSave() {
    if (countdownTick) clearInterval(countdownTick)
    percentUntilSave = 0
  }

  /// Save when the countdown complettes, reset countodnw if called again
  function countdownToSave() {
    resetCountdownToSave()

    countdownTick = setInterval(() => {
      percentUntilSave += 1
      if (percentUntilSave >= 100) {
        console.log("Saving note...")
        clearInterval(countdownTick)
        saveNote()
      }
    }, SAVE_COUNTDOWN_DURATION / 100)
  }

  function onKeyUp(): void {
    resetCountdownToSave()
    if (countdownStartTimeout) clearTimeout(countdownStartTimeout)

    countdownStartTimeout = setTimeout(() => {
      if (md5.hex(value) !== valueHash) {
        countdownToSave()
      }
    }, 500)
  }

  $effect(() => {
    if (!loaded) {
      const ssrBody = document.getElementById("ssrbody")?.getAttribute("value")
      if (ssrBody) {
        // Load note from server-side rendered hidden form
        const ssrUpdated = document
          .getElementById("ssrupdated")
          ?.getAttribute("value")
        update({
          body: ssrBody,
          updated: ssrUpdated ? parseInt(ssrUpdated) : 0,
        })
        loaded = true
      } else {
        // Load note from API
        loadNote().then(() => {
          loaded = true
        })
      }
    }
  })

  onDestroy(() => {
    if (countdownTick) clearInterval(countdownTick)
    if (updateTick) clearInterval(updateTick)
  })

  window.onblur = function () {
    // if nobody's watching, poll less frequently
    checkForUpdates(CHECK_INTERVAL * 10)
  }
  window.onfocus = function () {
    checkForUpdates()
  }
</script>

<textarea class="notepad" bind:value disabled={!loaded} onkeyup={onKeyUp}
></textarea>
<Status percent={percentUntilSave} {lastUpdated} />

<style type="text/css">
  textarea.notepad {
    border: 1px var(--bord) solid;
    width: 95vh; /* fallback */
    width: calc(95vw - 1.7rem);
    height: 90vh; /* fallback */
    height: calc(90vh - 3rem);
    margin: 0.5rem;
    margin-top: 1.5rem;
    padding: 0.75rem;
    font-size: 2rem;
  }

  /* vw > mobile */
  @media (min-width: 648px) {
    textarea.notepad {
      font-size: 1rem;
      width: calc(100vw - 4rem);
      height: calc(100vh - 6rem);
    }
  }
</style>
