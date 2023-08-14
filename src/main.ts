import Notepad from './notepad.svelte';

document.addEventListener("DOMContentLoaded", function(event) {
    new Notepad({
        target: document.getElementById('notepad'), // entry point in ../public/index.html
    });
});
