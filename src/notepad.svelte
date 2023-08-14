<script type="text/typescript" lang="ts">
    const CHECK_INTERVAL = 5000
    const UPDATE_INTERVAL = 1000

    let loaded = false
    let lastUpdated = 0
    let lastUpdatedHuman = ""
    let interval: ReturnType<typeof setInterval> | undefined
    let timeout: ReturnType<typeof setTimeout> | undefined
    let value = 'Loading....'

    interface NoteResponse {
        body: string
        updated: number
    }

    interface UpdatedResponse {
        updated: number
    }

    function humanUnixTime(unix: number): string {
        return (new Date(unix * 1000)).toLocaleString()
    }

    function update({ body, updated }: NoteResponse) {
        value = body
        lastUpdated = updated
        lastUpdatedHuman = humanUnixTime(updated)
    }

    function loadNote() {
        return fetch('/notepad').then(res => res.json().then(update))
    }

    $: if (!loaded) {
        loadNote().then(() => {
            loaded = true
        })
    }

    $: if (loaded) {
        if (interval) clearInterval(interval)
        interval = setInterval(() => {
            fetch('/updated').then(res => {
                res.json().then(({ updated }: UpdatedResponse) => {
                    if (updated > lastUpdated) loadNote()
                })
            })
        }, CHECK_INTERVAL)
    }

    function onChange(): void {
        if (timeout) clearTimeout(timeout)
        timeout = setTimeout(() => {
            fetch('/notepad', {
                method: 'POST',
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ body: value })
            }).then(res => {
                res.json().then(update)
            })
        }, UPDATE_INTERVAL)
    }
</script>

<textarea bind:value on:change="{onChange}" />
<div id="lastupdated">{lastUpdatedHuman}</div>
