<script>
    import { onMount } from "svelte";
    import TailwindCss from "../lib/TailwindCSS.svelte";

    let data = "Waiting...";
    onMount(async () => {
        const response = await fetch("http://127.0.0.1:8000/hello/world");
        data = await response.json();
        data = data["hi"];
    });
</script>

<TailwindCss />

<h1>{data}</h1>
<h1 class="font-sans hover:font-serif">{data}</h1>
