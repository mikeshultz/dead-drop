<script type="text/typescript" lang="ts">
    const NAME = document.location.toString().split('/').pop()
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
        return fetch(`/${NAME}`, { headers: new Headers({ Accept: 'application/json' }) }).then(res => res.json().then(update))
    }

    $: if (!loaded) {
        const ssrBody = document.getElementById('ssrbody')?.getAttribute('value')
        if (ssrBody) {
            const ssrUpdated = document.getElementById('ssrupdated')?.getAttribute('value')
            update({ body: ssrBody, updated: ssrUpdated ? parseInt(ssrUpdated) : 0 })
            loaded = true
        } else {
            loadNote().then(() => {
                loaded = true
            })
        }
    }

    $: if (loaded) {
        if (interval) clearInterval(interval)
        interval = setInterval(() => {
            fetch(`/${NAME}/updated`).then(res => {
                res.json().then(({ updated }: UpdatedResponse) => {
                    if (updated > lastUpdated) loadNote()
                })
            })
        }, CHECK_INTERVAL)
    }

    function onChange(): void {
        if (timeout) clearTimeout(timeout)
        timeout = setTimeout(() => {
            fetch(`/${NAME}`, {
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
<div id="lastupdated">Updated: {lastUpdatedHuman}</div>
